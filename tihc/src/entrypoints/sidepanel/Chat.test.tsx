import { act } from "react";
import { createRoot } from "react-dom/client";
import { beforeEach, describe, expect, test, vi } from "vitest";
import type { AppRuntimeSettings } from "@/lib/chat/agent-types";
import Chat from "./Chat";

const runtimeState = vi.hoisted(() => {
  const listeners = new Set<() => void>();
  const buildSettings = (): AppRuntimeSettings => ({
    activeCaseId: null,
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
    cases: [],
    googleAuth: null,
  });

  return {
    listeners,
    current: buildSettings(),
    reset() {
      this.current = buildSettings();
      this.listeners.clear();
    },
  };
});

const { createCaseMock, ensureGoogleAuthSessionMock, syncCloudCasesIfNeededMock } = vi.hoisted(() => ({
  createCaseMock: vi.fn((title = "New Case") => {
    const nextCase = {
      id: "case-1",
      title: title.trim() || "New Case",
      pluginId: "tidb.ai" as const,
      activityState: "ready" as const,
      resolvedAt: null,
      archivedAt: null,
      createdAt: "2026-04-14T10:00:00.000Z",
      updatedAt: "2026-04-14T10:00:00.000Z",
    };
    runtimeState.current = {
      ...runtimeState.current,
      activeCaseId: nextCase.id,
      cases: [nextCase],
    };
    for (const listener of runtimeState.listeners) listener();
    return nextCase;
  }),
  ensureGoogleAuthSessionMock: vi.fn(async () => null),
  syncCloudCasesIfNeededMock: vi.fn(),
}));

vi.mock("@/lib/app/runtime", () => ({
  createCase: createCaseMock,
  ensureGoogleAuthSession: ensureGoogleAuthSessionMock,
  getAppSettingsSnapshot: vi.fn(() => runtimeState.current),
  syncCloudCasesIfNeeded: syncCloudCasesIfNeededMock,
  subscribeAppSettings: vi.fn((listener: () => void) => {
    runtimeState.listeners.add(listener);
    return () => {
      runtimeState.listeners.delete(listener);
    };
  }),
}));

vi.mock("@/components/agent/agent-shell", () => ({
  CaseShell: ({ settings }: { settings: AppRuntimeSettings }) =>
    settings.activeCaseId ? (
      <div>
        <div>Thread mounted</div>
        <input aria-label="Message input" />
      </div>
    ) : (
      <div>No cases yet</div>
    ),
}));

(globalThis as typeof globalThis & { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT =
  true;

describe("sidepanel Chat", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    runtimeState.reset();
  });

  test("bootstraps a default case so the composer remains available when no case exists", async () => {
    const container = document.createElement("div");
    document.body.append(container);
    const root = createRoot(container);

    await act(async () => {
      root.render(<Chat />);
    });

    expect(createCaseMock).toHaveBeenCalledTimes(1);
    expect(createCaseMock).toHaveBeenCalledWith("");
    expect(container.textContent).toContain("Thread mounted");
    expect(container.textContent).toContain("Help improve TIHC");
    expect(container.textContent).toContain("Allow analytics");
    expect(container.querySelector('[aria-label="Message input"]')).toBeTruthy();
    expect(container.textContent).not.toContain("No cases yet");

    await act(async () => {
      root.unmount();
    });
    container.remove();
  });

  test("hides the analytics prompt after consent has already been denied", async () => {
    runtimeState.current = {
      ...runtimeState.current,
      analyticsConsent: "denied",
    };

    const container = document.createElement("div");
    document.body.append(container);
    const root = createRoot(container);

    await act(async () => {
      root.render(<Chat />);
    });

    expect(container.textContent).not.toContain("Help improve TIHC");

    await act(async () => {
      root.unmount();
    });
    container.remove();
  });
});
