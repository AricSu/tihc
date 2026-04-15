import { act } from "react";
import { createRoot } from "react-dom/client";
import { beforeEach, describe, expect, test, vi } from "vitest";
import { PluginManager } from "./plugin-manager";

function buildSettings() {
  return {
    activeCaseId: "case-1",
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
      {
        pluginId: "websearch",
        label: "Web Search",
        kind: "mcp",
        capabilities: ["mcp"],
        config: {
          enabled: true,
          mode: "aggressive",
          primaryEngine: "duckduckgo",
        },
      },
    ],
    cases: [],
    googleAuth: null as {
      accessToken: string;
      clientId: string;
      email: string;
      hostedDomain: string;
      expiresAt: string | null;
    } | null,
  };
}

const runtimeState = vi.hoisted(() => {
  const listeners = new Set<() => void>();
  return {
    current: null as any,
    listeners,
    emit() {
      for (const listener of listeners) listener();
    },
  };
});

const {
  clearGoogleAuthMock,
  getAppSettingsSnapshotMock,
  refreshGoogleAuthMock,
  setGoogleAuthMock,
  subscribeAppSettingsMock,
  updateGlobalLlmRuntimeMock,
  updateInstalledPluginConfigMock,
} = vi.hoisted(() => ({
  clearGoogleAuthMock: vi.fn(),
  getAppSettingsSnapshotMock: vi.fn(() => runtimeState.current),
  refreshGoogleAuthMock: vi.fn(),
  setGoogleAuthMock: vi.fn(),
  subscribeAppSettingsMock: vi.fn((listener: () => void) => {
    runtimeState.listeners.add(listener);
    return () => {
      runtimeState.listeners.delete(listener);
    };
  }),
  updateGlobalLlmRuntimeMock: vi.fn((partial: { providerId: string; model: string }) => {
    runtimeState.current = {
      ...buildSettings(),
      ...runtimeState.current,
      llmRuntime: partial,
    };
    runtimeState.emit();
  }),
  updateInstalledPluginConfigMock: vi.fn((pluginId: string, partial: Record<string, unknown>) => {
    runtimeState.current = {
      ...buildSettings(),
      ...runtimeState.current,
      installedPlugins: (runtimeState.current?.installedPlugins ?? []).map((plugin: any) =>
        plugin.pluginId === pluginId
          ? {
              ...plugin,
              config: {
                ...plugin.config,
                ...partial,
              },
        }
          : plugin,
      ),
    };
    runtimeState.emit();
  }),
}));

const {
  refreshGoogleAuthSessionMock,
  signInWithGoogleMock,
  signOutFromGoogleMock,
} = vi.hoisted(() => ({
  refreshGoogleAuthSessionMock: vi.fn(),
  signInWithGoogleMock: vi.fn(),
  signOutFromGoogleMock: vi.fn(),
}));

vi.mock("@/lib/app/runtime", () => ({
  clearGoogleAuth: clearGoogleAuthMock,
  getAppSettingsSnapshot: getAppSettingsSnapshotMock,
  refreshGoogleAuth: refreshGoogleAuthMock,
  setGoogleAuth: setGoogleAuthMock,
  subscribeAppSettings: subscribeAppSettingsMock,
  updateGlobalLlmRuntime: updateGlobalLlmRuntimeMock,
  updateInstalledPluginConfig: updateInstalledPluginConfigMock,
}));

vi.mock("@/lib/auth/google-oauth", () => ({
  isGoogleOAuthConfigured: vi.fn(() => true),
  refreshGoogleAuthSession: refreshGoogleAuthSessionMock,
  signInWithGoogle: signInWithGoogleMock,
  signOutFromGoogle: signOutFromGoogleMock,
}));

(globalThis as typeof globalThis & { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT =
  true;

async function renderPluginManager() {
  const container = document.createElement("div");
  document.body.append(container);
  const root = createRoot(container);

  await act(async () => {
    root.render(<PluginManager />);
  });

  return {
    container,
    cleanup: async () => {
      await act(async () => {
        root.unmount();
      });
      container.remove();
    },
  };
}

async function clickByText(container: HTMLElement, text: string) {
  const target = Array.from(container.querySelectorAll("button")).find((button) =>
    button.textContent?.includes(text),
  );

  expect(target).toBeTruthy();

  await act(async () => {
    target?.dispatchEvent(new MouseEvent("mousedown", { bubbles: true }));
    target?.dispatchEvent(new MouseEvent("mouseup", { bubbles: true }));
    target?.dispatchEvent(new MouseEvent("click", { bubbles: true }));
  });
}

describe("PluginManager", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.stubGlobal(
      "fetch",
      vi.fn(async () =>
        Response.json({
          providers: [
            {
              id: "openai",
              label: "OpenAI",
              authMode: "backend-managed",
              configured: true,
              defaultModel: "gpt-4.1-mini",
              models: [
                { id: "gpt-4.1-mini", label: "GPT-4.1 Mini" },
                { id: "gpt-4.1", label: "GPT-4.1" },
              ],
            },
          ],
        }),
      ),
    );
    runtimeState.current = buildSettings();
  });

  test("shows non-anonymous plugins as disabled until login", async () => {
    const { container, cleanup } = await renderPluginManager();
    const pluginButtons = Array.from(container.querySelectorAll("button"));
    const webSearchButton = pluginButtons.find((button) => button.textContent?.includes("Web Search"));

    expect(container.textContent).not.toContain("Plugins");
    expect(container.textContent).not.toContain("Manage");
    expect(container.textContent).not.toContain("Create");
    expect(container.textContent).not.toContain("Skills");
    expect(container.textContent).toContain("TiHC Native Supported");
    expect(container.textContent).toContain("tidb.ai");
    expect(container.textContent).toContain("Web Search");
    expect(container.textContent).toContain("Sign in to use this plugin");
    expect(webSearchButton).toBeTruthy();
    expect(webSearchButton?.hasAttribute("disabled")).toBe(true);
    expect(pluginButtons.some((button) => button.textContent?.includes("GitHub MCP"))).toBe(false);
    expect(pluginButtons.some((button) => button.textContent?.includes("Vercel"))).toBe(false);
    expect(container.textContent).not.toContain("OpenAI Responses");
    expect(container.querySelector('[data-plugin-icon="tidb.ai"]')).toBeTruthy();
    expect(container.querySelector('[data-plugin-icon="websearch"]')).toBeTruthy();

    await cleanup();
  });

  test("does not render management controls inside the tidb.ai detail page", async () => {
    const { container, cleanup } = await renderPluginManager();

    await clickByText(container, "tidb.ai");

    expect(container.querySelector('[data-plugin-detail-icon="tidb.ai"]')).toBeTruthy();
    expect(container.textContent).not.toContain(
      "Use tidb.ai as a TiDB-focused MCP client while the primary case chat runtime stays global and provider-driven.",
    );
    expect(container.textContent).not.toContain("Manage Settings");
    expect(container.textContent).not.toContain("Runtime Settings");
    expect(container.textContent).not.toContain("Configuration");
    expect(container.textContent).not.toContain("Base URL");
    expect(container.textContent).not.toContain("Save Plugin Settings");
    expect(container.textContent).not.toContain("Plugin Notes");
    expect(container.textContent).not.toContain("Google Workspace Auth");
    expect(container.textContent).not.toContain("Sign in with Google");
    expect(container.textContent).not.toContain("Usage Analytics");
    expect(container.textContent).not.toContain("Terms of service");
    expect(container.textContent).not.toContain("PingCAP Terms");
    expect(container.textContent).toContain("TiDB Context");
    expect(container.textContent).toContain(
      "Each case session keeps its own tidb.ai context for TiDB-focused MCP functionality.",
    );
    expect(container.textContent).toContain("Try in case");

    await cleanup();
  });

  test("does not expose a tidb.ai config form in its detail page", async () => {
    const { container, cleanup } = await renderPluginManager();

    await clickByText(container, "tidb.ai");

    const baseUrlInput = container.querySelector('input[placeholder="https://example.tidb.ai"]');
    expect(baseUrlInput).toBeNull();
    expect(updateInstalledPluginConfigMock).not.toHaveBeenCalled();

    await cleanup();
  });

  test("does not render user llm settings inside tidb.ai plugin settings", async () => {
    const { container, cleanup } = await renderPluginManager();

    await clickByText(container, "tidb.ai");

    expect(container.textContent).not.toContain("Provider");
    expect(container.textContent).not.toContain("Model");
    expect(updateGlobalLlmRuntimeMock).not.toHaveBeenCalled();

    await cleanup();
  });

  test("renders a separate websearch detail page with its own settings", async () => {
    runtimeState.current = {
      ...buildSettings(),
      googleAuth: {
        accessToken: "google-token",
        clientId: "client-id",
        email: "dev@example.com",
        hostedDomain: "example.com",
        expiresAt: "2026-04-14T16:00:00.000Z",
      },
    };
    const { container, cleanup } = await renderPluginManager();

    await clickByText(container, "Web Search");

    expect(container.querySelector('[data-plugin-detail-icon="websearch"]')).toBeTruthy();
    expect(container.textContent).toContain("Enable Web Search");
    expect(container.textContent).toContain("Web Search Mode");
    expect(container.textContent).toContain("Primary Search Engine");
    expect(container.textContent).not.toContain("Google Workspace Auth");

    await cleanup();
  });

  test("does not expose Google auth actions inside the tidb.ai detail page", async () => {
    const { container, cleanup } = await renderPluginManager();

    await clickByText(container, "tidb.ai");

    expect(container.textContent).not.toContain("Sign in with Google");
    expect(container.textContent).not.toContain("Refresh Google Token");
    expect(container.textContent).not.toContain("Sign out");
    expect(signInWithGoogleMock).not.toHaveBeenCalled();
    expect(setGoogleAuthMock).not.toHaveBeenCalled();

    await cleanup();
  });

  test("does not expose analytics controls inside the tidb.ai detail page", async () => {
    runtimeState.current = {
      ...buildSettings(),
      googleAuth: {
        accessToken: "google-token",
        clientId: "client-id",
        email: "dev@example.com",
        hostedDomain: "example.com",
        expiresAt: "2026-04-14T16:00:00.000Z",
      },
    };

    const { container, cleanup } = await renderPluginManager();

    await clickByText(container, "tidb.ai");

    expect(container.textContent).not.toContain("Usage Analytics");
    expect(container.textContent).not.toContain("Allow analytics");
    expect(container.textContent).not.toContain("Disable analytics");
    expect(signOutFromGoogleMock).not.toHaveBeenCalled();
    expect(clearGoogleAuthMock).not.toHaveBeenCalled();

    await cleanup();
  });
});
