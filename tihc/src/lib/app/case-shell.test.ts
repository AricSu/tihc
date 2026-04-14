import { describe, expect, test } from "vitest";
import { resolveActiveCaseWorkspace } from "./case-shell";
import type { AppRuntimeSettings } from "@/lib/chat/agent-types";

function buildSettings(): AppRuntimeSettings {
  return {
    activeCaseId: "case-2",
    analyticsConsent: "unknown",
    cloudSync: {
      importedClientId: null,
      lastHydratedAt: null,
      mode: "local",
    },
    llmRuntime: {
      baseUrl: "",
      providerId: "",
      model: "",
    },
    installedPlugins: [
      {
        pluginId: "tidb.ai",
        label: "tidb.ai",
        kind: "mcp",
        capabilities: ["mcp"],
        config: {
          baseUrl: "https://tidb.ai",
        },
      },
    ],
    cases: [
      {
        id: "case-1",
        title: "Primary",
        pluginId: "tidb.ai",
        activityState: "ready",
        resolvedAt: null,
        archivedAt: null,
        createdAt: "2026-03-17T10:00:00.000Z",
        updatedAt: "2026-03-17T10:00:00.000Z",
      },
      {
        id: "case-2",
        title: "Secondary",
        pluginId: "tidb.ai",
        activityState: "active",
        resolvedAt: null,
        archivedAt: null,
        createdAt: "2026-03-17T10:05:00.000Z",
        updatedAt: "2026-03-17T10:05:00.000Z",
      },
    ],
    googleAuth: null,
  };
}

describe("resolveActiveCaseWorkspace", () => {
  test("returns the active case", () => {
    const resolved = resolveActiveCaseWorkspace(buildSettings());

    expect(resolved.activeCase?.id).toBe("case-2");
  });

  test("falls back to the first visible case when activeCaseId is missing or archived", () => {
    const settings = buildSettings();
    settings.activeCaseId = "missing";
    settings.cases[0] = {
      ...settings.cases[0]!,
      archivedAt: "2026-03-17T10:10:00.000Z",
    };

    const resolved = resolveActiveCaseWorkspace(settings);

    expect(resolved.activeCase?.id).toBe("case-2");
  });
});
