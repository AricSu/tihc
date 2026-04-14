import { afterEach, describe, expect, test, vi } from "vitest";

const { getAppSettingsSnapshotMock } = vi.hoisted(() => ({
  getAppSettingsSnapshotMock: vi.fn<() => any>(() => ({
    activeCaseId: null,
    analyticsConsent: "unknown",
    cases: [],
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
    googleAuth: {
      accessToken: "google-access-token",
      clientId: "google-client-id",
      email: "dev@example.com",
      hostedDomain: "example.com",
      expiresAt: "2026-04-14T16:00:00.000Z",
    },
    installedPlugins: [],
  })),
}));

vi.mock("@/lib/app/runtime", () => ({
  getAppSettingsSnapshot: getAppSettingsSnapshotMock,
}));

import {
  getPluginAdapter,
  getPluginManifest,
  listMarketplacePluginCatalog,
} from "./registry";

afterEach(() => {
  vi.restoreAllMocks();
});

describe("plugin registry", () => {
  test("exposes tidb.ai and websearch as separate mcp manifests", () => {
    const manifest = getPluginManifest("tidb.ai");
    const webSearchManifest = getPluginManifest("websearch");

    expect(manifest).toMatchObject({
      pluginId: "tidb.ai",
      label: "tidb.ai",
      kind: "mcp",
      capabilities: ["mcp"],
    });
    expect(manifest.settingsFields.map((field) => field.key)).toEqual(["baseUrl"]);
    expect(webSearchManifest).toMatchObject({
      pluginId: "websearch",
      label: "Web Search",
      kind: "mcp",
      capabilities: ["mcp"],
    });
    expect(webSearchManifest.settingsFields.map((field) => field.key)).toEqual([
      "enabled",
      "mode",
      "primaryEngine",
    ]);
  });

  test("tidb.ai no longer exposes a standalone chat connection test", async () => {
    const adapter = getPluginAdapter("tidb.ai");
    const result = await adapter.testConnection({
      pluginId: "tidb.ai",
      label: "tidb.ai",
      kind: "mcp",
      capabilities: ["mcp"],
      config: {
        baseUrl: "https://tidb.ai",
      },
    });

    expect(result).toEqual({
      ok: true,
      message: "tidb.ai is configured as an MCP client and does not provide a standalone chat connection test.",
    });
  });

  test("exposes a marketplace catalog with separate installed tidb and websearch plugins", () => {
    const catalog = listMarketplacePluginCatalog(["tidb.ai", "websearch"]);

    expect(catalog[0]).toMatchObject({
      catalogId: "tidb.ai",
      installedPluginId: "tidb.ai",
      status: "installed",
    });
    expect(catalog[1]).toMatchObject({
      catalogId: "websearch",
      installedPluginId: "websearch",
      status: "installed",
    });
    expect(catalog.some((entry) => entry.catalogId === "websearch" && entry.status === "installed")).toBe(true);
    expect(catalog.some((entry) => entry.catalogId === "github-mcp")).toBe(false);
    expect(catalog.some((entry) => entry.catalogId === "vercel")).toBe(false);
    expect(catalog.some((entry) => entry.catalogId === "browser-automation")).toBe(false);
    expect(catalog.some((entry) => entry.catalogId === "openai-responses")).toBe(false);
    expect(catalog.some((entry) => entry.status === "coming-soon")).toBe(false);
  });
});
