import { beforeEach, describe, expect, test, vi } from "vitest";

import { createMemoryCaseStore } from "./infrastructure/persistence/memory/memory-case-store";
import { createMemoryUsageStore } from "./infrastructure/persistence/memory/memory-usage-store";
import { createApp as rawCreateApp } from "./index";

function createMockCodexBridge(overrides: Record<string, unknown> = {}) {
  return {
    listModels: vi.fn(async () => [
      { id: "gpt-5.4", label: "gpt-5.4" },
      { id: "gpt-5.3-codex", label: "gpt-5.3-codex" },
    ]),
    readStatus: vi.fn(async () => ({
      available: true,
      loggedIn: true,
      needsLogin: false,
      message: "Codex OAuth is ready.",
      account: {
        email: "dev@example.com",
        planType: "pro",
      },
    })),
    startLogin: vi.fn(async () => ({
      authUrl: "https://chatgpt.com/auth/codex",
      loginId: "login-123",
    })),
    streamChat: vi.fn(async function* () {
      yield "Hello from Codex";
    }),
    ...overrides,
  };
}

function createMockLogger() {
  return {
    debug: vi.fn(),
    info: vi.fn(),
    warn: vi.fn(),
    error: vi.fn(),
  };
}

function createDefaultCodexBridge() {
  return createMockCodexBridge({
    listModels: vi.fn(async () => []),
    readStatus: vi.fn(async () => ({
      available: false,
      loggedIn: false,
      needsLogin: false,
      message: "Codex bridge unavailable.",
      account: null,
    })),
  });
}

function createApp(options: Parameters<typeof rawCreateApp>[0] = {}) {
  return rawCreateApp({
    codexBridge: createDefaultCodexBridge() as never,
    ...options,
  });
}

function jsonHeaders(extra: Record<string, string> = {}) {
  return {
    "Content-Type": "application/json",
    ...extra,
  };
}

function textStreamResponse(chunks: string[], status = 200) {
  const encoder = new TextEncoder();
  return new Response(
    new ReadableStream({
      start(controller) {
        for (const chunk of chunks) {
          controller.enqueue(encoder.encode(chunk));
        }
        controller.close();
      },
    }),
    {
      status,
      headers: {
        "Content-Type": "text/plain; charset=utf-8",
      },
    },
  );
}

function telemetryBody(overrides: Record<string, unknown> = {}) {
  return JSON.stringify({
    event: "tihc_ext_chat_submitted",
    params: {
      surface: "sidepanel",
    },
    context: {
      auth_state: "anonymous",
      case_id: "case-123",
      extension_version: "1.1.1",
      plugin_id: "tidb.ai",
      surface: "sidepanel",
    },
    identifiers: {
      client_id: "install-123",
      session_id: "1713110400",
    },
    debug: false,
    ...overrides,
  });
}

function caseRepositoryBody(overrides: Record<string, unknown> = {}) {
  return JSON.stringify({
    headId: "m-2",
    messages: [
      {
        parentId: null,
        message: {
          id: "m-1",
          role: "user",
          content: [{ type: "text", text: "Customer checkout fails after card entry." }],
          createdAt: "2026-04-14T11:59:00.000Z",
        },
      },
      {
        parentId: "m-1",
        message: {
          id: "m-2",
          role: "assistant",
          content: [{ type: "text", text: "Auth refresh latency appears after token exchange." }],
          createdAt: "2026-04-14T12:00:00.000Z",
        },
      },
    ],
    ...overrides,
  });
}

describe("createApp", () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  test("serves a health endpoint", async () => {
    const app = createApp();

    const response = await app.request("/health");

    expect(response.status).toBe(200);
    await expect(response.json()).resolves.toMatchObject({
      ok: true,
    });
  });

  test("adds a request id header and logs request lifecycle plus upstream config failures", async () => {
    const logger = createMockLogger();
    const app = createApp({
      logger,
    });

    const response = await app.request("/v1/chat/completions", {
      method: "POST",
      headers: jsonHeaders(),
      body: JSON.stringify({
        model: "tidb",
        messages: [{ role: "user", content: "hello" }],
        stream: false,
      }),
    });

    expect(response.status).toBe(500);
    expect(response.headers.get("x-request-id")).toBeTruthy();
    expect(logger.info).toHaveBeenCalledWith(
      "request.started",
      expect.objectContaining({
        method: "POST",
        path: "/v1/chat/completions",
        request_id: expect.any(String),
      }),
    );
    expect(logger.info).toHaveBeenCalledWith(
      "chat.request",
      expect.objectContaining({
        message_count: 1,
        model: "tidb",
        request_id: expect.any(String),
        stream: false,
      }),
    );
    expect(logger.error).toHaveBeenCalledWith(
      "upstream.config_missing",
      expect.objectContaining({
        request_id: expect.any(String),
        tidb_api_token_present: false,
        tidb_api_url_present: false,
      }),
    );
    expect(logger.error).toHaveBeenCalledWith(
      "request.completed",
      expect.objectContaining({
        method: "POST",
        path: "/v1/chat/completions",
        request_id: expect.any(String),
        status: 500,
      }),
    );
  });

  test("requires bearer auth for TiDB-backed case routes even when chat auth is optional", async () => {
    const app = createApp({
      env: {
        REQUIRE_AUTH: "false",
      },
      caseStore: createMemoryCaseStore(),
      fetchImpl: vi.fn(),
    });

    const response = await app.request("/v1/cases", {
      method: "GET",
      headers: jsonHeaders(),
    });

    expect(response.status).toBe(401);
    await expect(response.text()).resolves.toContain("Missing bearer token");
  });

  test("stores principal settings in isolation per authenticated user", async () => {
    const fetchImpl = vi.fn(async (input: RequestInfo | URL) => {
      const url = String(input);
      if (!url.includes("oauth2.googleapis.com/tokeninfo")) {
        throw new Error(`unexpected request: ${url}`);
      }

      if (url.includes("token-a")) {
        return Response.json({
          aud: "google-client-id",
          email: "alice@example.com",
          hd: "example.com",
          sub: "google-sub-alice",
        });
      }

      if (url.includes("token-b")) {
        return Response.json({
          aud: "google-client-id",
          email: "bob@example.com",
          hd: "example.com",
          sub: "google-sub-bob",
        });
      }

      return new Response("nope", { status: 401 });
    });
    const app = createApp({
      env: {
        DATABASE_URL: "mysql://user:pass@host/db",
        GOOGLE_CLIENT_ID: "google-client-id",
        GOOGLE_WORKSPACE_DOMAIN: "example.com",
      },
      caseStore: createMemoryCaseStore(),
      fetchImpl,
    });

    const saveResponse = await app.request("/v1/settings", {
      method: "PUT",
      headers: jsonHeaders({
        Authorization: "Bearer token-a",
      }),
      body: JSON.stringify({
        activeCaseId: "case-a",
        analyticsConsent: "granted",
        llmRuntime: {
          baseUrl: "https://runtime.example.com",
          providerId: "openai",
          model: "gpt-4.1-mini",
        },
        installedPlugins: [
          {
            pluginId: "tidb.ai",
            config: {
              baseUrl: "https://tidb.ai",
            },
          },
          {
            pluginId: "websearch",
            config: {
              enabled: false,
              mode: "off",
              primaryEngine: "bing",
            },
          },
        ],
      }),
    });

    expect(saveResponse.status).toBe(200);
    await expect(saveResponse.json()).resolves.toMatchObject({
      settings: {
        activeCaseId: "case-a",
        analyticsConsent: "granted",
        llmRuntime: {
          baseUrl: "https://runtime.example.com",
          providerId: "openai",
          model: "gpt-4.1-mini",
        },
        installedPlugins: [
          {
            pluginId: "tidb.ai",
            config: {
              baseUrl: "https://tidb.ai",
            },
          },
          {
            pluginId: "websearch",
            config: {
              enabled: false,
              mode: "off",
              primaryEngine: "bing",
            },
          },
        ],
      },
    });

    const readAlice = await app.request("/v1/settings", {
      method: "GET",
      headers: jsonHeaders({
        Authorization: "Bearer token-a",
      }),
    });
    const readBob = await app.request("/v1/settings", {
      method: "GET",
      headers: jsonHeaders({
        Authorization: "Bearer token-b",
      }),
    });

    expect(readAlice.status).toBe(200);
    await expect(readAlice.json()).resolves.toMatchObject({
      settings: {
        activeCaseId: "case-a",
        analyticsConsent: "granted",
        llmRuntime: {
          baseUrl: "https://runtime.example.com",
          providerId: "openai",
          model: "gpt-4.1-mini",
        },
      },
    });
    expect(readBob.status).toBe(200);
    await expect(readBob.json()).resolves.toMatchObject({
      settings: null,
    });
  });

  test("lists openai as a user-configurable provider when no backend key is configured", async () => {
    const app = createApp({
      env: {} as never,
    });

    const response = await app.request("/v1/llm/providers");

    expect(response.status).toBe(200);
    const payload = await response.json();
    expect(payload).toMatchObject({
      providers: expect.arrayContaining([
        expect.objectContaining({
          id: "openai",
          authMode: "user-api-key",
          configured: false,
        }),
      ]),
    });
  });

  test("lists anthropic and google as user-configurable providers when no backend keys are configured", async () => {
    const app = createApp({
      env: {} as never,
    });

    const response = await app.request("/v1/llm/providers");

    expect(response.status).toBe(200);
    await expect(response.json()).resolves.toMatchObject({
      providers: expect.arrayContaining([
        expect.objectContaining({
          id: "anthropic",
          authMode: "user-api-key",
          configured: false,
        }),
        expect.objectContaining({
          id: "google",
          authMode: "user-api-key",
          configured: false,
        }),
      ]),
    });
  });

  test("lists opencode-style hosted providers such as openrouter and xai from the shared registry", async () => {
    const app = createApp({
      env: {} as never,
    });

    const response = await app.request("/v1/llm/providers");

    expect(response.status).toBe(200);
    await expect(response.json()).resolves.toMatchObject({
      providers: expect.arrayContaining([
        expect.objectContaining({
          id: "openrouter",
          authMode: "user-api-key",
          configured: false,
        }),
        expect.objectContaining({
          id: "xai",
          authMode: "user-api-key",
          configured: false,
        }),
      ]),
    });
  });

  test("hydrates provider models from models.dev when remote metadata is available", async () => {
    const fetchImpl = vi.fn(async (input: RequestInfo | URL) => {
      const url = String(input);
      if (url !== "https://models.dev/api.json") {
        throw new Error(`unexpected request: ${url}`);
      }

      return Response.json({
        openai: {
          id: "openai",
          name: "OpenAI",
          env: ["OPENAI_API_KEY"],
          models: {
            "gpt-5-mini": {
              id: "gpt-5-mini",
              name: "GPT-5 Mini",
            },
            "text-embedding-3-large": {
              id: "text-embedding-3-large",
              name: "text-embedding-3-large",
              family: "text-embedding",
            },
          },
        },
      });
    });
    const app = createApp({
      env: {
        MODELS_DEV_DISABLE_FETCH: "0",
      } as never,
      fetchImpl,
    });

    const response = await app.request("/v1/llm/providers");

    expect(response.status).toBe(200);
    expect(fetchImpl).toHaveBeenCalledWith(
      "https://models.dev/api.json",
      expect.objectContaining({
        method: "GET",
      }),
    );
    const payload = await response.json();
    expect(payload).toMatchObject({
      providers: expect.arrayContaining([
        expect.objectContaining({
          id: "openai",
          models: expect.arrayContaining([
            {
              id: "gpt-5-mini",
              label: "GPT-5 Mini",
            },
          ]),
        }),
      ]),
    });
    const openaiProvider = payload.providers.find((provider: { id: string }) => provider.id === "openai");
    expect(openaiProvider?.models).not.toEqual(
      expect.arrayContaining([
        expect.objectContaining({
          id: "text-embedding-3-large",
        }),
      ]),
    );
  });

  test("lists codex as an oauth-backed provider when the codex bridge is available", async () => {
    const app = createApp({
      codexBridge: createMockCodexBridge() as never,
      env: {} as never,
    });

    const response = await app.request("/v1/llm/providers");

    expect(response.status).toBe(200);
    await expect(response.json()).resolves.toMatchObject({
      providers: expect.arrayContaining([
        expect.objectContaining({
          id: "codex",
          authMode: "codex-oauth",
          configured: true,
          defaultModel: "gpt-5.4",
        }),
      ]),
    });
  });

  test("stores user llm credentials per authenticated principal without returning raw secrets", async () => {
    const fetchImpl = vi.fn(async (input: RequestInfo | URL) => {
      const url = String(input);
      if (!url.includes("oauth2.googleapis.com/tokeninfo")) {
        throw new Error(`unexpected request: ${url}`);
      }

      if (url.includes("token-a")) {
        return Response.json({
          aud: "google-client-id",
          email: "alice@example.com",
          hd: "example.com",
          sub: "google-sub-alice",
        });
      }

      if (url.includes("token-b")) {
        return Response.json({
          aud: "google-client-id",
          email: "bob@example.com",
          hd: "example.com",
          sub: "google-sub-bob",
        });
      }

      return new Response("nope", { status: 401 });
    });
    const app = createApp({
      env: {
        DATABASE_URL: "mysql://user:pass@host/db",
        GOOGLE_CLIENT_ID: "google-client-id",
        GOOGLE_WORKSPACE_DOMAIN: "example.com",
      },
      caseStore: createMemoryCaseStore(),
      fetchImpl,
    });

    const saveResponse = await app.request("/v1/llm/credentials", {
      method: "PUT",
      headers: jsonHeaders({
        Authorization: "Bearer token-a",
      }),
      body: JSON.stringify({
        providerId: "openai",
        apiKey: "sk-user-openai-secret",
      }),
    });

    expect(saveResponse.status).toBe(200);
    const savedBody = await saveResponse.json();
    expect(savedBody).toMatchObject({
      credential: {
        providerId: "openai",
        hasSecret: true,
      },
    });
    expect(JSON.stringify(savedBody)).not.toContain("sk-user-openai-secret");

    const readAlice = await app.request("/v1/llm/credentials/openai", {
      method: "GET",
      headers: jsonHeaders({
        Authorization: "Bearer token-a",
      }),
    });
    const readBob = await app.request("/v1/llm/credentials/openai", {
      method: "GET",
      headers: jsonHeaders({
        Authorization: "Bearer token-b",
      }),
    });

    expect(readAlice.status).toBe(200);
    expect(await readAlice.json()).toMatchObject({
      credential: {
        providerId: "openai",
        hasSecret: true,
      },
    });
    expect(readBob.status).toBe(200);
    expect(await readBob.json()).toMatchObject({
      credential: {
        providerId: "openai",
        hasSecret: false,
      },
    });
  });

  test("stores and reads anthropic credentials for the authenticated principal", async () => {
    const fetchImpl = vi.fn(async (input: RequestInfo | URL) => {
      const url = String(input);
      if (!url.includes("oauth2.googleapis.com/tokeninfo")) {
        throw new Error(`unexpected request: ${url}`);
      }

      return Response.json({
        aud: "google-client-id",
        email: "alice@example.com",
        hd: "example.com",
        sub: "google-sub-alice",
      });
    });
    const app = createApp({
      env: {
        DATABASE_URL: "mysql://user:pass@host/db",
        GOOGLE_CLIENT_ID: "google-client-id",
        GOOGLE_WORKSPACE_DOMAIN: "example.com",
      },
      caseStore: createMemoryCaseStore(),
      fetchImpl,
    });

    const saveResponse = await app.request("/v1/llm/credentials", {
      method: "PUT",
      headers: jsonHeaders({
        Authorization: "Bearer token-a",
      }),
      body: JSON.stringify({
        providerId: "anthropic",
        apiKey: "sk-ant-user-secret",
      }),
    });

    expect(saveResponse.status).toBe(200);
    await expect(saveResponse.json()).resolves.toMatchObject({
      credential: {
        providerId: "anthropic",
        hasSecret: true,
        updatedAt: expect.any(String),
      },
    });

    const readResponse = await app.request("/v1/llm/credentials/anthropic", {
      method: "GET",
      headers: jsonHeaders({
        Authorization: "Bearer token-a",
      }),
    });

    expect(readResponse.status).toBe(200);
    await expect(readResponse.json()).resolves.toMatchObject({
      credential: {
        providerId: "anthropic",
        hasSecret: true,
        updatedAt: expect.any(String),
      },
    });
  });

  test("imports cases once per principal plus client id and enforces cross-user isolation", async () => {
    const fetchImpl = vi.fn(async (input: RequestInfo | URL) => {
      const url = String(input);
      if (!url.includes("oauth2.googleapis.com/tokeninfo")) {
        throw new Error(`unexpected request: ${url}`);
      }

      if (url.includes("token-a")) {
        return Response.json({
          aud: "google-client-id",
          email: "alice@example.com",
          hd: "example.com",
          sub: "google-sub-alice",
        });
      }

      if (url.includes("token-b")) {
        return Response.json({
          aud: "google-client-id",
          email: "bob@example.com",
          hd: "example.com",
          sub: "google-sub-bob",
        });
      }

      return new Response("nope", { status: 401 });
    });
    const app = createApp({
      env: {
        DATABASE_URL: "mysql://user:pass@host/db",
        GOOGLE_CLIENT_ID: "google-client-id",
        GOOGLE_WORKSPACE_DOMAIN: "example.com",
      },
      caseStore: createMemoryCaseStore(),
      fetchImpl,
    });

    const importResponse = await app.request("/v1/cases/import", {
      method: "POST",
      headers: jsonHeaders({
        Authorization: "Bearer token-a",
      }),
      body: JSON.stringify({
        clientId: "client-123",
        cases: [
          {
            id: "case-1",
            title: "Checkout timeout",
            pluginId: "tidb.ai",
            activityState: "active",
            resolvedAt: null,
            archivedAt: null,
            createdAt: "2026-04-14T09:30:00.000Z",
            updatedAt: "2026-04-14T12:00:00.000Z",
          },
        ],
        historiesByCaseId: {
          "case-1": JSON.parse(caseRepositoryBody()),
        },
      }),
    });

    expect(importResponse.status).toBe(200);
    await expect(importResponse.json()).resolves.toMatchObject({
      alreadyImported: false,
      importedCases: 1,
    });

    const secondImportResponse = await app.request("/v1/cases/import", {
      method: "POST",
      headers: jsonHeaders({
        Authorization: "Bearer token-a",
      }),
      body: JSON.stringify({
        clientId: "client-123",
        cases: [
          {
            id: "case-1",
            title: "Checkout timeout",
            pluginId: "tidb.ai",
            activityState: "active",
            resolvedAt: null,
            archivedAt: null,
            createdAt: "2026-04-14T09:30:00.000Z",
            updatedAt: "2026-04-14T12:00:00.000Z",
          },
        ],
        historiesByCaseId: {
          "case-1": JSON.parse(caseRepositoryBody()),
        },
      }),
    });

    expect(secondImportResponse.status).toBe(200);
    await expect(secondImportResponse.json()).resolves.toMatchObject({
      alreadyImported: true,
      importedCases: 0,
    });

    const ownerListResponse = await app.request("/v1/cases", {
      method: "GET",
      headers: jsonHeaders({
        Authorization: "Bearer token-a",
      }),
    });

    expect(ownerListResponse.status).toBe(200);
    await expect(ownerListResponse.json()).resolves.toMatchObject({
      cases: [
        {
          id: "case-1",
          title: "Checkout timeout",
          activityState: "active",
          summary: "Auth refresh latency appears after token exchange.",
          signals: expect.arrayContaining([
            "Auth refresh latency appears after token exchange.",
          ]),
          messagesPreview: [
            {
              role: "operator",
              text: "Customer checkout fails after card entry.",
            },
            {
              role: "tihc",
              text: "Auth refresh latency appears after token exchange.",
            },
          ],
        },
      ],
    });

    const ownerHistoryResponse = await app.request("/v1/cases/case-1/history", {
      method: "GET",
      headers: jsonHeaders({
        Authorization: "Bearer token-a",
      }),
    });

    expect(ownerHistoryResponse.status).toBe(200);
    await expect(ownerHistoryResponse.json()).resolves.toMatchObject({
      repository: {
        headId: "m-2",
      },
    });

    const otherUserListResponse = await app.request("/v1/cases", {
      method: "GET",
      headers: jsonHeaders({
        Authorization: "Bearer token-b",
      }),
    });

    expect(otherUserListResponse.status).toBe(200);
    await expect(otherUserListResponse.json()).resolves.toMatchObject({
      cases: [],
    });

    const otherUserHistoryResponse = await app.request("/v1/cases/case-1/history", {
      method: "GET",
      headers: jsonHeaders({
        Authorization: "Bearer token-b",
      }),
    });

    expect(otherUserHistoryResponse.status).toBe(404);
  });

  test("updates case history and lifecycle only for the owning principal", async () => {
    const fetchImpl = vi.fn(async (input: RequestInfo | URL) => {
      const url = String(input);
      if (!url.includes("oauth2.googleapis.com/tokeninfo")) {
        throw new Error(`unexpected request: ${url}`);
      }

      if (url.includes("owner-token")) {
        return Response.json({
          aud: "google-client-id",
          email: "owner@example.com",
          hd: "example.com",
          sub: "google-sub-owner",
        });
      }

      if (url.includes("other-token")) {
        return Response.json({
          aud: "google-client-id",
          email: "other@example.com",
          hd: "example.com",
          sub: "google-sub-other",
        });
      }

      return new Response("nope", { status: 401 });
    });
    const app = createApp({
      env: {
        DATABASE_URL: "mysql://user:pass@host/db",
        GOOGLE_CLIENT_ID: "google-client-id",
        GOOGLE_WORKSPACE_DOMAIN: "example.com",
      },
      caseStore: createMemoryCaseStore(),
      fetchImpl,
    });

    const createResponse = await app.request("/v1/cases", {
      method: "POST",
      headers: jsonHeaders({
        Authorization: "Bearer owner-token",
      }),
      body: JSON.stringify({
        id: "case-owned",
        title: "Investigate payment retries",
        activityState: "ready",
        resolvedAt: null,
        archivedAt: null,
        createdAt: "2026-04-14T12:00:00.000Z",
        updatedAt: "2026-04-14T12:00:00.000Z",
      }),
    });

    expect(createResponse.status).toBe(201);

    const updateHistoryResponse = await app.request("/v1/cases/case-owned/history", {
      method: "PUT",
      headers: jsonHeaders({
        Authorization: "Bearer owner-token",
      }),
      body: JSON.stringify({
        repository: JSON.parse(caseRepositoryBody({
          headId: "m-3",
          messages: [
            {
              parentId: null,
              message: {
                id: "m-1",
                role: "user",
                content: [{ type: "text", text: "Payment retries spike after deploy." }],
                createdAt: "2026-04-14T11:58:00.000Z",
              },
            },
            {
              parentId: "m-1",
              message: {
                id: "m-2",
                role: "assistant",
                content: [{ type: "text", text: "Stripe webhook lag increased to 9 seconds." }],
                createdAt: "2026-04-14T11:59:00.000Z",
              },
            },
            {
              parentId: "m-2",
              message: {
                id: "m-3",
                role: "assistant",
                content: [{ type: "text", text: "Retry queue backlog is now the dominant signal." }],
                createdAt: "2026-04-14T12:00:00.000Z",
              },
            },
          ],
        })),
      }),
    });

    expect(updateHistoryResponse.status).toBe(200);

    const patchByOtherUser = await app.request("/v1/cases/case-owned", {
      method: "PATCH",
      headers: jsonHeaders({
        Authorization: "Bearer other-token",
      }),
      body: JSON.stringify({
        title: "Hacked title",
      }),
    });

    expect(patchByOtherUser.status).toBe(404);

    const patchByOwner = await app.request("/v1/cases/case-owned", {
      method: "PATCH",
      headers: jsonHeaders({
        Authorization: "Bearer owner-token",
      }),
      body: JSON.stringify({
        title: "Investigate payment retry backlog",
        activityState: "resolved",
        resolvedAt: "2026-04-14T12:30:00.000Z",
      }),
    });

    expect(patchByOwner.status).toBe(200);
    await expect(patchByOwner.json()).resolves.toMatchObject({
      case: {
        id: "case-owned",
        title: "Investigate payment retry backlog",
        activityState: "resolved",
        summary: "Retry queue backlog is now the dominant signal.",
      },
    });

    const deleteByOtherUser = await app.request("/v1/cases/case-owned", {
      method: "DELETE",
      headers: jsonHeaders({
        Authorization: "Bearer other-token",
      }),
    });

    expect(deleteByOtherUser.status).toBe(404);

    const deleteByOwner = await app.request("/v1/cases/case-owned", {
      method: "DELETE",
      headers: jsonHeaders({
        Authorization: "Bearer owner-token",
      }),
    });

    expect(deleteByOwner.status).toBe(204);

    const listAfterDelete = await app.request("/v1/cases", {
      method: "GET",
      headers: jsonHeaders({
        Authorization: "Bearer owner-token",
      }),
    });

    await expect(listAfterDelete.json()).resolves.toMatchObject({
      cases: [],
    });
  });

  test("streams OpenAI-compatible SSE from upstream protocol frames", async () => {
    const fetchImpl = vi.fn(async (input: RequestInfo | URL) => {
      if (String(input).includes("oauth2.googleapis.com/tokeninfo")) {
        throw new Error("tokeninfo should not be called when auth is disabled");
      }

      return textStreamResponse([
        '8:[{"display":"kb-main","state":"RUNNING","message":"warming cache"}]\n',
        '0:{"delta":"Hello"}\n',
        '2:[{"assistant_message":{"content":"Hello world","finished_at":"2026-04-14T12:00:00Z"}}]\n',
      ]);
    });

    const app = createApp({
      env: {
        TIDB_API_TOKEN: "upstream-secret",
        TIDB_API_URL: "https://tidb.example.com/chat",
      },
      fetchImpl,
    });

    const response = await app.request("/v1/chat/completions", {
      method: "POST",
      headers: jsonHeaders(),
      body: JSON.stringify({
        model: "tidb",
        messages: [{ role: "user", content: "hello" }],
        stream: true,
      }),
    });

    expect(response.status).toBe(200);
    expect(response.headers.get("content-type")).toContain("text/event-stream");

    const payload = await response.text();
    expect(payload).toContain('"delta":{"content":"Retrieving: kb-main\\n"}');
    expect(payload).toContain('"delta":{"content":"Retrieval state: RUNNING\\n"}');
    expect(payload).toContain('"delta":{"content":"Retrieval message: warming cache\\n"}');
    expect(payload).toContain('"delta":{"content":"Hello"}');
    expect(payload).toContain('"delta":{"content":" world"}');
    expect(payload.trim().endsWith("data: [DONE]")).toBe(true);
    expect(fetchImpl).toHaveBeenCalledWith(
      "https://tidb.example.com/chat",
      expect.objectContaining({
        method: "POST",
      }),
    );
  });

  test("lists configured llm providers from the backend catalog", async () => {
    const app = createApp({
      env: {
        OPENAI_API_KEY: "openai-key",
        ANTHROPIC_API_KEY: "anthropic-key",
      } as never,
      codexBridge: createMockCodexBridge() as never,
    });

    const response = await app.request("/v1/llm/providers");

    expect(response.status).toBe(200);
    await expect(response.json()).resolves.toMatchObject({
      providers: expect.arrayContaining([
        expect.objectContaining({
          id: "openai",
          configured: true,
          models: expect.arrayContaining([
            expect.objectContaining({
              id: expect.any(String),
              label: expect.any(String),
            }),
          ]),
        }),
        expect.objectContaining({
          id: "anthropic",
          configured: true,
        }),
        expect.objectContaining({
          id: "codex",
          authMode: "codex-oauth",
          configured: true,
        }),
      ]),
    });
  });

  test("reports codex oauth status through the provider status endpoint", async () => {
    const codexBridge = createMockCodexBridge();
    const app = createApp({
      codexBridge: codexBridge as never,
      env: {} as never,
      fetchImpl: vi.fn(),
    });

    const response = await app.request("/v1/llm/providers/codex/status");

    expect(response.status).toBe(200);
    await expect(response.json()).resolves.toMatchObject({
      available: true,
      loggedIn: true,
      needsLogin: false,
      account: {
        email: "dev@example.com",
        planType: "pro",
      },
    });
    expect(codexBridge.readStatus).toHaveBeenCalledTimes(1);
  });

  test("starts codex oauth through the provider login endpoint", async () => {
    const codexBridge = createMockCodexBridge();
    const app = createApp({
      codexBridge: codexBridge as never,
      env: {} as never,
      fetchImpl: vi.fn(),
    });

    const response = await app.request("/v1/llm/providers/codex/login", {
      method: "POST",
      headers: jsonHeaders(),
    });

    expect(response.status).toBe(200);
    await expect(response.json()).resolves.toMatchObject({
      authUrl: "https://chatgpt.com/auth/codex",
      loginId: "login-123",
    });
    expect(codexBridge.startLogin).toHaveBeenCalledTimes(1);
  });

  test("rejects provider-routed requests that do not include a provider and model", async () => {
    const app = createApp({
      env: {
        OPENAI_API_KEY: "openai-key",
      } as never,
      fetchImpl: vi.fn(),
    });

    const response = await app.request("/v1/chat/completions", {
      method: "POST",
      headers: jsonHeaders(),
      body: JSON.stringify({
        provider: "openai",
        messages: [{ role: "user", content: "hello" }],
        stream: true,
      }),
    });

    expect(response.status).toBe(400);
    await expect(response.json()).resolves.toMatchObject({
      error: {
        message: "provider and model are required when provider routing is selected.",
      },
    });
  });

  test("routes provider-selected requests through the selected openai provider", async () => {
    const fetchImpl = vi.fn(async (_input: RequestInfo | URL, init?: RequestInit) => {
      const body = JSON.parse(String(init?.body ?? "{}"));
      expect(body).toMatchObject({
        model: "gpt-4.1-mini",
        messages: [{ role: "user", content: "hello" }],
        stream: true,
      });

      return textStreamResponse([
        'data: {"choices":[{"delta":{"content":"Hello from OpenAI"},"finish_reason":null,"index":0}]}\n\n',
        "data: [DONE]\n\n",
      ]);
    });

    const app = createApp({
      env: {
        OPENAI_API_KEY: "openai-key",
      } as never,
      fetchImpl,
    });

    const response = await app.request("/v1/chat/completions", {
      method: "POST",
      headers: jsonHeaders(),
      body: JSON.stringify({
        provider: "openai",
        model: "gpt-4.1-mini",
        messages: [{ role: "user", content: "hello" }],
        stream: true,
      }),
    });

    expect(response.status).toBe(200);
    await expect(response.text()).resolves.toContain('"delta":{"content":"Hello from OpenAI"}');
    expect(fetchImpl).toHaveBeenCalledWith(
      "https://api.openai.com/v1/chat/completions",
      expect.objectContaining({
        headers: expect.objectContaining({
          authorization: "Bearer openai-key",
        }),
        method: "POST",
      }),
    );
  });

  test("routes non-stream anthropic provider requests through the unified provider layer", async () => {
    const fetchImpl = vi.fn(async (input: RequestInfo | URL) => {
      expect(String(input)).toContain("https://api.anthropic.com");
      return Response.json({
        id: "msg_123",
        type: "message",
        role: "assistant",
        model: "claude-3-5-sonnet-latest",
        stop_reason: "end_turn",
        stop_sequence: null,
        usage: {
          input_tokens: 12,
          output_tokens: 4,
        },
        content: [
          {
            type: "text",
            text: "Hello from Anthropic",
          },
        ],
      });
    });

    const app = createApp({
      env: {
        ANTHROPIC_API_KEY: "anthropic-key",
      } as never,
      fetchImpl,
    });

    const response = await app.request("/v1/chat/completions", {
      method: "POST",
      headers: jsonHeaders(),
      body: JSON.stringify({
        provider: "anthropic",
        model: "claude-3-5-sonnet-latest",
        messages: [{ role: "user", content: "hello" }],
        stream: false,
      }),
    });

    expect(response.status).toBe(200);
    await expect(response.json()).resolves.toMatchObject({
      choices: [
        {
          finish_reason: "stop",
          message: {
            role: "assistant",
            content: "Hello from Anthropic",
          },
        },
      ],
      model: "claude-3-5-sonnet-latest",
      object: "chat.completion",
    });
  });

  test("records provider usage events and exposes summary plus timeseries for the authenticated principal", async () => {
    const fetchImpl = vi.fn(async (input: RequestInfo | URL) => {
      const url = String(input);

      if (url.includes("oauth2.googleapis.com/tokeninfo")) {
        return Response.json({
          aud: "google-client-id",
          email: "alice@example.com",
          hd: "example.com",
          sub: "google-sub-alice",
        });
      }

      if (url.includes("api.anthropic.com")) {
        return Response.json({
          id: "msg_123",
          type: "message",
          role: "assistant",
          model: "claude-3-5-sonnet-latest",
          stop_reason: "end_turn",
          usage: {
            input_tokens: 12,
            output_tokens: 4,
          },
          content: [
            {
              type: "text",
              text: "Hello from Anthropic",
            },
          ],
        });
      }

      throw new Error(`unexpected request: ${url}`);
    });
    const app = createApp({
      env: {
        DATABASE_URL: "mysql://user:pass@host/db",
        GOOGLE_CLIENT_ID: "google-client-id",
        GOOGLE_WORKSPACE_DOMAIN: "example.com",
        ANTHROPIC_API_KEY: "anthropic-key",
      } as never,
      caseStore: createMemoryCaseStore(),
      usageStore: createMemoryUsageStore(),
      fetchImpl,
    });

    const chatResponse = await app.request("/v1/chat/completions", {
      method: "POST",
      headers: jsonHeaders({
        Authorization: "Bearer token-a",
      }),
      body: JSON.stringify({
        provider: "anthropic",
        model: "claude-3-5-sonnet-latest",
        messages: [{ role: "user", content: "hello" }],
        stream: false,
      }),
    });

    expect(chatResponse.status).toBe(200);

    const summaryResponse = await app.request("/v1/usage/summary?days=30", {
      method: "GET",
      headers: jsonHeaders({
        Authorization: "Bearer token-a",
      }),
    });

    expect(summaryResponse.status).toBe(200);
    await expect(summaryResponse.json()).resolves.toMatchObject({
      summary: {
        windowDays: 30,
        current: {
          requestCount: 1,
          inputTokens: 12,
          outputTokens: 4,
          totalTokens: 16,
          reasoningTokens: 0,
          cachedInputTokens: 0,
          costUsd: 0,
        },
        previous: {
          requestCount: 0,
          totalTokens: 0,
        },
      },
    });

    const timeseriesResponse = await app.request("/v1/usage/timeseries?days=30", {
      method: "GET",
      headers: jsonHeaders({
        Authorization: "Bearer token-a",
      }),
    });

    expect(timeseriesResponse.status).toBe(200);
    await expect(timeseriesResponse.json()).resolves.toMatchObject({
      points: expect.arrayContaining([
        expect.objectContaining({
          inputTokens: 12,
          outputTokens: 4,
          totalTokens: 16,
          requestCount: 1,
        }),
      ]),
    });
  });

  test("routes openrouter provider requests through the shared provider registry", async () => {
    const fetchImpl = vi.fn(async (input: RequestInfo | URL, init?: RequestInit) => {
      expect(String(input)).toContain("openrouter.ai");
      expect(init).toMatchObject({
        headers: expect.objectContaining({
          authorization: "Bearer openrouter-key",
        }),
        method: "POST",
      });

      return textStreamResponse([
        'data: {"choices":[{"delta":{"content":"Hello from OpenRouter"},"finish_reason":null,"index":0}]}\n\n',
        "data: [DONE]\n\n",
      ]);
    });

    const app = createApp({
      env: {
        OPENROUTER_API_KEY: "openrouter-key",
      } as never,
      fetchImpl,
    });

    const response = await app.request("/v1/chat/completions", {
      method: "POST",
      headers: jsonHeaders(),
      body: JSON.stringify({
        provider: "openrouter",
        model: "openai/gpt-4.1-mini",
        messages: [{ role: "user", content: "hello" }],
        stream: true,
      }),
    });

    expect(response.status).toBe(200);
    await expect(response.text()).resolves.toContain('"delta":{"content":"Hello from OpenRouter"}');
  });

  test("routes provider-selected requests through the authenticated user's stored openai credential", async () => {
    const fetchImpl = vi.fn(async (input: RequestInfo | URL, init?: RequestInit) => {
      const url = String(input);

      if (url.includes("oauth2.googleapis.com/tokeninfo")) {
        return Response.json({
          aud: "google-client-id",
          email: "alice@example.com",
          hd: "example.com",
          sub: "google-sub-alice",
        });
      }

      if (url === "https://api.openai.com/v1/chat/completions") {
        const body = JSON.parse(String(init?.body ?? "{}"));
        expect(body).toMatchObject({
          model: "gpt-4.1-mini",
          messages: [{ role: "user", content: "hello" }],
          stream: true,
        });

        expect(init).toMatchObject({
          headers: expect.objectContaining({
            authorization: "Bearer sk-user-openai-secret",
          }),
          method: "POST",
        });

        return textStreamResponse([
          'data: {"choices":[{"delta":{"content":"Hello from user secret"},"finish_reason":null,"index":0}]}\n\n',
          "data: [DONE]\n\n",
        ]);
      }

      throw new Error(`unexpected request: ${url}`);
    });
    const app = createApp({
      env: {
        DATABASE_URL: "mysql://user:pass@host/db",
        GOOGLE_CLIENT_ID: "google-client-id",
        GOOGLE_WORKSPACE_DOMAIN: "example.com",
      },
      caseStore: createMemoryCaseStore(),
      fetchImpl,
    });

    const saveCredentialResponse = await app.request("/v1/llm/credentials", {
      method: "PUT",
      headers: jsonHeaders({
        Authorization: "Bearer token-a",
      }),
      body: JSON.stringify({
        providerId: "openai",
        apiKey: "sk-user-openai-secret",
      }),
    });
    expect(saveCredentialResponse.status).toBe(200);

    const response = await app.request("/v1/chat/completions", {
      method: "POST",
      headers: jsonHeaders({
        Authorization: "Bearer token-a",
      }),
      body: JSON.stringify({
        provider: "openai",
        model: "gpt-4.1-mini",
        messages: [{ role: "user", content: "hello" }],
        stream: true,
      }),
    });

    expect(response.status).toBe(200);
    await expect(response.text()).resolves.toContain('"delta":{"content":"Hello from user secret"}');
  });

  test("routes provider-selected requests through the codex bridge", async () => {
    const codexBridge = createMockCodexBridge({
      streamChat: vi.fn(async function* () {
        yield "Hello";
        yield " from Codex";
      }),
    });
    const app = createApp({
      codexBridge: codexBridge as never,
      env: {} as never,
      fetchImpl: vi.fn(),
    });

    const response = await app.request("/v1/chat/completions", {
      method: "POST",
      headers: jsonHeaders(),
      body: JSON.stringify({
        provider: "codex",
        model: "gpt-5.4",
        messages: [{ role: "user", content: "hello" }],
        stream: true,
      }),
    });

    expect(response.status).toBe(200);
    const payload = await response.text();
    expect(payload).toContain('"delta":{"content":"Hello"}');
    expect(payload).toContain('"delta":{"content":" from Codex"}');
    expect(codexBridge.streamChat).toHaveBeenCalledWith(
      expect.objectContaining({
        model: "gpt-5.4",
        messages: [{ role: "user", content: "hello" }],
      }),
    );
  });

  test("returns a non-stream OpenAI-compatible completion response", async () => {
    const fetchImpl = vi.fn(async () =>
      textStreamResponse([
        '8:[{"display":"kb-main","state":"RUNNING"}]\n',
        '0:{"delta":"Hello"}\n',
        '2:[{"assistant_message":{"content":"Hello world","finished_at":"2026-04-14T12:00:00Z"}}]\n',
      ]),
    );

    const app = createApp({
      env: {
        TIDB_API_TOKEN: "upstream-secret",
        TIDB_API_URL: "https://tidb.example.com/chat",
      },
      fetchImpl,
    });

    const response = await app.request("/v1/chat/completions", {
      method: "POST",
      headers: jsonHeaders(),
      body: JSON.stringify({
        model: "tidb",
        messages: [{ role: "user", content: "hello" }],
        stream: false,
      }),
    });

    expect(response.status).toBe(200);
    await expect(response.json()).resolves.toMatchObject({
      choices: [
        {
          finish_reason: "stop",
          message: {
            content:
              "Retrieving: kb-main\nRetrieval state: RUNNING\nHello world",
            role: "assistant",
          },
        },
      ],
      object: "chat.completion",
    });
  });

  test("rejects missing bearer token when auth is required", async () => {
    const app = createApp({
      env: {
        REQUIRE_AUTH: "true",
        TIDB_API_TOKEN: "upstream-secret",
        TIDB_API_URL: "https://tidb.example.com/chat",
      },
      fetchImpl: vi.fn(),
    });

    const response = await app.request("/v1/chat/completions", {
      method: "POST",
      headers: jsonHeaders(),
      body: JSON.stringify({
        model: "tidb",
        messages: [{ role: "user", content: "hello" }],
        stream: true,
      }),
    });

    expect(response.status).toBe(401);
    await expect(response.text()).resolves.toContain("Missing bearer token");
  });

  test("rejects an invalid Google token when auth is required", async () => {
    const fetchImpl = vi.fn(async (input: RequestInfo | URL) => {
      if (String(input).includes("oauth2.googleapis.com/tokeninfo")) {
        return new Response("nope", { status: 401 });
      }
      throw new Error("upstream request should not be reached");
    });

    const app = createApp({
      env: {
        REQUIRE_AUTH: "true",
        TIDB_API_TOKEN: "upstream-secret",
        TIDB_API_URL: "https://tidb.example.com/chat",
      },
      fetchImpl,
    });

    const response = await app.request("/v1/chat/completions", {
      method: "POST",
      headers: jsonHeaders({
        Authorization: "Bearer invalid-token",
      }),
      body: JSON.stringify({
        model: "tidb",
        messages: [{ role: "user", content: "hello" }],
        stream: true,
      }),
    });

    expect(response.status).toBe(401);
    await expect(response.text()).resolves.toContain("Unauthorized");
  });

  test("allows a valid Google token through the auth middleware", async () => {
    const fetchImpl = vi.fn(async (input: RequestInfo | URL) => {
      if (String(input).includes("oauth2.googleapis.com/tokeninfo")) {
        return Response.json({
          aud: "google-client-id",
          email: "dev@example.com",
          hd: "example.com",
        });
      }

      return textStreamResponse(['0:{"delta":"Hello"}\n']);
    });

    const app = createApp({
      env: {
        GOOGLE_CLIENT_ID: "google-client-id",
        GOOGLE_WORKSPACE_DOMAIN: "example.com",
        REQUIRE_AUTH: "true",
        TIDB_API_TOKEN: "upstream-secret",
        TIDB_API_URL: "https://tidb.example.com/chat",
      },
      fetchImpl,
    });

    const response = await app.request("/v1/chat/completions", {
      method: "POST",
      headers: jsonHeaders({
        Authorization: "Bearer valid-token",
      }),
      body: JSON.stringify({
        model: "tidb",
        messages: [{ role: "user", content: "hello" }],
        stream: true,
      }),
    });

    expect(response.status).toBe(200);
    await expect(response.text()).resolves.toContain('"delta":{"content":"Hello"}');
  });

  test("returns 204 and skips GA4 forwarding when telemetry is disabled", async () => {
    const fetchImpl = vi.fn(async (_input: RequestInfo | URL, _init?: RequestInit) =>
      new Response(null, { status: 204 }),
    );
    const app = createApp({
      env: {},
      fetchImpl,
    });

    const response = await app.request("/v1/telemetry", {
      method: "POST",
      headers: jsonHeaders(),
      body: telemetryBody(),
    });

    expect(response.status).toBe(204);
    expect(fetchImpl).not.toHaveBeenCalled();
  });

  test("rejects telemetry events outside the allowlist", async () => {
    const fetchImpl = vi.fn(async (_input: RequestInfo | URL, _init?: RequestInit) =>
      new Response(null, { status: 204 }),
    );
    const app = createApp({
      env: {
        GA4_API_SECRET: "secret",
        GA4_ENABLED: "true",
        GA4_MEASUREMENT_ID: "G-TEST123",
      },
      fetchImpl,
    });

    const response = await app.request("/v1/telemetry", {
      method: "POST",
      headers: jsonHeaders(),
      body: telemetryBody({
        event: "not_allowed",
      }),
    });

    expect(response.status).toBe(400);
    await expect(response.json()).resolves.toMatchObject({
      error: {
        message: expect.stringContaining("Invalid telemetry event"),
      },
    });
    expect(fetchImpl).not.toHaveBeenCalled();
  });

  test("forwards anonymous telemetry to the GA4 measurement protocol", async () => {
    const fetchImpl = vi.fn(async (_input: RequestInfo | URL, _init?: RequestInit) =>
      new Response(null, { status: 204 }),
    );
    const app = createApp({
      env: {
        GA4_API_SECRET: "secret",
        GA4_ENABLED: "true",
        GA4_MEASUREMENT_ID: "G-TEST123",
      },
      fetchImpl,
    });

    const response = await app.request("/v1/telemetry", {
      method: "POST",
      headers: jsonHeaders(),
      body: telemetryBody(),
    });

    expect(response.status).toBe(204);
    expect(fetchImpl).toHaveBeenCalledWith(
      expect.stringContaining("/mp/collect?measurement_id=G-TEST123&api_secret=secret"),
      expect.objectContaining({
        method: "POST",
      }),
    );

    const [, requestInit] = fetchImpl.mock.calls[0] ?? [];
    expect(JSON.parse(String(requestInit?.body))).toMatchObject({
      client_id: "install-123",
      events: [
        {
          name: "tihc_ext_chat_submitted",
          params: expect.objectContaining({
            auth_state: "anonymous",
            case_id: "case-123",
            extension_version: "1.1.1",
            plugin_id: "tidb.ai",
            session_id: 1713110400,
            surface: "sidepanel",
          }),
        },
      ],
    });
    expect(JSON.parse(String(requestInit?.body)).user_id).toBeUndefined();
  });

  test("adds a stable hashed user_id for telemetry when a valid Google bearer token is present", async () => {
    const fetchImpl = vi.fn(async (input: RequestInfo | URL, _init?: RequestInit) => {
      if (String(input).includes("oauth2.googleapis.com/tokeninfo")) {
        return Response.json({
          aud: "google-client-id",
          email: "dev@example.com",
          hd: "example.com",
          sub: "google-subject-id",
        });
      }

      return new Response(null, { status: 204 });
    });
    const app = createApp({
      env: {
        GA4_API_SECRET: "secret",
        GA4_ENABLED: "true",
        GA4_MEASUREMENT_ID: "G-TEST123",
        GA4_USER_ID_SALT: "salted",
        GOOGLE_CLIENT_ID: "google-client-id",
        GOOGLE_WORKSPACE_DOMAIN: "example.com",
      },
      fetchImpl,
    });

    const response = await app.request("/v1/telemetry", {
      method: "POST",
      headers: jsonHeaders({
        Authorization: "Bearer valid-token",
      }),
      body: telemetryBody({
        context: {
          auth_state: "authenticated",
          case_id: "case-123",
          extension_version: "1.1.1",
          plugin_id: "tidb.ai",
          surface: "sidepanel",
        },
      }),
    });

    expect(response.status).toBe(204);
    expect(fetchImpl).toHaveBeenCalledTimes(2);

    const [, requestInit] = fetchImpl.mock.calls[1] ?? [];
    const payload = JSON.parse(String(requestInit?.body));
    expect(payload.client_id).toBe("install-123");
    expect(payload.user_id).toBeDefined();
    expect(payload.user_id).not.toBe("google-subject-id");
    expect(payload.user_id).toMatch(/^[a-f0-9]{64}$/);
  });

  test("forwards allowlisted non-chat telemetry events and drops unknown telemetry fields", async () => {
    const fetchImpl = vi.fn(async (_input: RequestInfo | URL, _init?: RequestInit) =>
      new Response(null, { status: 204 }),
    );
    const app = createApp({
      env: {
        GA4_API_SECRET: "secret",
        GA4_ENABLED: "true",
        GA4_MEASUREMENT_ID: "G-TEST123",
      },
      fetchImpl,
    });

    const response = await app.request("/v1/telemetry", {
      method: "POST",
      headers: jsonHeaders(),
      body: telemetryBody({
        event: "tihc_ext_outbound_click",
        params: {
          link_source: "chat_link",
          target_domain: "docs.example.com",
          target_path: "/guide",
          target_url: "https://docs.example.com/guide?secret=1",
        },
        context: {
          auth_state: "anonymous",
          case_id: "case-123",
          extension_version: "1.1.1",
          plugin_id: "tidb.ai",
          surface: "sidepanel",
          unknown_context: "drop-me",
        },
      }),
    });

    expect(response.status).toBe(204);

    const [, requestInit] = fetchImpl.mock.calls[0] ?? [];
    expect(JSON.parse(String(requestInit?.body))).toMatchObject({
      client_id: "install-123",
      events: [
        {
          name: "tihc_ext_outbound_click",
          params: {
            auth_state: "anonymous",
            case_id: "case-123",
            extension_version: "1.1.1",
            link_source: "chat_link",
            plugin_id: "tidb.ai",
            session_id: 1713110400,
            surface: "sidepanel",
            target_domain: "docs.example.com",
            target_path: "/guide",
          },
        },
      ],
    });
    expect(JSON.parse(String(requestInit?.body)).events[0].params.unknown_context).toBeUndefined();
    expect(JSON.parse(String(requestInit?.body)).events[0].params.target_url).toBeUndefined();
  });

  test("normalizes chat failure payloads to failure_kind and strips free-form failure text", async () => {
    const fetchImpl = vi.fn(async (_input: RequestInfo | URL, _init?: RequestInit) =>
      new Response(null, { status: 204 }),
    );
    const app = createApp({
      env: {
        GA4_API_SECRET: "secret",
        GA4_ENABLED: "true",
        GA4_MEASUREMENT_ID: "G-TEST123",
      },
      fetchImpl,
    });

    const response = await app.request("/v1/telemetry", {
      method: "POST",
      headers: jsonHeaders(),
      body: telemetryBody({
        event: "tihc_ext_chat_failed",
        context: {
          auth_state: "anonymous",
          case_id: "case-123",
          extension_version: "1.1.1",
          failure_kind: "timeout",
          failure_reason: "socket timeout after 30 seconds with raw upstream detail",
          plugin_id: "tidb.ai",
          surface: "sidepanel",
        },
      }),
    });

    expect(response.status).toBe(204);

    const [, requestInit] = fetchImpl.mock.calls[0] ?? [];
    expect(JSON.parse(String(requestInit?.body)).events[0].params).toMatchObject({
      auth_state: "anonymous",
      case_id: "case-123",
      extension_version: "1.1.1",
      failure_kind: "timeout",
      plugin_id: "tidb.ai",
      session_id: 1713110400,
      surface: "sidepanel",
    });
    expect(JSON.parse(String(requestInit?.body)).events[0].params.failure_reason).toBeUndefined();
  });

  test("uses the GA4 debug endpoint and returns validation JSON when debug mode is requested", async () => {
    const fetchImpl = vi.fn(async (input: RequestInfo | URL, _init?: RequestInit) => {
      if (String(input).includes("/debug/mp/collect")) {
        return Response.json({
          validationMessages: [],
        });
      }
      throw new Error(`unexpected request: ${String(input)}`);
    });
    const app = createApp({
      env: {
        GA4_API_SECRET: "secret",
        GA4_ENABLED: "true",
        GA4_MEASUREMENT_ID: "G-TEST123",
      },
      fetchImpl,
    });

    const response = await app.request("/v1/telemetry", {
      method: "POST",
      headers: jsonHeaders(),
      body: telemetryBody({
        debug: true,
      }),
    });

    expect(response.status).toBe(200);
    await expect(response.json()).resolves.toMatchObject({
      validationMessages: [],
    });
    expect(fetchImpl).toHaveBeenCalledWith(
      expect.stringContaining("/debug/mp/collect?measurement_id=G-TEST123&api_secret=secret"),
      expect.objectContaining({
        method: "POST",
      }),
    );
  });

  test("logs telemetry forwarding outcomes", async () => {
    const logger = createMockLogger();
    const fetchImpl = vi.fn(async (_input: RequestInfo | URL, _init?: RequestInit) =>
      new Response(null, { status: 204 }),
    );
    const app = createApp({
      env: {
        GA4_API_SECRET: "secret",
        GA4_ENABLED: "true",
        GA4_MEASUREMENT_ID: "G-TEST123",
      },
      fetchImpl,
      logger,
    });

    const response = await app.request("/v1/telemetry", {
      method: "POST",
      headers: jsonHeaders(),
      body: telemetryBody(),
    });

    expect(response.status).toBe(204);
    expect(logger.info).toHaveBeenCalledWith(
      "telemetry.forwarded",
      expect.objectContaining({
        debug: false,
        event: "tihc_ext_chat_submitted",
        ga4_status: 204,
        request_id: expect.any(String),
        user_id_attached: false,
      }),
    );
  });
});
