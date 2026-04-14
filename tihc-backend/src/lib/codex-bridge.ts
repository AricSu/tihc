import { spawn, type ChildProcessWithoutNullStreams } from "node:child_process";
import { createInterface, type Interface as ReadLineInterface } from "node:readline";

export type CodexChatMessage = {
  role: string;
  content: string;
};

export type CodexModelEntry = {
  id: string;
  label: string;
};

export type CodexStatusResult = {
  available: boolean;
  loggedIn: boolean;
  needsLogin: boolean;
  message: string;
  account: {
    email?: string;
    planType?: string;
  } | null;
};

export type CodexLoginResult = {
  authUrl: string;
  loginId: string;
};

export interface CodexBridge {
  listModels(): Promise<CodexModelEntry[]>;
  readStatus(): Promise<CodexStatusResult>;
  startLogin(): Promise<CodexLoginResult>;
  streamChat(input: {
    model: string;
    messages: CodexChatMessage[];
  }): AsyncGenerator<string>;
}

type PendingRequest = {
  method: string;
  resolve: (value: Record<string, unknown>) => void;
  reject: (error: Error) => void;
  timeout: ReturnType<typeof setTimeout> | null;
};

type NotificationWaiter = {
  predicate: (message: Record<string, unknown>) => boolean;
  resolve: (message: Record<string, unknown>) => void;
  reject: (error: Error) => void;
  timeout: ReturnType<typeof setTimeout> | null;
};

function asString(value: unknown): string | null {
  return typeof value === "string" ? value : null;
}

function asRecord(value: unknown): Record<string, unknown> | null {
  if (!value || typeof value !== "object" || Array.isArray(value)) return null;
  return value as Record<string, unknown>;
}

function errorMessage(error: unknown): string {
  if (error instanceof Error && error.message.trim()) return error.message.trim();
  return String(error || "Unknown error");
}

function buildPrompt(messages: CodexChatMessage[]): string {
  return messages
    .filter((message) => message.content.trim())
    .map((message) => {
      const role = message.role === "assistant" ? "Assistant" : "User";
      return `${role}:\n${message.content.trim()}`;
    })
    .join("\n\n");
}

class CodexAppServerConnection {
  private readonly process: ChildProcessWithoutNullStreams;
  private readonly lines: ReadLineInterface;
  private readonly pending = new Map<number, PendingRequest>();
  private readonly bufferedNotifications: Record<string, unknown>[] = [];
  private readonly notificationWaiters: NotificationWaiter[] = [];
  private nextId = 1;
  private closed = false;

  private constructor(process: ChildProcessWithoutNullStreams) {
    this.process = process;
    this.lines = createInterface({ input: process.stdout });
    this.lines.on("line", (line) => this.handleLine(line));
    process.once("error", (error) => this.closeWithError(error instanceof Error ? error : new Error(String(error))));
    process.once("exit", (code, signal) => {
      this.closeWithError(
        new Error(
          `codex app-server exited: code=${String(code ?? "null")} signal=${String(signal ?? "null")}`,
        ),
      );
    });
  }

  static async connect(): Promise<CodexAppServerConnection> {
    let process: ChildProcessWithoutNullStreams;
    try {
      process = spawn("codex", ["app-server"], {
        stdio: ["pipe", "pipe", "pipe"],
      });
    } catch (error) {
      throw new Error(`Failed to start Codex CLI: ${errorMessage(error)}`);
    }

    const connection = new CodexAppServerConnection(process);
    try {
      await connection.request("initialize", {
        clientInfo: {
          name: "tihc-backend",
          title: "TIHC Backend",
          version: "0.1.0",
        },
        capabilities: {
          experimentalApi: true,
        },
      });
      connection.notify("initialized", {});
      return connection;
    } catch (error) {
      connection.close();
      throw error;
    }
  }

  close(): void {
    if (this.closed) return;
    this.closed = true;
    this.lines.close();
    try {
      this.process.stdin.end();
    } catch {
      // ignore
    }
    if (!this.process.killed) {
      this.process.kill();
    }
    this.rejectAll(new Error("codex app-server client is closed"));
  }

  async request(
    method: string,
    params: Record<string, unknown>,
    timeoutMs = 10_000,
  ): Promise<Record<string, unknown>> {
    if (this.closed) {
      throw new Error("codex app-server client is closed");
    }

    const id = this.nextId++;
    const payload = JSON.stringify({ id, method, params });
    return new Promise<Record<string, unknown>>((resolve, reject) => {
      const timeout = setTimeout(() => {
        this.pending.delete(id);
        reject(new Error(`${method} timed out`));
      }, timeoutMs);
      timeout.unref?.();
      this.pending.set(id, {
        method,
        resolve,
        reject,
        timeout,
      });
      this.process.stdin.write(`${payload}\n`);
    }).then((result) => {
      const errorPayload = asRecord(result.error);
      if (errorPayload) {
        throw new Error(asString(errorPayload.message) || `${method} failed`);
      }
      return asRecord(result.result) ?? {};
    });
  }

  notify(method: string, params: Record<string, unknown>): void {
    if (this.closed) return;
    this.process.stdin.write(`${JSON.stringify({ method, params })}\n`);
  }

  async waitForNotification(
    predicate: (message: Record<string, unknown>) => boolean,
    timeoutMs = 60_000,
  ): Promise<Record<string, unknown>> {
    const bufferedIndex = this.bufferedNotifications.findIndex(predicate);
    if (bufferedIndex >= 0) {
      const [message] = this.bufferedNotifications.splice(bufferedIndex, 1);
      return message!;
    }

    return new Promise<Record<string, unknown>>((resolve, reject) => {
      const timeout = setTimeout(() => {
        this.removeNotificationWaiter(waiter);
        reject(new Error("Timed out waiting for Codex notification."));
      }, timeoutMs);
      timeout.unref?.();
      const waiter: NotificationWaiter = {
        predicate,
        resolve,
        reject,
        timeout,
      };
      this.notificationWaiters.push(waiter);
    });
  }

  private handleLine(rawLine: string): void {
    const line = rawLine.trim();
    if (!line) return;

    let message: Record<string, unknown>;
    try {
      const parsed = JSON.parse(line) as unknown;
      if (!parsed || typeof parsed !== "object" || Array.isArray(parsed)) return;
      message = parsed as Record<string, unknown>;
    } catch {
      return;
    }

    const responseId = message.id;
    if (typeof responseId === "number") {
      const pending = this.pending.get(responseId);
      if (!pending) return;
      this.pending.delete(responseId);
      if (pending.timeout) {
        clearTimeout(pending.timeout);
      }
      pending.resolve(message);
      return;
    }

    const waiter = this.notificationWaiters.find((entry) => entry.predicate(message));
    if (waiter) {
      this.removeNotificationWaiter(waiter);
      waiter.resolve(message);
      return;
    }

    this.bufferedNotifications.push(message);
  }

  private removeNotificationWaiter(waiter: NotificationWaiter): void {
    const index = this.notificationWaiters.indexOf(waiter);
    if (index >= 0) {
      this.notificationWaiters.splice(index, 1);
    }
    if (waiter.timeout) {
      clearTimeout(waiter.timeout);
    }
  }

  private closeWithError(error: Error): void {
    if (this.closed) return;
    this.closed = true;
    this.lines.close();
    this.rejectAll(error);
  }

  private rejectAll(error: Error): void {
    for (const [id, pending] of this.pending.entries()) {
      this.pending.delete(id);
      if (pending.timeout) {
        clearTimeout(pending.timeout);
      }
      pending.reject(error);
    }
    while (this.notificationWaiters.length > 0) {
      const waiter = this.notificationWaiters.shift()!;
      if (waiter.timeout) {
        clearTimeout(waiter.timeout);
      }
      waiter.reject(error);
    }
  }
}

async function withConnection<T>(
  operation: (connection: CodexAppServerConnection) => Promise<T>,
): Promise<T> {
  const connection = await CodexAppServerConnection.connect();
  try {
    return await operation(connection);
  } finally {
    connection.close();
  }
}

function parseCodexModels(result: Record<string, unknown>): CodexModelEntry[] {
  const rawModels = Array.isArray(result.data) ? result.data : [];
  return rawModels
    .map((item) => asRecord(item))
    .filter((item): item is Record<string, unknown> => Boolean(item && item.hidden !== true))
    .map((item) => ({
      id: asString(item.id) || asString(item.model) || "",
      label: asString(item.displayName) || asString(item.id) || asString(item.model) || "",
    }))
    .filter((item) => item.id && item.label);
}

function parseAccount(result: Record<string, unknown>): CodexStatusResult {
  const account = asRecord(result.account);
  if (!account) {
    return {
      available: true,
      loggedIn: false,
      needsLogin: true,
      message: "Codex OAuth needs a fresh login.",
      account: null,
    };
  }

  return {
    available: true,
    loggedIn: true,
    needsLogin: false,
    message: "Codex OAuth is ready.",
    account: {
      ...(asString(account.email) ? { email: asString(account.email)! } : {}),
      ...(asString(account.planType) ? { planType: asString(account.planType)! } : {}),
    },
  };
}

export function createCodexBridge(): CodexBridge {
  return {
    async listModels() {
      const result = await withConnection((connection) =>
        connection.request("model/list", {}, 15_000),
      );
      return parseCodexModels(result);
    },

    async readStatus() {
      try {
        const result = await withConnection((connection) =>
          connection.request("account/read", { refreshToken: false }),
        );
        return parseAccount(result);
      } catch (error) {
        const message = errorMessage(error);
        const unavailable = /failed to start codex cli|spawn .*enoent|not installed/i.test(message);
        return {
          available: !unavailable,
          loggedIn: false,
          needsLogin: !unavailable,
          message,
          account: null,
        };
      }
    },

    async startLogin() {
      const result = await withConnection((connection) =>
        connection.request("account/login/start", { type: "chatgpt" }),
      );
      const authUrl = asString(result.authUrl) || "";
      const loginId = asString(result.loginId) || "";
      if (!authUrl || !loginId) {
        throw new Error("Codex login did not return an auth URL.");
      }
      return {
        authUrl,
        loginId,
      };
    },

    async *streamChat(input) {
      const connection = await CodexAppServerConnection.connect();
      try {
        const account = await connection.request("account/read", { refreshToken: false });
        if (!asRecord(account.account)) {
          throw new Error("Codex OAuth needs a fresh login.");
        }

        const thread = await connection.request(
          "thread/start",
          {
            model: input.model,
            summary: "concise",
          },
          15_000,
        );
        const threadId = asString(asRecord(thread.thread)?.id) || "";
        if (!threadId) {
          throw new Error("Codex did not return a thread id.");
        }

        const turn = await connection.request(
          "turn/start",
          {
            threadId,
            input: [
              {
                type: "text",
                text: buildPrompt(input.messages),
              },
            ],
            summary: "concise",
            sandboxPolicy: {
              type: "readOnly",
              access: {
                type: "fullAccess",
              },
            },
          },
          15_000,
        );
        const turnId = asString(asRecord(turn.turn)?.id) || "";
        if (!turnId) {
          throw new Error("Codex did not return a turn id.");
        }

        let finalText = "";
        let emittedLength = 0;
        while (true) {
          const notification = await connection.waitForNotification(() => true, 60_000);
          const method = asString(notification.method) || "";
          const params = asRecord(notification.params) ?? {};

          if (method === "item/agentMessage/delta") {
            const delta = asString(params.delta) || "";
            if (!delta) continue;
            finalText += delta;
            emittedLength += delta.length;
            yield delta;
            continue;
          }

          if (method === "item/completed") {
            const item = asRecord(params.item);
            if (asString(item?.type) === "agentMessage") {
              finalText = asString(item?.text) || finalText;
            }
            continue;
          }

          if (method !== "turn/completed") {
            continue;
          }

          const completedTurn = asRecord(params.turn);
          if (asString(completedTurn?.id) !== turnId) {
            continue;
          }

          if (asString(completedTurn?.status) !== "completed") {
            const error = asRecord(completedTurn?.error);
            throw new Error(asString(error?.message) || "Codex turn failed.");
          }

          if (finalText && emittedLength < finalText.length) {
            yield finalText.slice(emittedLength);
          }
          return;
        }
      } finally {
        connection.close();
      }
    },
  };
}
