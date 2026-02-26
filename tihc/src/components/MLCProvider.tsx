"use client";

import type { ReactNode } from "react";
import { useMemo } from "react";
import {
  AssistantRuntimeProvider,
  useLocalRuntime,
  type ChatModelAdapter,
} from "@assistant-ui/react";
import { createLocalThreadHistoryAdapter } from "@/lib/app/thread-history";

type ServerlessMessage = { role: string; content: string };
type ServerlessChatRequest = {
  messages: ServerlessMessage[];
  chat_engine: string;
  stream: boolean;
};
const SERVERLESS_BASE_URL = (import.meta.env.VITE_SERVERLESS_BASE_URL ?? "").trim();
const DEFAULT_CHAT_ENGINE = (import.meta.env.VITE_CHAT_ENGINE ?? "tidb").trim() || "tidb";

function stripHtmlComments(text: string): string {
  return text.replace(/<!--[\s\S]*?-->/g, "").trim();
}

function toServerMessages(
  messages: Parameters<ChatModelAdapter["run"]>[0]["messages"],
): ServerlessMessage[] {
  const tail = messages.slice(-12);
  const converted: ServerlessMessage[] = [];
  for (const msg of tail) {
    if (msg.role !== "user" && msg.role !== "assistant") continue;
    const content = stripHtmlComments(
      msg.content
        .filter((c) => c.type === "text")
        .map((c) => (c as { type: "text"; text: string }).text)
        .join("\n"),
    );
    if (!content) continue;
    converted.push({ role: msg.role, content });
  }
  return converted;
}

function tryParseJson(raw: string): unknown {
  try {
    return JSON.parse(raw);
  } catch {
    return raw;
  }
}

function isSpaceChar(char: string): boolean {
  return char === " " || char === "\n" || char === "\r" || char === "\t";
}

function skipSpaces(input: string, start: number): number {
  let index = start;
  while (index < input.length && isSpaceChar(input[index]!)) index += 1;
  return index;
}

function consumeJsonString(input: string, start: number): number | null {
  if (input[start] !== '"') return null;
  let escaped = false;
  for (let i = start + 1; i < input.length; i += 1) {
    const ch = input[i]!;
    if (escaped) {
      escaped = false;
      continue;
    }
    if (ch === "\\") {
      escaped = true;
      continue;
    }
    if (ch === '"') {
      return i + 1;
    }
  }
  return null;
}

function consumeJsonContainer(input: string, start: number): number | null {
  const first = input[start]!;
  if (first !== "{" && first !== "[") return null;

  const stack: string[] = [first === "{" ? "}" : "]"];
  let inString = false;
  let escaped = false;

  for (let i = start + 1; i < input.length; i += 1) {
    const ch = input[i]!;

    if (inString) {
      if (escaped) {
        escaped = false;
      } else if (ch === "\\") {
        escaped = true;
      } else if (ch === '"') {
        inString = false;
      }
      continue;
    }

    if (ch === '"') {
      inString = true;
      continue;
    }
    if (ch === "{") {
      stack.push("}");
      continue;
    }
    if (ch === "[") {
      stack.push("]");
      continue;
    }
    if (ch === "}" || ch === "]") {
      const expected = stack.pop();
      if (expected !== ch) return null;
      if (stack.length === 0) return i + 1;
    }
  }
  return null;
}

function consumeJsonNumber(input: string, start: number): number | null {
  const match = input.slice(start).match(/^-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?/);
  if (!match) return null;
  return start + match[0].length;
}

function consumeJsonValue(input: string, start: number): number | null {
  const index = skipSpaces(input, start);
  if (index >= input.length) return null;
  const first = input[index]!;

  if (first === '"') return consumeJsonString(input, index);
  if (first === "{" || first === "[") return consumeJsonContainer(input, index);
  if (first === "-" || (first >= "0" && first <= "9")) {
    return consumeJsonNumber(input, index);
  }
  if (input.startsWith("true", index)) return index + 4;
  if (input.startsWith("false", index)) return index + 5;
  if (input.startsWith("null", index)) return index + 4;
  return null;
}

function extractProtocolFrames(buffer: string): { frames: string[]; rest: string } {
  const frames: string[] = [];
  let index = 0;

  while (index < buffer.length) {
    index = skipSpaces(buffer, index);
    if (index >= buffer.length) return { frames, rest: "" };

    const frameStart = index;
    if (buffer.slice(index, index + 5).toLowerCase() === "data:") {
      index += 5;
      index = skipSpaces(buffer, index);
      if (index >= buffer.length) return { frames, rest: "" };
    }

    if (buffer.startsWith("[DONE]", index)) {
      frames.push("[DONE]");
      index += "[DONE]".length;
      continue;
    }

    const codeStart = index;
    while (index < buffer.length && /[0-9a-zA-Z]/.test(buffer[index]!)) {
      index += 1;
    }
    if (index === codeStart || buffer[index] !== ":") {
      return { frames, rest: buffer.slice(frameStart) };
    }

    const code = buffer.slice(codeStart, index).toLowerCase();
    index += 1;
    const payloadStart = skipSpaces(buffer, index);
    const payloadEnd = consumeJsonValue(buffer, payloadStart);
    if (payloadEnd === null) {
      return { frames, rest: buffer.slice(frameStart) };
    }

    frames.push(`${code}:${buffer.slice(payloadStart, payloadEnd)}`);
    index = payloadEnd;
  }

  return { frames, rest: "" };
}

function asRecord(value: unknown): Record<string, unknown> | null {
  if (!value || typeof value !== "object" || Array.isArray(value)) return null;
  return value as Record<string, unknown>;
}

function asArray(value: unknown): unknown[] {
  if (Array.isArray(value)) return value;
  return [value];
}

function toShortText(value: unknown, maxLen = 120): string | null {
  if (typeof value !== "string") return null;
  const text = value.trim();
  if (!text) return null;
  if (text.length <= maxLen) return text;
  return `${text.slice(0, maxLen)}...`;
}

function extractProgress(payload: unknown): { lines: string[]; finished: boolean } {
  const lines: string[] = [];
  let finished = false;
  for (const item of asArray(payload)) {
    const obj = asRecord(item);
    if (!obj) continue;

    const display = toShortText(obj.display);
    const state = toShortText(obj.state);
    const message = toShortText(obj.message);
    const stateUpper =
      typeof obj.state === "string" ? obj.state.trim().toUpperCase() : "";
    if (stateUpper === "FINISHED") {
      finished = true;
      continue;
    }
    if (display) lines.push(`检索中：${display}`);
    else if (state) lines.push(`检索状态：${state}`);
    if (message) lines.push(`检索信息：${message}`);

    const kbId = obj.knowledge_base_id;
    const entities = Array.isArray(obj.entities) ? obj.entities.length : null;
    const relationships = Array.isArray(obj.relationships) ? obj.relationships.length : null;
    if (kbId !== undefined || entities !== null || relationships !== null) {
      lines.push(
        `命中知识库：${kbId ?? "-"}，实体：${entities ?? 0}，关系：${relationships ?? 0}`,
      );
    }

    const query = toShortText(obj.query, 80);
    if (query) lines.push(`检索查询：${query}`);

  }
  return { lines, finished };
}

function extractCode0AnswerDelta(payload: unknown): string {
  if (typeof payload === "string") return payload;
  const obj = asRecord(payload);
  if (!obj) return "";

  const delta = obj.delta;
  if (typeof delta === "string") return delta;

  const text = obj.text;
  if (typeof text === "string") return text;

  const content = obj.content;
  if (typeof content === "string") return content;

  return "";
}

function extractCode2Snapshot(payload: unknown): {
  answerSnapshot: string;
  done: boolean;
} {
  let answerSnapshot = "";
  let done = false;

  for (const item of asArray(payload)) {
    const obj = asRecord(item);
    if (!obj) continue;

    const assistantMessage = asRecord(obj.assistant_message);
    if (assistantMessage) {
      const content = assistantMessage.content;
      if (typeof content === "string" && content.trim()) {
        answerSnapshot = content;
      }
      if (
        typeof assistantMessage.finished_at === "string" &&
        assistantMessage.finished_at.trim()
      ) {
        done = true;
      }
      continue;
    }

    if (obj.role === "assistant") {
      const content = obj.content;
      if (typeof content === "string" && content.trim()) {
        answerSnapshot = content;
      }
      if (typeof obj.finished_at === "string" && obj.finished_at.trim()) {
        done = true;
      }
    }
  }

  return { answerSnapshot, done };
}

function parseProtocolLine(line: string): {
  answerDelta: string;
  answerSnapshot: string;
  progress: string[];
  progressFinished: boolean;
  matched: boolean;
  done: boolean;
} {
  const normalized = line.replace(/^data:\s*/i, "").trim();
  if (!normalized) {
    return {
      answerDelta: "",
      answerSnapshot: "",
      progress: [],
      progressFinished: false,
      matched: false,
      done: false,
    };
  }
  if (normalized === "[DONE]") {
    return {
      answerDelta: "",
      answerSnapshot: "",
      progress: [],
      progressFinished: false,
      matched: true,
      done: true,
    };
  }

  const match = normalized.match(/^([0-9a-zA-Z]+):(.*)$/s);
  if (!match) {
    return {
      answerDelta: "",
      answerSnapshot: "",
      progress: [],
      progressFinished: false,
      matched: false,
      done: false,
    };
  }

  const code = match[1].toLowerCase();
  const rawPayload = match[2].trim();
  const payload = tryParseJson(rawPayload);

  if (code === "8") {
    const progress = extractProgress(payload);
    return {
      answerDelta: "",
      answerSnapshot: "",
      progress: progress.lines,
      progressFinished: progress.finished,
      matched: true,
      done: false,
    };
  }

  if (code === "0") {
    return {
      answerDelta: extractCode0AnswerDelta(payload),
      answerSnapshot: "",
      progress: [],
      progressFinished: false,
      matched: true,
      done: false,
    };
  }

  if (code === "2") {
    const snapshot = extractCode2Snapshot(payload);
    return {
      answerDelta: "",
      answerSnapshot: snapshot.answerSnapshot,
      progress: [],
      progressFinished: false,
      matched: true,
      done: snapshot.done,
    };
  }

  return {
    answerDelta: "",
    answerSnapshot: "",
    progress: [],
    progressFinished: false,
    matched: true,
    done: false,
  };
}

function extractPlainAnswerLine(line: string): string {
  const normalized = line.replace(/^data:\s*/i, "").trim();
  if (!normalized) return "";
  if (/^(event|id):/i.test(normalized)) return "";
  if (/^[0-9a-zA-Z]+:/.test(normalized)) return "";
  if (normalized.startsWith("{") || normalized.startsWith("[")) return "";
  return normalized;
}

const MyModelAdapter: ChatModelAdapter = {
  async *run({ messages, abortSignal }) {
    const lastUser = messages.filter((m) => m.role === "user").at(-1);
    if (!lastUser) {
      yield { content: [{ type: "text", text: "请先输入你的问题。" }] };
      return;
    }

    if (!SERVERLESS_BASE_URL) {
      yield {
        content: [
          {
            type: "text",
            text: "Missing VITE_SERVERLESS_BASE_URL. Please configure .env.local.",
          },
        ],
      };
      return;
    }

    let url = "";
    try {
      url = new URL("/api/stream_chat", SERVERLESS_BASE_URL).toString();
    } catch {
      yield {
        content: [
          {
            type: "text",
            text: "Invalid VITE_SERVERLESS_BASE_URL. Please provide a valid URL.",
          },
        ],
      };
      return;
    }

    const requestBody: ServerlessChatRequest = {
      messages: toServerMessages(messages),
      chat_engine: DEFAULT_CHAT_ENGINE,
      stream: true,
    };

    const res = await fetch(url, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(requestBody),
      signal: abortSignal,
    });

    if (!res.ok) {
      const body = await res.text().catch(() => "");
      yield {
        content: [
          {
            type: "text",
            text: `Serverless error (${res.status}): ${body || res.statusText}`.trim(),
          },
        ],
      };
      return;
    }
    if (!res.body) {
      const body = await res.text().catch(() => "");
      yield { content: [{ type: "text", text: body.trim() }] };
      return;
    }

    const reader = res.body.getReader();
    const decoder = new TextDecoder("utf-8");
    let protocolMode: boolean | null = null;
    let probeBuffer = "";
    let frameBuffer = "";
    let progressHeaderShown = false;
    let answerHeaderShown = false;
    let progressFinished = false;
    let answerBuffer = "";
    let renderedText = "";
    const seenProgress = new Set<string>();
    const FINISHED_IDLE_TIMEOUT_MS = 1800;
    const textChunk = (text: string) => ({
      content: [{ type: "text" as const, text }],
    });
    const appendRendered = (text: string) => {
      if (!text) return null;
      renderedText += text;
      return textChunk(renderedText);
    };
    const answerHeaderText = "\n回答：\n";

    const collectProgressTexts = (lines: string[]): string[] => {
      const uniq = lines.filter((line) => {
        const key = line.trim();
        if (!key || seenProgress.has(key)) return false;
        seenProgress.add(key);
        return true;
      });
      if (!uniq.length) return [];
      const out: string[] = [];
      if (!progressHeaderShown) {
        progressHeaderShown = true;
        out.push("检索过程：\n");
      }
      for (const line of uniq) {
        out.push(`- ${line}\n`);
      }
      return out;
    };

    const collectAnswerTexts = (text: string): string[] => {
      if (!text) return [];
      const out: string[] = [];
      if (!answerHeaderShown) {
        answerHeaderShown = true;
        out.push(answerHeaderText);
      }
      out.push(text);
      return out;
    };

    while (true) {
      if (abortSignal.aborted) return;
      const canIdleFinish = progressFinished && !!answerBuffer.trim();
      const readResult = canIdleFinish
        ? await Promise.race<
            ReadableStreamReadResult<Uint8Array> | "__finished_idle_timeout__"
          >([
            reader.read(),
            new Promise<"__finished_idle_timeout__">((resolve) => {
              setTimeout(() => resolve("__finished_idle_timeout__"), FINISHED_IDLE_TIMEOUT_MS);
            }),
          ])
        : await reader.read();
      if (readResult === "__finished_idle_timeout__") {
        await reader.cancel().catch(() => undefined);
        break;
      }
      const { value, done } = readResult;
      if (done) break;
      if (!value) continue;
      const chunkText = decoder.decode(value, { stream: true });
      if (!chunkText) continue;

      if (protocolMode === null) {
        probeBuffer += chunkText;
        const probe = probeBuffer.replace(/\r/g, "");
        if (/(^|\n)\s*(?:data:\s*)?\d+:/i.test(probe)) {
          protocolMode = true;
        } else if (probe.length > 256 || probe.includes("\n")) {
          protocolMode = false;
        }
      }

      if (protocolMode === null) {
        frameBuffer += chunkText;
        continue;
      }

      if (!protocolMode) {
        const plain = frameBuffer + chunkText;
        frameBuffer = "";
        const out = appendRendered(plain);
        if (out) yield out;
        continue;
      }

      frameBuffer += chunkText;
      const { frames, rest } = extractProtocolFrames(frameBuffer);
      frameBuffer = rest;
      for (const frame of frames) {
        const parsed = parseProtocolLine(frame);
        if (!parsed.matched) {
          const fallbackAnswer = extractPlainAnswerLine(frame);
          for (const text of collectAnswerTexts(fallbackAnswer)) {
            if (text !== answerHeaderText) answerBuffer += text;
            const out = appendRendered(text);
            if (out) yield out;
          }
          continue;
        }
        if (parsed.progressFinished) progressFinished = true;
        for (const text of collectProgressTexts(parsed.progress)) {
          const out = appendRendered(text);
          if (out) yield out;
        }
        for (const text of collectAnswerTexts(parsed.answerDelta)) {
          if (text !== answerHeaderText) answerBuffer += text;
          const out = appendRendered(text);
          if (out) yield out;
        }
        if (parsed.answerSnapshot.trim()) {
          if (!answerBuffer.trim()) {
            for (const text of collectAnswerTexts(parsed.answerSnapshot)) {
              if (text !== answerHeaderText) answerBuffer += text;
              const out = appendRendered(text);
              if (out) yield out;
            }
          } else if (parsed.answerSnapshot.startsWith(answerBuffer)) {
            const suffix = parsed.answerSnapshot.slice(answerBuffer.length);
            for (const text of collectAnswerTexts(suffix)) {
              if (text !== answerHeaderText) answerBuffer += text;
              const out = appendRendered(text);
              if (out) yield out;
            }
          }
        }
        if (parsed.done) {
          await reader.cancel().catch(() => undefined);
          return;
        }
      }
    }
    const tail = decoder.decode();
    if (tail) frameBuffer += tail;
    if (!frameBuffer.trim()) {
      if (progressFinished && !answerBuffer.trim()) {
        const out = appendRendered("\n回答：\n（后端未返回可展示的正文内容）");
        if (out) yield out;
      }
      return;
    }

    if (!protocolMode) {
      const out = appendRendered(frameBuffer.replace(/\r/g, "").trim());
      if (out) yield out;
      return;
    }
    const finalParsed = extractProtocolFrames(frameBuffer);
    for (const frame of finalParsed.frames) {
      const parsed = parseProtocolLine(frame);
      if (!parsed.matched) {
        const fallbackAnswer = extractPlainAnswerLine(frame);
        for (const text of collectAnswerTexts(fallbackAnswer)) {
          if (text !== answerHeaderText) answerBuffer += text;
          const out = appendRendered(text);
          if (out) yield out;
        }
        continue;
      }
      if (parsed.progressFinished) progressFinished = true;
      for (const text of collectProgressTexts(parsed.progress)) {
        const out = appendRendered(text);
        if (out) yield out;
      }
      for (const text of collectAnswerTexts(parsed.answerDelta)) {
        if (text !== answerHeaderText) answerBuffer += text;
        const out = appendRendered(text);
        if (out) yield out;
      }
      if (parsed.answerSnapshot.trim()) {
        if (!answerBuffer.trim()) {
          for (const text of collectAnswerTexts(parsed.answerSnapshot)) {
            if (text !== answerHeaderText) answerBuffer += text;
            const out = appendRendered(text);
            if (out) yield out;
          }
        } else if (parsed.answerSnapshot.startsWith(answerBuffer)) {
          const suffix = parsed.answerSnapshot.slice(answerBuffer.length);
          for (const text of collectAnswerTexts(suffix)) {
            if (text !== answerHeaderText) answerBuffer += text;
            const out = appendRendered(text);
            if (out) yield out;
          }
        }
      }
      if (parsed.done) {
        return;
      }
    }
    const remaining = finalParsed.rest.replace(/\r/g, "").trim();
    if (remaining) {
      const fallbackAnswer = extractPlainAnswerLine(remaining);
      for (const text of collectAnswerTexts(fallbackAnswer)) {
        if (text !== answerHeaderText) answerBuffer += text;
        const out = appendRendered(text);
        if (out) yield out;
      }
    }
    if (progressFinished && !answerBuffer.trim()) {
      const out = appendRendered("\n回答：\n（后端未返回可展示的正文内容）");
      if (out) yield out;
    }
  },
};

export function MLCProvider({
  children,
}: Readonly<{
  children: ReactNode;
}>) {
  const historyAdapter = useMemo(() => createLocalThreadHistoryAdapter(), []);
  const runtimeOptions = useMemo(
    () => ({
      adapters: {
        history: historyAdapter,
      },
    }),
    [historyAdapter],
  );
  const runtime = useLocalRuntime(MyModelAdapter, runtimeOptions);

  return (
    <AssistantRuntimeProvider runtime={runtime}>
      {children}
    </AssistantRuntimeProvider>
  );
}
