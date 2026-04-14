import { act } from "react";
import { createRoot } from "react-dom/client";
import { beforeEach, describe, expect, test, vi } from "vitest";
import { UserLlmSettingsPanel } from "./user-llm-settings-panel";

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

function buildSettings() {
  return {
    activeCaseId: "case-1",
    assistantReplyFontSize: "default",
    analyticsConsent: "unknown",
    cloudSync: {
      importedClientId: "client-1",
      lastHydratedAt: "2026-04-14T12:05:00.000Z",
      mode: "cloud",
    },
    llmRuntime: {
      baseUrl: "https://runtime.example.com",
      providerId: "openai",
      model: "gpt-4.1-mini",
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
    cases: [],
    googleAuth: {
      accessToken: "token-1",
      clientId: "google-client-id",
      email: "dev@example.com",
      hostedDomain: "example.com",
      expiresAt: "2026-04-14T16:00:00.000Z",
    },
  };
}

const {
  getAppSettingsSnapshotMock,
  subscribeAppSettingsMock,
  updateGlobalLlmRuntimeMock,
} = vi.hoisted(() => ({
  getAppSettingsSnapshotMock: vi.fn(() => runtimeState.current),
  subscribeAppSettingsMock: vi.fn((listener: () => void) => {
    runtimeState.listeners.add(listener);
    return () => {
      runtimeState.listeners.delete(listener);
    };
  }),
  updateGlobalLlmRuntimeMock: vi.fn((partial: { baseUrl: string; providerId: string; model: string }) => {
    runtimeState.current = {
      ...runtimeState.current,
      llmRuntime: {
        ...runtimeState.current.llmRuntime,
        ...partial,
      },
    };
    runtimeState.emit();
  }),
}));

const {
  getStoredLlmCredentialStatusMock,
  listLlmProvidersMock,
  saveStoredLlmCredentialMock,
} = vi.hoisted(() => ({
  getStoredLlmCredentialStatusMock: vi.fn(async () => ({
    providerId: "openai",
    hasSecret: false,
    updatedAt: null,
  })),
  listLlmProvidersMock: vi.fn(async () => [
    {
      id: "openai",
      label: "OpenAI",
      authMode: "user-api-key",
      configured: false,
      defaultModel: "gpt-4.1-mini",
      models: [
        { id: "gpt-4.1-mini", label: "GPT-4.1 Mini" },
        { id: "gpt-4.1", label: "GPT-4.1" },
      ],
    },
  ]),
  saveStoredLlmCredentialMock: vi.fn(async () => ({
    providerId: "openai",
    hasSecret: true,
    updatedAt: "2026-04-14T12:10:00.000Z",
  })),
}));

vi.mock("@/lib/app/runtime", () => ({
  getAppSettingsSnapshot: getAppSettingsSnapshotMock,
  subscribeAppSettings: subscribeAppSettingsMock,
  updateGlobalLlmRuntime: updateGlobalLlmRuntimeMock,
}));

vi.mock("@/lib/app/cloud-settings", () => ({
  getStoredLlmCredentialStatus: getStoredLlmCredentialStatusMock,
  listLlmProviders: listLlmProvidersMock,
  saveStoredLlmCredential: saveStoredLlmCredentialMock,
}));

(globalThis as typeof globalThis & { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT =
  true;

async function setInputValue(input: Element | null, value: string) {
  if (!(input instanceof HTMLInputElement)) return;
  await act(async () => {
    const descriptor = Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, "value");
    descriptor?.set?.call(input, value);
    input.dispatchEvent(new Event("input", { bubbles: true }));
    input.dispatchEvent(new Event("change", { bubbles: true }));
    await Promise.resolve();
  });
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
    await Promise.resolve();
    await Promise.resolve();
  });
}

async function renderPanel() {
  const container = document.createElement("div");
  document.body.append(container);
  const root = createRoot(container);

  await act(async () => {
    root.render(<UserLlmSettingsPanel />);
    await Promise.resolve();
    await Promise.resolve();
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

describe("UserLlmSettingsPanel", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    runtimeState.current = buildSettings();
  });

  test("saves the non-secret llm runtime separately from the write-only provider secret", async () => {
    const { container, cleanup } = await renderPanel();

    expect(container.textContent).toContain("User LLM Settings");

    const apiKeyInput = container.querySelector('input[type="password"]');
    await setInputValue(apiKeyInput, "sk-user-openai-secret");

    await clickByText(container, "Save LLM Settings");

    expect(updateGlobalLlmRuntimeMock).toHaveBeenCalledWith({
      baseUrl: "https://runtime.example.com",
      providerId: "openai",
      model: "gpt-4.1-mini",
    });
    expect(saveStoredLlmCredentialMock).toHaveBeenCalledWith(
      runtimeState.current,
      expect.objectContaining({
        providerId: "openai",
        apiKey: "sk-user-openai-secret",
      }),
    );
    expect(JSON.stringify(runtimeState.current)).not.toContain("sk-user-openai-secret");

    await cleanup();
  });

  test("shows a visible error when the provider catalog cannot be loaded", async () => {
    listLlmProvidersMock.mockRejectedValueOnce(new Error("backend unavailable"));

    const { container, cleanup } = await renderPanel();

    const providerSelect = container.querySelector("select");
    expect(providerSelect).toBeTruthy();
    expect((providerSelect as HTMLSelectElement).disabled).toBe(false);
    expect(container.textContent).toContain("OpenAI");
    expect(container.textContent).toContain("Anthropic");
    expect(container.textContent).toContain("Google");

    await cleanup();
  });

  test("keeps provider selection available with a local fallback catalog when backend base url is missing", async () => {
    runtimeState.current = {
      ...buildSettings(),
      installedPlugins: [
        {
          pluginId: "tidb.ai",
          label: "tidb.ai",
          kind: "mcp",
          capabilities: ["mcp"],
          config: {
            baseUrl: "",
          },
        },
      ],
      llmRuntime: {
        ...buildSettings().llmRuntime,
        baseUrl: "",
      },
    };

    const { container, cleanup } = await renderPanel();

    const providerSelect = container.querySelector("select");
    expect(providerSelect).toBeTruthy();
    expect((providerSelect as HTMLSelectElement).disabled).toBe(false);
    expect(container.textContent).toContain("Anthropic");
    expect(container.textContent).toContain("Google");
    expect(container.textContent).toContain("OpenRouter");

    await cleanup();
  });

  test("does not render a separate status card", async () => {
    const { container, cleanup } = await renderPanel();

    expect(container.textContent).not.toContain("Check what is ready before saving.");
    expect(container.textContent).not.toContain("Save status");
    expect(container.textContent).not.toContain("Provider list");
    expect(container.textContent).not.toContain("Google sign-in");
    expect(container.textContent).not.toContain("OpenAI API key");
    expect(container.textContent).not.toContain("Status");

    await cleanup();
  });

  test("does not render the assistant reply font size control after it moves to sidebar settings", async () => {
    const { container, cleanup } = await renderPanel();

    expect(container.textContent).not.toContain("Reply font size");

    await cleanup();
  });
});
