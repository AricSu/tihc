import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";

const { getAppSettingsSnapshotMock } = vi.hoisted(() => ({
  getAppSettingsSnapshotMock: vi.fn(),
}));

vi.mock("@/lib/app/runtime", () => ({
  getAppSettingsSnapshot: getAppSettingsSnapshotMock,
}));

function buildSettings(overrides: Record<string, unknown> = {}) {
  return {
    activeCaseId: "case-123",
    analyticsConsent: "granted",
    cases: [],
    cloudSync: {
      importedClientId: null,
      lastHydratedAt: null,
      mode: "local",
    },
    llmRuntime: {
      baseUrl: "https://runtime.example.com",
      providerId: "openai",
      model: "gpt-4.1-mini",
    },
    googleAuth: null,
    installedPlugins: [
      {
        pluginId: "tidb.ai",
        label: "tidb.ai",
        kind: "agent",
        capabilities: ["chat"],
        config: {
          baseUrl: "https://tidb.ai",
          model: "tidb",
        },
      },
    ],
    ...overrides,
  };
}

describe("telemetry client", () => {
  beforeEach(() => {
    vi.resetModules();
    vi.restoreAllMocks();
    vi.useFakeTimers();
    getAppSettingsSnapshotMock.mockReturnValue(buildSettings());
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  test.each(["unknown", "denied"] as const)(
    "does not send telemetry while analytics consent is %s",
    async (analyticsConsent) => {
      const fetchMock = vi.fn();
      vi.stubGlobal("fetch", fetchMock);
      getAppSettingsSnapshotMock.mockReturnValue(buildSettings({
        analyticsConsent,
      }));

      const { trackTelemetryEvent } = await import("./index");
      await trackTelemetryEvent("tihc_ext_surface_viewed", {
        context: {
          surface: "sidepanel",
        },
      });

      expect(fetchMock).not.toHaveBeenCalled();
    },
  );

  test("keeps telemetry identifiers only in memory and rotates the session_id after 30 minutes of inactivity", async () => {
    const fetchMock = vi.fn(async () => new Response(null, { status: 204 }));
    vi.stubGlobal("fetch", fetchMock);
    vi.setSystemTime(new Date("2026-04-14T10:00:00.000Z"));

    const telemetry = await import("./index");
    await telemetry.trackTelemetryEvent("tihc_ext_surface_viewed", {
      context: {
        surface: "sidepanel",
      },
    });

    vi.setSystemTime(new Date("2026-04-14T10:05:00.000Z"));
    await telemetry.trackTelemetryEvent("tihc_ext_case_created", {
      context: {
        case_id: "case-123",
        surface: "sidepanel",
      },
    });

    vi.setSystemTime(new Date("2026-04-14T10:36:00.000Z"));
    await telemetry.trackTelemetryEvent("tihc_ext_surface_viewed", {
      context: {
        surface: "options",
      },
    });

    const [firstRequest, secondRequest, thirdRequest] = fetchMock.mock.calls as unknown as Array<
      [RequestInfo | URL, RequestInit | undefined]
    >;
    const firstPayload = JSON.parse(String(firstRequest?.[1]?.body));
    const secondPayload = JSON.parse(String(secondRequest?.[1]?.body));
    const thirdPayload = JSON.parse(String(thirdRequest?.[1]?.body));

    expect(firstPayload.identifiers.client_id).toBe(secondPayload.identifiers.client_id);
    expect(firstPayload.identifiers.session_id).toBe(secondPayload.identifiers.session_id);
    expect(thirdPayload.identifiers.session_id).not.toBe(firstPayload.identifiers.session_id);
  });
});
