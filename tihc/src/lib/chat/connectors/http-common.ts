import type { AgentInstance } from "@/lib/chat/agent-types";
import type { ChatMessage } from "./types";

function tryParseObject(raw: string): Record<string, unknown> {
  const text = raw.trim();
  if (!text) return {};
  try {
    const parsed = JSON.parse(text);
    if (!parsed || typeof parsed !== "object" || Array.isArray(parsed)) return {};
    return parsed as Record<string, unknown>;
  } catch {
    return {};
  }
}

export function buildHeaders(agent: AgentInstance): Record<string, string> {
  const headers: Record<string, string> = {
    "Content-Type": "application/json",
  };
  const customHeaders = tryParseObject(agent.headersJson);
  for (const [key, value] of Object.entries(customHeaders)) {
    if (typeof value === "string") headers[key] = value;
  }
  if (agent.apiKey.trim() && !headers.Authorization) {
    headers.Authorization = `Bearer ${agent.apiKey.trim()}`;
  }
  return headers;
}

export function buildMessageBody(messages: ChatMessage[], agent: AgentInstance): Record<string, unknown> {
  const extraBody = tryParseObject(agent.extraBodyJson);
  return {
    messages,
    model: agent.model.trim() || undefined,
    ...extraBody,
    stream: true,
  };
}

export function buildJsonBody(messages: ChatMessage[], agent: AgentInstance): string {
  return JSON.stringify(buildMessageBody(messages, agent));
}

export function extractErrorText(error: unknown): string {
  if (error instanceof Error && error.message.trim()) return error.message;
  return String(error || "Unknown error");
}

export async function summarizeHttpError(res: Response): Promise<string> {
  const body = await res.text().catch(() => "");
  return `HTTP ${res.status}${body.trim() ? `: ${body.trim()}` : ""}`;
}

export function extractSseFrames(buffer: string): { frames: string[]; rest: string } {
  const normalized = buffer.replace(/\r/g, "");
  const parts = normalized.split("\n\n");
  const rest = parts.pop() ?? "";
  const frames = parts
    .map((part) =>
      part
        .split("\n")
        .filter((line) => line.trim().startsWith("data:"))
        .join("\n")
        .trim(),
    )
    .filter(Boolean);
  return { frames, rest };
}

export async function* readTextStream(
  stream: ReadableStream<Uint8Array>,
): AsyncGenerator<string> {
  const reader = stream.getReader();
  const decoder = new TextDecoder("utf-8");
  while (true) {
    const { value, done } = await reader.read();
    if (done) break;
    if (!value) continue;
    const text = decoder.decode(value, { stream: true });
    if (text) yield text;
  }
  const tail = decoder.decode();
  if (tail) yield tail;
}
