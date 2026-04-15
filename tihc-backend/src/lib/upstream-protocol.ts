type ParsedUpstreamEvent =
  | { type: "answer-delta"; text: string }
  | { type: "answer-snapshot"; text: string; done: boolean }
  | { type: "chat-binding"; chatId: string }
  | { type: "progress-text"; text: string }
  | { type: "plain-text"; text: string };

function tryParseJson(raw: string): unknown {
  try {
    return JSON.parse(raw);
  } catch {
    return raw;
  }
}

function asRecord(value: unknown): Record<string, unknown> | null {
  if (!value || typeof value !== "object" || Array.isArray(value)) return null;
  return value as Record<string, unknown>;
}

function asArray(value: unknown): unknown[] {
  return Array.isArray(value) ? value : [value];
}

function toShortText(value: unknown, maxLen = 120): string | null {
  if (typeof value !== "string") return null;
  const text = value.trim();
  if (!text) return null;
  if (text.length <= maxLen) return text;
  return `${text.slice(0, maxLen)}...`;
}

function parseProgressText(payload: unknown): ParsedUpstreamEvent[] {
  const events: ParsedUpstreamEvent[] = [];

  for (const item of asArray(payload)) {
    const obj = asRecord(item);
    if (!obj) continue;

    const display = toShortText(obj.display);
    const state = toShortText(obj.state);
    const message = toShortText(obj.message);
    const stateUpper = typeof obj.state === "string" ? obj.state.trim().toUpperCase() : "";

    if (stateUpper === "FINISHED") continue;
    if (display) {
      events.push({ type: "progress-text", text: `Retrieving: ${display}\n` });
    }
    if (state) {
      events.push({ type: "progress-text", text: `Retrieval state: ${state}\n` });
    }
    if (message) {
      events.push({ type: "progress-text", text: `Retrieval message: ${message}\n` });
    }
  }

  return events;
}

function parseAnswerDelta(payload: unknown): ParsedUpstreamEvent[] {
  if (typeof payload === "string" && payload) {
    return [{ type: "answer-delta", text: payload }];
  }

  const obj = asRecord(payload);
  if (!obj) return [];

  for (const key of ["delta", "text", "content"] as const) {
    const value = obj[key];
    if (typeof value === "string" && value) {
      return [{ type: "answer-delta", text: value }];
    }
  }

  return [];
}

function parseAnswerSnapshot(payload: unknown): ParsedUpstreamEvent[] {
  const events: ParsedUpstreamEvent[] = [];

  for (const item of asArray(payload)) {
    const obj = asRecord(item);
    if (!obj) continue;

    const chat = asRecord(obj.chat);
    const chatId = toShortText(chat?.id, 128);
    if (chatId) {
      events.push({ type: "chat-binding", chatId });
    }

    const assistantMessage = asRecord(obj.assistant_message);
    if (assistantMessage) {
      const content = assistantMessage.content;
      if (typeof content === "string" && content.trim()) {
        events.push({
          type: "answer-snapshot",
          text: content,
          done:
            typeof assistantMessage.finished_at === "string" &&
            assistantMessage.finished_at.trim().length > 0,
        });
        return events;
      }
      continue;
    }

    if (obj.role === "assistant" && typeof obj.content === "string" && obj.content.trim()) {
      events.push({
        type: "answer-snapshot",
        text: obj.content,
        done: typeof obj.finished_at === "string" && obj.finished_at.trim().length > 0,
      });
      return events;
    }
  }

  return events;
}

export function parseUpstreamFrame(frame: string): ParsedUpstreamEvent[] {
  const normalized = frame.trim();
  if (!normalized) return [];

  const match = normalized.match(/^([0-9a-zA-Z]+):(.*)$/s);
  if (!match) {
    return [{ type: "plain-text", text: normalized }];
  }

  const code = match[1].toLowerCase();
  const payload = tryParseJson(match[2].trim());

  if (code === "8") return parseProgressText(payload);
  if (code === "0") return parseAnswerDelta(payload);
  if (code === "2") return parseAnswerSnapshot(payload);
  return [];
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
  for (let index = start + 1; index < input.length; index += 1) {
    const char = input[index]!;
    if (escaped) {
      escaped = false;
      continue;
    }
    if (char === "\\") {
      escaped = true;
      continue;
    }
    if (char === '"') {
      return index + 1;
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

  for (let index = start + 1; index < input.length; index += 1) {
    const char = input[index]!;

    if (inString) {
      if (escaped) {
        escaped = false;
      } else if (char === "\\") {
        escaped = true;
      } else if (char === '"') {
        inString = false;
      }
      continue;
    }

    if (char === '"') {
      inString = true;
      continue;
    }
    if (char === "{") {
      stack.push("}");
      continue;
    }
    if (char === "[") {
      stack.push("]");
      continue;
    }
    if (char === "}" || char === "]") {
      const expected = stack.pop();
      if (expected !== char) return null;
      if (stack.length === 0) return index + 1;
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
  if (first === "-" || (first >= "0" && first <= "9")) return consumeJsonNumber(input, index);
  if (input.startsWith("true", index)) return index + 4;
  if (input.startsWith("false", index)) return index + 5;
  if (input.startsWith("null", index)) return index + 4;
  return null;
}

export function extractUpstreamFrames(buffer: string): { frames: string[]; rest: string } {
  const frames: string[] = [];
  let index = 0;

  while (index < buffer.length) {
    index = skipSpaces(buffer, index);
    if (index >= buffer.length) return { frames, rest: "" };

    const frameStart = index;
    if (buffer.slice(index, index + 5).toLowerCase() === "data:") {
      index += 5;
      index = skipSpaces(buffer, index);
      if (index >= buffer.length) {
        return { frames, rest: buffer.slice(frameStart) };
      }
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
      const nextLineBreak = buffer.indexOf("\n", frameStart);
      if (nextLineBreak < 0) return { frames, rest: buffer.slice(frameStart) };
      frames.push(buffer.slice(frameStart, nextLineBreak).trim());
      index = nextLineBreak + 1;
      continue;
    }

    index += 1;
    const payloadStart = skipSpaces(buffer, index);
    const payloadEnd = consumeJsonValue(buffer, payloadStart);
    if (payloadEnd === null) {
      return { frames, rest: buffer.slice(frameStart) };
    }

    frames.push(`${buffer.slice(codeStart, codeStart + (index - codeStart - 1)).toLowerCase()}:${buffer.slice(payloadStart, payloadEnd)}`);
    index = payloadEnd;
  }

  return { frames, rest: "" };
}

export type { ParsedUpstreamEvent };
