import { buildMessageBody, extractErrorText } from "./http-common";
import { parseConfiguredJsonFrame } from "./parsers";
import type { AgentConnector, AgentEvent } from "./types";

export const genericWebSocketConnector: AgentConnector = {
  id: "generic-websocket",
  async *stream(agent, request) {
    if (!agent.endpoint.trim()) {
      yield { type: "error", message: "Missing agent endpoint." };
      return;
    }

    let socket: WebSocket | null = null;
    const queue: AgentEvent[] = [];
    let resolveNext: (() => void) | null = null;
    let closed = false;

    const push = (event: AgentEvent) => {
      queue.push(event);
      resolveNext?.();
      resolveNext = null;
    };

    try {
      socket = new WebSocket(agent.endpoint.trim());
      socket.addEventListener("open", () => {
        socket?.send(
          JSON.stringify({
            ...buildMessageBody(request.messages, agent),
            apiKey: agent.apiKey.trim() || undefined,
          }),
        );
      });
      socket.addEventListener("message", (event) => {
        const parsed = parseConfiguredJsonFrame(String(event.data ?? ""), {
          deltaPath: agent.deltaPath,
          snapshotPath: agent.snapshotPath,
          donePath: agent.donePath,
          doneSentinel: agent.doneSentinel,
        });
        if (parsed.textDelta) push({ type: "text-delta", text: parsed.textDelta });
        if (parsed.snapshot) push({ type: "replace-text", text: parsed.snapshot });
        if (parsed.done) {
          push({ type: "done" });
          socket?.close();
        }
      });
      socket.addEventListener("error", () => {
        push({ type: "error", message: "WebSocket connection error." });
      });
      socket.addEventListener("close", () => {
        closed = true;
        push({ type: "done" });
      });

      request.abortSignal.addEventListener(
        "abort",
        () => {
          socket?.close();
        },
        { once: true },
      );

      while (!closed || queue.length) {
        if (!queue.length) {
          await new Promise<void>((resolve) => {
            resolveNext = resolve;
          });
          continue;
        }
        const next = queue.shift();
        if (next) yield next;
      }
    } catch (error) {
      if (request.abortSignal.aborted) return;
      yield { type: "error", message: extractErrorText(error) };
    } finally {
      socket?.close();
    }
  },
  async testConnection(agent) {
    if (!agent.endpoint.trim()) {
      return { ok: false, message: "Missing agent endpoint." };
    }
    try {
      await new Promise<void>((resolve, reject) => {
        const socket = new WebSocket(agent.endpoint.trim());
        const timeout = setTimeout(() => {
          socket.close();
          reject(new Error("WebSocket connection timed out."));
        }, 5000);

        socket.addEventListener("open", () => {
          clearTimeout(timeout);
          socket.close();
          resolve();
        });
        socket.addEventListener("error", () => {
          clearTimeout(timeout);
          socket.close();
          reject(new Error("WebSocket connection failed."));
        });
      });
      return { ok: true, message: "WebSocket connected successfully." };
    } catch (error) {
      return { ok: false, message: extractErrorText(error) };
    }
  },
};
