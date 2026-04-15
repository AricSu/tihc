import {
  sanitizePrincipalLlmCredentialInput,
} from "../cases/case-payloads";
import {
  buildLlmProviderCatalog,
  buildUpstreamRequestBody,
  completeCodexProviderResponse,
  createTidbCompletionResponse,
  createTidbStreamResponse,
  fetchProviderUpstream,
  fetchTidbUpstream,
  streamCodexProviderResponse,
  type ChatCompletionsRequest,
} from "../../../application/chat/chat-completions";
import {
  extractTidbChatIdFromHistory,
  persistTidbChatBinding,
} from "../../../application/chat/tidb-chat-binding";
import { supportsUserLlmCredentialProvider } from "../../../application/chat/provider-registry";
import { createUsageRecordCallback } from "../../../application/chat/usage-recorder";
import type { CaseStore } from "../../../domain/cases/case-store";
import type { UsageStore } from "../../../domain/usage/usage-store";
import { extractBearerToken } from "../../../lib/google-auth";
import type { CodexBridge } from "../../../lib/codex-bridge";
import type { AppLogger } from "../../../lib/logger";
import {
  JSON_RESPONSE_HEADERS,
  STREAM_RESPONSE_HEADERS,
  jsonError,
} from "../http";
import type { AppEnv } from "../../../shared/support";
import type { AppInstance, HttpContextHelpers } from "../http-context";
import { verifyRequestAuth } from "../../../application/auth/request-auth";

type RegisterLlmRoutesOptions = {
  app: AppInstance;
  caseStore: CaseStore | null;
  codexBridge: CodexBridge;
  env: AppEnv;
  fetchImpl: typeof fetch;
  helpers: HttpContextHelpers;
  logger: AppLogger;
  usageStore: UsageStore | null;
};

export function registerLlmRoutes({
  app,
  caseStore,
  codexBridge,
  env,
  fetchImpl,
  helpers,
  logger,
  usageStore,
}: RegisterLlmRoutesOptions) {
  const sanitizeCaseId = (value: unknown): string | null => {
    if (typeof value !== "string") return null;
    const normalized = value.trim();
    if (!normalized || normalized.length > 128) return null;
    return normalized;
  };

  app.get("/v1/llm/providers", async (context) => {
    return context.json(
      {
        providers: await buildLlmProviderCatalog(env, codexBridge, fetchImpl),
      },
      200,
      helpers.noStoreHeaders,
    );
  });

  app.get("/v1/llm/providers/:providerId/status", async (context) => {
    const providerId = context.req.param("providerId")?.trim() || "";
    if (providerId !== "codex") {
      return jsonError(400, "Unsupported provider.");
    }

    return context.json(await codexBridge.readStatus(), 200, helpers.noStoreHeaders);
  });

  app.post("/v1/llm/providers/:providerId/login", async (context) => {
    const providerId = context.req.param("providerId")?.trim() || "";
    if (providerId !== "codex") {
      return jsonError(400, "Unsupported provider.");
    }

    try {
      return context.json(await codexBridge.startLogin(), 200, helpers.noStoreHeaders);
    } catch (error) {
      return jsonError(502, error instanceof Error ? error.message : "Login failed.");
    }
  });

  app.get("/v1/llm/credentials/:providerId", async (context) => {
    const principal = await helpers.requireAppPrincipal(context);
    if (principal instanceof Response) return principal;

    const providerId = context.req.param("providerId")?.trim() || "";
    if (!supportsUserLlmCredentialProvider(providerId)) {
      return jsonError(400, "Unsupported provider.");
    }

    const credential = await caseStore!.getLlmCredential(principal.id, providerId);
    return context.json(
      {
        credential: {
          providerId,
          hasSecret: Boolean(credential),
          updatedAt: credential?.updatedAt ?? null,
        },
      },
      200,
      helpers.noStoreHeaders,
    );
  });

  app.put("/v1/llm/credentials", async (context) => {
    const requestId = helpers.requestIdOf(context);
    const principal = await helpers.requireAppPrincipal(context);
    if (principal instanceof Response) return principal;

    const parsedBody = await helpers.readJsonBody<unknown>(context);
    if (!parsedBody.ok) return parsedBody.response;
    const input = sanitizePrincipalLlmCredentialInput(parsedBody.value);
    if (!input) {
      return jsonError(400, "Invalid LLM credential payload.");
    }

    const credential = await caseStore!.saveLlmCredential(principal.id, input);
    logger.info("llm_credential.updated", {
      principal_id: principal.id,
      provider_id: credential.providerId,
      request_id: requestId,
    });

    return context.json({ credential }, 200, helpers.noStoreHeaders);
  });

  app.post("/v1/chat/completions", async (context) => {
    const requestId = helpers.requestIdOf(context);
    const authFailure = await verifyRequestAuth(context.req.raw, env, fetchImpl, logger, requestId);
    if (authFailure) return authFailure;
    const resolvedPrincipal = await helpers.resolveOptionalAppPrincipal(context);
    if (resolvedPrincipal instanceof Response) return resolvedPrincipal;
    const principal = resolvedPrincipal;

    const parsedBody = await helpers.readJsonBody<ChatCompletionsRequest>(context);
    if (!parsedBody.ok) {
      logger.warn("chat.request_invalid", {
        request_id: requestId,
        status: parsedBody.response.status,
      });
      return parsedBody.response;
    }

    const provider = parsedBody.value.provider?.trim() || "";
    const caseId = sanitizeCaseId(parsedBody.value.case_id);
    const requestedModel = parsedBody.value.model?.trim() || "";
    const model = requestedModel || "tidb";
    const startedAt = new Date().toISOString();
    const messageCount = Array.isArray(parsedBody.value.messages)
      ? parsedBody.value.messages.length
      : 0;
    const stream = Boolean(parsedBody.value.stream);
    const recordUsage = createUsageRecordCallback({
      usageStore,
      logger,
      baseRecord: {
        caseId: caseId ?? null,
        model,
        principalId: principal?.id ?? null,
        provider: provider || "tidb",
        requestId,
        route: "chat.completions",
        sessionId: null,
        startedAt,
        stream,
      },
    });
    logger.info("chat.request", {
      message_count: messageCount,
      model,
      provider: provider || undefined,
      request_id: requestId,
      stream,
    });

    if (provider) {
      if (!requestedModel) {
        return jsonError(400, "provider and model are required when provider routing is selected.");
      }

      if (provider === "codex") {
        return stream
          ? streamCodexProviderResponse(codexBridge, parsedBody.value, requestedModel, recordUsage)
          : completeCodexProviderResponse(codexBridge, parsedBody.value, requestedModel, recordUsage);
      }

      let providerApiKey: string | null = null;
      if (supportsUserLlmCredentialProvider(provider) && principal && caseStore) {
        providerApiKey = await caseStore.getLlmCredentialSecret(principal.id, provider);
      }

      const upstreamResponse = await fetchProviderUpstream(
        env,
        provider,
        parsedBody.value,
        fetchImpl,
        logger,
        requestId,
        {
          messageCount,
          model: requestedModel,
          stream,
        },
        {
          apiKeyOverride: providerApiKey,
          recordUsage,
        },
      );
      if (!upstreamResponse.ok) return upstreamResponse;

      if (stream) {
        return new Response(upstreamResponse.body, {
          headers: STREAM_RESPONSE_HEADERS,
          status: 200,
        });
      }

      const responseText = await upstreamResponse.text().catch(() => "");
      return new Response(responseText, {
        headers: JSON_RESPONSE_HEADERS,
        status: 200,
      });
    }

    const existingTidbChatId =
      caseId && principal && caseStore
        ? extractTidbChatIdFromHistory(await caseStore.getHistory(principal.id, caseId))
        : null;
    const persistTidbBinding =
      caseId && principal && caseStore
        ? async (chatId: string) => {
            await persistTidbChatBinding(caseStore, principal.id, caseId, chatId);
          }
        : undefined;

    const upstreamResponse = await fetchTidbUpstream(
      env,
      buildUpstreamRequestBody(parsedBody.value, existingTidbChatId),
      fetchImpl,
      logger,
      requestId,
      {
        messageCount,
        model,
        stream,
      },
    );
    if (!upstreamResponse.ok) return upstreamResponse;

    return stream
      ? createTidbStreamResponse(
          upstreamResponse,
          logger,
          requestId,
          model,
          recordUsage,
          persistTidbBinding,
        )
      : createTidbCompletionResponse(
          upstreamResponse,
          logger,
          requestId,
          model,
          recordUsage,
          persistTidbBinding,
        );
  });
}
