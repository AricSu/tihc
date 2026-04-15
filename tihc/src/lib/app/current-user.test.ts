import { beforeEach, describe, expect, test, vi } from "vitest";

function buildSettings(overrides: Record<string, unknown> = {}) {
  return {
    activeCaseId: "case-1",
    analyticsConsent: "unknown" as const,
    cases: [],
    cloudSync: {
      importedClientId: null,
      lastHydratedAt: null,
      mode: "local" as const,
    },
    llmRuntime: {
      baseUrl: "https://runtime.example.com",
      providerId: "openai",
      model: "gpt-4.1-mini",
    },
    installedPlugins: [],
    googleAuth: null,
    ...overrides,
  };
}

async function loadModule() {
  vi.resetModules();
  return import("./current-user");
}

describe("current user helper", () => {
  beforeEach(() => {
    vi.stubGlobal("fetch", vi.fn());
  });

  test("prefers backend current-user data when available", async () => {
    vi.mocked(fetch).mockResolvedValueOnce(
      Response.json({
        user: {
          id: "principal-1",
          authState: "authenticated",
          displayName: "Alice Smith",
          email: "alice.smith@example.com",
          hostedDomain: "example.com",
        },
      }),
    );

    const currentUser = await loadModule();
    const user = await currentUser.getCurrentUser(
      buildSettings({
        googleAuth: {
          accessToken: "google-token",
          clientId: "google-client-id",
          email: "alice.smith@example.com",
          hostedDomain: "example.com",
          expiresAt: "2026-04-15T18:00:00.000Z",
        },
      }),
    );

    expect(fetch).toHaveBeenCalledWith("https://runtime.example.com/v1/me", {
      headers: {
        Authorization: "Bearer google-token",
      },
      method: "GET",
    });
    expect(user).toMatchObject({
      id: "principal-1",
      authState: "authenticated",
      displayName: "Alice Smith",
      email: "alice.smith@example.com",
      hostedDomain: "example.com",
    });
  });

  test("falls back to an anonymous local user when the backend user endpoint is unavailable", async () => {
    vi.mocked(fetch).mockRejectedValueOnce(new Error("network down"));

    const currentUser = await loadModule();
    const user = await currentUser.getCurrentUser(buildSettings());

    expect(user).toMatchObject({
      id: null,
      authState: "anonymous",
      displayName: "匿名",
      email: "",
      hostedDomain: "",
    });
  });
});
