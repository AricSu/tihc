import type { AppLogger } from "../../lib/logger";
import {
  extractUpstreamFrames,
  parseUpstreamFrame,
  type ParsedUpstreamEvent,
} from "../../lib/upstream-protocol";
import {
  JSON_RESPONSE_HEADERS,
  MAX_BODY_BYTES,
  STREAM_RESPONSE_HEADERS,
  jsonError,
} from "../../interfaces/http/http";
import { errorMessage } from "../../shared/support";
import type { ChatStreamState } from "./chat-types";
import type { UsageRecordCallback } from "./usage-recorder";

type TidbChatBindingCallback = (chatId: string) => Promise<void> | void;

export function createChatStreamState(): ChatStreamState {
  return {
    answerText: "",
    emittedText: "",
    streamedText: [],
  };
}

function suffixSnapshot(previous: string, next: string): string {
  if (!previous) return next;
  if (next.startsWith(previous)) return next.slice(previous.length);
  return next;
}

export function applyEventToState(event: ParsedUpstreamEvent, state: ChatStreamState) {
  if (event.type === "chat-binding") {
    return;
  }

  if (event.type === "progress-text") {
    state.streamedText.push(event.text);
    return;
  }

  if (event.type === "answer-delta") {
    state.answerText += event.text;
    state.streamedText.push(event.text);
    return;
  }

  if (event.type === "answer-snapshot") {
    const delta = suffixSnapshot(state.answerText, event.text);
    state.answerText = event.text;
    if (delta) {
      state.streamedText.push(delta);
    }
    return;
  }

  if (event.type === "plain-text") {
    const normalized = event.text.trim() ? `${event.text.trim()}\n` : "";
    if (!normalized) return;
    state.streamedText.push(normalized);
  }
}

async function* iterateUpstreamEvents(
  upstreamResponse: Response,
): AsyncGenerator<ParsedUpstreamEvent> {
  if (!upstreamResponse.body) {
    const text = await upstreamResponse.text().catch(() => "");
    if (text.trim()) {
      yield { type: "plain-text", text };
    }
    return;
  }

  const reader = upstreamResponse.body.getReader();
  const decoder = new TextDecoder("utf-8");
  let buffer = "";

  try {
    while (true) {
      const { done, value } = await reader.read();
      if (done) break;
      if (!value) continue;

      buffer += decoder.decode(value, { stream: true });
      const extracted = extractUpstreamFrames(buffer);
      buffer = extracted.rest;

      for (const frame of extracted.frames) {
        if (frame === "[DONE]") return;
        const events = parseUpstreamFrame(frame);
        if (!events.length && frame.trim()) {
          yield { type: "plain-text", text: frame };
          continue;
        }
        for (const event of events) {
          yield event;
        }
      }
    }

    const tail = decoder.decode();
    if (tail) buffer += tail;
    if (buffer.trim()) {
      const events = parseUpstreamFrame(buffer.trim());
      if (events.length) {
        for (const event of events) {
          yield event;
        }
      } else {
        yield { type: "plain-text", text: buffer.trim() };
      }
    }
  } finally {
    reader.releaseLock();
  }
}

export function encodeSseDelta(content: string, done = false, model = "tidb"): string {
  if (done) return "data: [DONE]\n\n";

  return `data: ${JSON.stringify({
    choices: [
      {
        delta: {
          content,
        },
        finish_reason: null,
        index: 0,
      },
    ],
    created: Math.floor(Date.now() / 1000),
    id: `chatcmpl_${crypto.randomUUID()}`,
    model,
    object: "chat.completion.chunk",
  })}\n\n`;
}

export function emptyJsonCompletion(model: string, content: string): Response {
  return new Response(
    JSON.stringify({
      choices: [
        {
          finish_reason: "stop",
          index: 0,
          message: {
            content,
            role: "assistant",
          },
        },
      ],
      created: Math.floor(Date.now() / 1000),
      id: `chatcmpl_${crypto.randomUUID()}`,
      model,
      object: "chat.completion",
    }),
    {
      headers: JSON_RESPONSE_HEADERS,
      status: 200,
    },
  );
}

export function createTidbStreamResponse(
  upstreamResponse: Response,
  logger: AppLogger,
  requestId: string,
  model: string,
  recordUsage?: UsageRecordCallback,
  onChatBinding?: TidbChatBindingCallback,
): Response {
  const stream = new ReadableStream<Uint8Array>({
    async start(controller) {
      const encoder = new TextEncoder();
      const state = createChatStreamState();
      let eventCount = 0;
      let latestChatId: string | null = null;

      try {
        // The TiDB upstream can emit delta events, snapshots, and plain text frames.
        // We normalize them into OpenAI-style SSE chunks at the edge of the app.
        for await (const event of iterateUpstreamEvents(upstreamResponse)) {
          eventCount += 1;
          if (event.type === "chat-binding") {
            if (event.chatId !== latestChatId) {
              latestChatId = event.chatId;
              await onChatBinding?.(event.chatId);
            }
            continue;
          }
          applyEventToState(event, state);
          while (state.streamedText.length > 0) {
            const nextText = state.streamedText.shift();
            if (!nextText) continue;
            controller.enqueue(encoder.encode(encodeSseDelta(nextText, false, model)));
            state.emittedText += nextText;
          }
        }
        logger.info("chat.stream_completed", {
          emitted_chars: state.emittedText.length,
          event_count: eventCount,
          model,
          request_id: requestId,
        });
        await recordUsage?.({
          finishedAt: new Date().toISOString(),
          source: "unknown",
          success: true,
          usage: null,
        });
        controller.enqueue(encoder.encode(encodeSseDelta("", true, model)));
        controller.close();
      } catch (error) {
        const message = errorMessage(error) || "Unknown upstream stream error";
        logger.error("chat.stream_failed", {
          error_message: message,
          event_count: eventCount,
          model,
          request_id: requestId,
        });
        await recordUsage?.({
          errorCode: "tidb_stream_failed",
          finishedAt: new Date().toISOString(),
          source: "unknown",
          success: false,
          usage: null,
        });
        controller.enqueue(encoder.encode(encodeSseDelta(`Upstream stream error: ${message}\n`)));
        controller.enqueue(encoder.encode(encodeSseDelta("", true, model)));
        controller.close();
      }
    },
  });

  return new Response(stream, {
    headers: STREAM_RESPONSE_HEADERS,
    status: 200,
  });
}

export async function createTidbCompletionResponse(
  upstreamResponse: Response,
  logger: AppLogger,
  requestId: string,
  model: string,
  recordUsage?: UsageRecordCallback,
  onChatBinding?: TidbChatBindingCallback,
): Promise<Response> {
  const state = createChatStreamState();
  let latestChatId: string | null = null;
  try {
    for await (const event of iterateUpstreamEvents(upstreamResponse)) {
      if (event.type === "chat-binding") {
        if (event.chatId !== latestChatId) {
          latestChatId = event.chatId;
          await onChatBinding?.(event.chatId);
        }
        continue;
      }
      applyEventToState(event, state);
      if (state.streamedText.length > 0) {
        state.emittedText += state.streamedText.join("");
        state.streamedText = [];
      }
      if (new TextEncoder().encode(state.emittedText).byteLength > MAX_BODY_BYTES) {
        logger.error("chat.response_too_large", {
          emitted_chars: state.emittedText.length,
          model,
          request_id: requestId,
        });
        await recordUsage?.({
          errorCode: "tidb_response_too_large",
          finishedAt: new Date().toISOString(),
          source: "unknown",
          success: false,
          usage: null,
        });
        return jsonError(413, "Response body exceeds the 4.5 MB Vercel Functions limit.");
      }
    }

    logger.info("chat.non_stream_completed", {
      emitted_chars: state.emittedText.length,
      model,
      request_id: requestId,
    });
    await recordUsage?.({
      finishedAt: new Date().toISOString(),
      source: "unknown",
      success: true,
      usage: null,
    });
    return emptyJsonCompletion(model, state.emittedText);
  } catch (error) {
    await recordUsage?.({
      errorCode: "tidb_non_stream_failed",
      finishedAt: new Date().toISOString(),
      source: "unknown",
      success: false,
      usage: null,
    });
    throw error;
  }
}
