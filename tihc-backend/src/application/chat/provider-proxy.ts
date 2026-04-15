import {
  generateText,
  streamText,
  type LanguageModelUsage,
  type ModelMessage,
} from "ai";
import type { CodexBridge } from "../../lib/codex-bridge";
import type { AppLogger } from "../../lib/logger";
import { STREAM_RESPONSE_HEADERS, jsonError } from "../../interfaces/http/http";
import {
  errorMessage,
  redactUrlForLogs,
  resolveEnvValue,
  truncateValue,
  type AppEnv,
} from "../../shared/support";
import {
  buildHostedProviderLanguageModel,
  getHostedProviderDefinition,
  resolveHostedProviderLogUrl,
  resolveHostedProviderRequest,
} from "./provider-registry";
import type { UsageRecordCallback } from "./usage-recorder";
import { encodeSseDelta, emptyJsonCompletion } from "./tidb-streaming";
import type { ChatCompletionsRequest, ChatRequestSummary } from "./chat-types";

export async function resolveProviderUsageSafely({
  usage,
  logger,
  provider,
  requestId,
  upstreamUrl,
}: {
  usage: PromiseLike<LanguageModelUsage | null | undefined> | LanguageModelUsage | null | undefined;
  logger: AppLogger;
  provider: string;
  requestId: string;
  upstreamUrl?: string;
}): Promise<LanguageModelUsage | null> {
  try {
    return (await usage) ?? null;
  } catch (error) {
    logger.warn("provider.usage_unavailable", {
      error_message: errorMessage(error),
      provider,
      request_id: requestId,
      upstream_url: redactUrlForLogs(upstreamUrl),
    });
    return null;
  }
}

export function buildUpstreamRequestBody(
  request: ChatCompletionsRequest,
  chatId?: string | null,
): string {
  return JSON.stringify({
    ...(chatId ? { chat_id: chatId } : {}),
    chat_engine: request.model?.trim() || "tidb",
    messages: Array.isArray(request.messages) ? request.messages : [],
    stream: Boolean(request.stream),
  });
}

export function buildProviderRequestBody(request: ChatCompletionsRequest): string {
  return JSON.stringify({
    model: request.model?.trim() || "",
    messages: Array.isArray(request.messages) ? request.messages : [],
    stream: Boolean(request.stream),
  });
}

function toModelMessages(request: ChatCompletionsRequest): ModelMessage[] {
  const messages = Array.isArray(request.messages) ? request.messages : [];
  return messages.map((message) => {
    if (message.role === "assistant") {
      return {
        role: "assistant",
        content: [{ type: "text", text: message.content || "" }],
      };
    }

    if (message.role === "system") {
      return {
        role: "system",
        content: message.content || "",
      };
    }

    return {
      role: "user",
      content: [{ type: "text", text: message.content || "" }],
    };
  });
}

export async function fetchTidbUpstream(
  env: AppEnv,
  requestBody: string,
  fetchImpl: typeof fetch,
  logger: AppLogger,
  requestId: string,
  requestSummary: ChatRequestSummary,
): Promise<Response> {
  const apiUrl = resolveEnvValue(env, "TIDB_API_URL");
  const apiToken = resolveEnvValue(env, "TIDB_API_TOKEN");

  if (!apiUrl || !apiToken) {
    logger.error("upstream.config_missing", {
      request_id: requestId,
      tidb_api_token_present: Boolean(apiToken),
      tidb_api_url: redactUrlForLogs(apiUrl),
      tidb_api_url_present: Boolean(apiUrl),
    });
    return jsonError(500, "Missing TIDB_API_URL or TIDB_API_TOKEN");
  }

  const startedAt = Date.now();
  logger.info("upstream.request", {
    message_count: requestSummary.messageCount,
    model: requestSummary.model,
    request_id: requestId,
    stream: requestSummary.stream,
    upstream_url: redactUrlForLogs(apiUrl),
  });

  let upstreamResponse: Response;
  try {
    upstreamResponse = await fetchImpl(apiUrl, {
      body: requestBody,
      headers: {
        Accept: "text/plain, application/json",
        Authorization: `Bearer ${apiToken}`,
        "Content-Type": "application/json",
      },
      method: "POST",
    });
  } catch (error) {
    logger.error("upstream.fetch_failed", {
      duration_ms: Date.now() - startedAt,
      error_message: errorMessage(error),
      request_id: requestId,
      upstream_url: redactUrlForLogs(apiUrl),
    });
    throw error;
  }

  logger.info("upstream.response", {
    duration_ms: Date.now() - startedAt,
    request_id: requestId,
    status: upstreamResponse.status,
    upstream_url: redactUrlForLogs(apiUrl),
  });

  if (!upstreamResponse.ok) {
    const body = await upstreamResponse.text().catch(() => "");
    logger.warn("upstream.rejected", {
      body_excerpt: truncateValue(body, 512),
      request_id: requestId,
      status: upstreamResponse.status,
      upstream_url: redactUrlForLogs(apiUrl),
    });
    return jsonError(
      upstreamResponse.status,
      `Upstream returned ${upstreamResponse.status}${body.trim() ? `: ${body.trim()}` : ""}`,
    );
  }

  return upstreamResponse;
}

export async function fetchProviderUpstream(
  env: AppEnv,
  provider: string,
  request: ChatCompletionsRequest,
  fetchImpl: typeof fetch,
  logger: AppLogger,
  requestId: string,
  requestSummary: ChatRequestSummary,
  options: {
    apiKeyOverride?: string | null;
    recordUsage?: UsageRecordCallback;
  } = {},
): Promise<Response> {
  const providerDefinition = getHostedProviderDefinition(provider);
  if (!providerDefinition) {
    return jsonError(400, `Unsupported provider: ${provider}`);
  }

  const providerRequest = resolveHostedProviderRequest(env, providerDefinition.id, options.apiKeyOverride);
  if (!providerRequest.ok) {
    logger.error("provider.config_missing", {
      error_message: providerRequest.errorMessage,
      provider,
      request_id: requestId,
    });
    return jsonError(500, providerRequest.errorMessage);
  }

  const apiUrl = resolveHostedProviderLogUrl(env, providerDefinition.id);
  const startedAt = Date.now();
  logger.info("provider.request", {
    message_count: requestSummary.messageCount,
    model: requestSummary.model,
    provider,
    request_id: requestId,
    stream: requestSummary.stream,
    upstream_url: redactUrlForLogs(apiUrl),
  });

  try {
    const model = buildHostedProviderLanguageModel({
      apiKey: providerRequest.apiKey,
      env,
      fetchImpl,
      modelId: requestSummary.model,
      providerId: providerDefinition.id,
    });
    const messages = toModelMessages(request);

    if (requestSummary.stream) {
      const result = streamText({
        model,
        messages,
      });
      const encoder = new TextEncoder();

      return new Response(
        new ReadableStream<Uint8Array>({
          async start(controller) {
            let emittedChars = 0;
            try {
              for await (const chunk of result.textStream) {
                if (!chunk) continue;
                emittedChars += chunk.length;
                controller.enqueue(encoder.encode(encodeSseDelta(chunk, false, requestSummary.model)));
              }
              logger.info("provider.response", {
                duration_ms: Date.now() - startedAt,
                emitted_chars: emittedChars,
                provider,
                request_id: requestId,
                status: 200,
                upstream_url: redactUrlForLogs(apiUrl),
              });
              const resolvedUsage = await resolveProviderUsageSafely({
                usage: result.usage,
                logger,
                provider,
                requestId,
                upstreamUrl: apiUrl,
              });
              await options.recordUsage?.({
                finishedAt: new Date().toISOString(),
                success: true,
                usage: resolvedUsage,
              });
              controller.enqueue(encoder.encode(encodeSseDelta("", true, requestSummary.model)));
            } catch (error) {
              logger.error("provider.fetch_failed", {
                duration_ms: Date.now() - startedAt,
                error_message: errorMessage(error),
                provider,
                request_id: requestId,
                upstream_url: redactUrlForLogs(apiUrl),
              });
              const failedUsage = await resolveProviderUsageSafely({
                usage: result.usage,
                logger,
                provider,
                requestId,
                upstreamUrl: apiUrl,
              });
              await options.recordUsage?.({
                errorCode: "provider_stream_failed",
                finishedAt: new Date().toISOString(),
                success: false,
                usage: failedUsage,
              });
              controller.enqueue(
                encoder.encode(
                  encodeSseDelta(`Provider stream error: ${errorMessage(error)}\n`, false, requestSummary.model),
                ),
              );
              controller.enqueue(encoder.encode(encodeSseDelta("", true, requestSummary.model)));
            } finally {
              controller.close();
            }
          },
        }),
        {
          headers: STREAM_RESPONSE_HEADERS,
          status: 200,
        },
      );
    }

    const result = await generateText({
      model,
      messages,
    });

    logger.info("provider.response", {
      duration_ms: Date.now() - startedAt,
      provider,
      request_id: requestId,
      status: 200,
      upstream_url: redactUrlForLogs(apiUrl),
    });
    await options.recordUsage?.({
      finishedAt: new Date().toISOString(),
      success: true,
      usage: result.usage,
    });
    return emptyJsonCompletion(requestSummary.model, result.text);
  } catch (error) {
    logger.error("provider.fetch_failed", {
      duration_ms: Date.now() - startedAt,
      error_message: errorMessage(error),
      provider,
      request_id: requestId,
      upstream_url: redactUrlForLogs(apiUrl),
    });
    await options.recordUsage?.({
      errorCode: "provider_request_failed",
      finishedAt: new Date().toISOString(),
      success: false,
      usage: null,
    });
    return jsonError(502, `Provider request failed: ${errorMessage(error)}`);
  }
}

export async function streamCodexProviderResponse(
  codexBridge: CodexBridge,
  request: ChatCompletionsRequest,
  model: string,
  recordUsage?: UsageRecordCallback,
): Promise<Response> {
  const generator = codexBridge.streamChat({
    model,
    messages: Array.isArray(request.messages) ? request.messages : [],
  });

  let firstChunk: IteratorResult<string, void>;
  try {
    firstChunk = await generator.next();
  } catch (error) {
    return jsonError(502, errorMessage(error));
  }

  const encoder = new TextEncoder();
  return new Response(
    new ReadableStream<Uint8Array>({
      async start(controller) {
        const push = (chunk: string) => {
          controller.enqueue(encoder.encode(chunk));
        };

        try {
          if (!firstChunk.done && firstChunk.value) {
            push(encodeSseDelta(firstChunk.value, false, model));
          }
          if (!firstChunk.done) {
            for await (const chunk of generator) {
              if (!chunk) continue;
              push(encodeSseDelta(chunk, false, model));
            }
          }
          await recordUsage?.({
            finishedAt: new Date().toISOString(),
            source: "unknown",
            success: true,
            usage: null,
          });
          push(encodeSseDelta("", true, model));
        } catch (error) {
          await recordUsage?.({
            errorCode: "codex_stream_failed",
            finishedAt: new Date().toISOString(),
            source: "unknown",
            success: false,
            usage: null,
          });
          push(encodeSseDelta(`Codex error: ${errorMessage(error)}`, false, model));
          push(encodeSseDelta("", true, model));
        } finally {
          controller.close();
        }
      },
    }),
    {
      headers: STREAM_RESPONSE_HEADERS,
      status: 200,
    },
  );
}

export async function completeCodexProviderResponse(
  codexBridge: CodexBridge,
  request: ChatCompletionsRequest,
  model: string,
  recordUsage?: UsageRecordCallback,
): Promise<Response> {
  try {
    let text = "";
    for await (const chunk of codexBridge.streamChat({
      model,
      messages: Array.isArray(request.messages) ? request.messages : [],
    })) {
      text += chunk;
    }
    await recordUsage?.({
      finishedAt: new Date().toISOString(),
      source: "unknown",
      success: true,
      usage: null,
    });
    return emptyJsonCompletion(model, text);
  } catch (error) {
    await recordUsage?.({
      errorCode: "codex_request_failed",
      finishedAt: new Date().toISOString(),
      source: "unknown",
      success: false,
      usage: null,
    });
    return jsonError(502, errorMessage(error));
  }
}
