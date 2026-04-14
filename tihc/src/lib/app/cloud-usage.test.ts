import { beforeEach, describe, expect, test, vi } from "vitest";

function buildSettings() {
  return {
    activeCaseId: "case-1",
    analyticsConsent: "unknown" as const,
    cases: [],
    cloudSync: {
      importedClientId: "client-1",
      lastHydratedAt: "2026-04-15T09:00:00.000Z",
      mode: "cloud" as const,
    },
    llmRuntime: {
      baseUrl: "https://runtime.example.com",
      providerId: "openai",
      model: "gpt-4.1-mini",
    },
    installedPlugins: [],
    googleAuth: {
      accessToken: "google-token",
      clientId: "google-client-id",
      email: "alice@example.com",
      hostedDomain: "example.com",
      expiresAt: "2026-04-15T18:00:00.000Z",
    },
  };
}

async function loadModule() {
  vi.resetModules();
  return import("./cloud-usage");
}

describe("cloud usage api", () => {
  beforeEach(() => {
    vi.stubGlobal("fetch", vi.fn());
  });

  test("fetches usage summary for the authenticated principal", async () => {
    vi.mocked(fetch).mockResolvedValueOnce(
      Response.json({
        summary: {
          windowDays: 30,
          current: {
            requestCount: 3,
            inputTokens: 120,
            outputTokens: 80,
            totalTokens: 200,
            cachedInputTokens: 20,
            reasoningTokens: 10,
            costUsd: 0,
          },
          previous: {
            requestCount: 1,
            inputTokens: 10,
            outputTokens: 5,
            totalTokens: 15,
            cachedInputTokens: 0,
            reasoningTokens: 0,
            costUsd: 0,
          },
        },
      }),
    );

    const cloudUsage = await loadModule();
    const summary = await cloudUsage.getStoredUsageSummary(buildSettings(), 30);

    expect(fetch).toHaveBeenCalledWith("https://runtime.example.com/v1/usage/summary?days=30", {
      headers: {
        Authorization: "Bearer google-token",
      },
      method: "GET",
    });
    expect(summary).toMatchObject({
      current: {
        totalTokens: 200,
      },
      previous: {
        totalTokens: 15,
      },
      windowDays: 30,
    });
  });

  test("fetches usage timeseries for the authenticated principal", async () => {
    vi.mocked(fetch).mockResolvedValueOnce(
      Response.json({
        points: [
          {
            date: "2026-04-14",
            requestCount: 1,
            inputTokens: 12,
            outputTokens: 4,
            totalTokens: 16,
            cachedInputTokens: 0,
            reasoningTokens: 0,
            costUsd: 0,
          },
        ],
      }),
    );

    const cloudUsage = await loadModule();
    const points = await cloudUsage.getStoredUsageTimeseries(buildSettings(), 30);

    expect(fetch).toHaveBeenCalledWith("https://runtime.example.com/v1/usage/timeseries?days=30", {
      headers: {
        Authorization: "Bearer google-token",
      },
      method: "GET",
    });
    expect(points).toEqual([
      expect.objectContaining({
        date: "2026-04-14",
        totalTokens: 16,
      }),
    ]);
  });
});
