import type { AgentInstance } from "@/lib/chat/agent-types";
import {
  buildMessageBody,
  buildHeaders,
  buildJsonBody,
  extractErrorText,
  extractSseFrames,
  readTextStream,
  summarizeHttpError,
} from "./http-common";
import { parseConfiguredJsonFrame, parseOpenAICompatibleFrame } from "./parsers";
import type { AgentConnector, AgentEvent, UnifiedChatRequest } from "./types";

async function* streamFromSseResponse(
  res: Response,
  agent: AgentInstance,
  parser: (frame: string) => { textDelta: string; snapshot: string; done: boolean },
): AsyncGenerator<AgentEvent> {
  if (!res.body) {
    const body = await res.text().catch(() => "");
    if (body.trim()) yield { type: "replace-text", text: body.trim() };
    yield { type: "done" };
    return;
  }

  let answer = "";
  let buffer = "";

  for await (const chunk of readTextStream(res.body)) {
    buffer += chunk;
    const extracted = extractSseFrames(buffer);
    buffer = extracted.rest;
    for (const frame of extracted.frames) {
      const parsed = parser(frame);
      if (parsed.textDelta) {
        answer += parsed.textDelta;
        yield { type: "text-delta", text: parsed.textDelta };
      }
      if (parsed.snapshot) {
        if (!answer || !parsed.snapshot.startsWith(answer)) {
          answer = parsed.snapshot;
          yield { type: "replace-text", text: parsed.snapshot };
        } else {
          const suffix = parsed.snapshot.slice(answer.length);
          answer = parsed.snapshot;
          if (suffix) yield { type: "text-delta", text: suffix };
        }
      }
      if (parsed.done) {
        yield { type: "done" };
        return;
      }
    }
  }

  if (buffer.trim()) {
    const parsed = parser(buffer);
    if (parsed.textDelta) yield { type: "text-delta", text: parsed.textDelta };
    if (parsed.snapshot) yield { type: "replace-text", text: parsed.snapshot };
  }

  yield { type: "done" };
}

async function* streamHttpSse(
  agent: AgentInstance,
  request: UnifiedChatRequest,
  parser: (frame: string) => { textDelta: string; snapshot: string; done: boolean },
): AsyncGenerator<AgentEvent> {
  if (!agent.endpoint.trim()) {
    yield { type: "error", message: "Missing agent endpoint." };
    return;
  }

  try {
    const res = await fetch(agent.endpoint.trim(), {
      method: "POST",
      headers: buildHeaders(agent),
      body: buildJsonBody(request.messages, agent),
      signal: request.abortSignal,
    });

    if (!res.ok) {
      const body = await res.text().catch(() => "");
      yield {
        type: "error",
        message: `Agent error (${res.status}): ${body || res.statusText}`.trim(),
      };
      return;
    }

    yield* streamFromSseResponse(res, agent, parser);
  } catch (error) {
    if (request.abortSignal.aborted) return;
    yield { type: "error", message: extractErrorText(error) };
  }
}

export const openAICompatibleConnector: AgentConnector = {
  id: "openai-compatible",
  async *stream(agent, request) {
    yield* streamHttpSse(agent, request, parseOpenAICompatibleFrame);
  },
  async testConnection(agent) {
    if (!agent.endpoint.trim()) {
      return { ok: false, message: "Missing agent endpoint." };
    }
    try {
      const body = buildMessageBody([{ role: "user", content: "ping" }], agent);
      const res = await fetch(agent.endpoint.trim(), {
        method: "POST",
        headers: buildHeaders(agent),
        body: JSON.stringify({
          ...body,
          stream: false,
        }),
      });
      if (!res.ok) {
        return { ok: false, message: await summarizeHttpError(res) };
      }
      return { ok: true, message: `Connection succeeded (HTTP ${res.status}).` };
    } catch (error) {
      return { ok: false, message: extractErrorText(error) };
    }
  },
};

export const genericHttpSseConnector: AgentConnector = {
  id: "generic-http-sse",
  async *stream(agent, request) {
    yield* streamHttpSse(agent, request, (frame) =>
      parseConfiguredJsonFrame(frame, {
        deltaPath: agent.deltaPath,
        snapshotPath: agent.snapshotPath,
        donePath: agent.donePath,
        doneSentinel: agent.doneSentinel,
      }),
    );
  },
  async testConnection(agent) {
    if (!agent.endpoint.trim()) {
      return { ok: false, message: "Missing agent endpoint." };
    }
    try {
      const res = await fetch(agent.endpoint.trim(), {
        method: "POST",
        headers: buildHeaders(agent),
        body: JSON.stringify({
          messages: [{ role: "user", content: "ping" }],
          stream: false,
          ...JSON.parse(agent.extraBodyJson || "{}"),
        }),
      });
      if (!res.ok) {
        return { ok: false, message: await summarizeHttpError(res) };
      }
      return { ok: true, message: `Connection succeeded (HTTP ${res.status}).` };
    } catch (error) {
      return { ok: false, message: extractErrorText(error) };
    }
  },
};
