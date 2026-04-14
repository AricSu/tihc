import { act } from "react";
import { createRoot } from "react-dom/client";
import { renderToStaticMarkup } from "react-dom/server";
import type { ReactNode } from "react";
import { beforeEach, describe, expect, test, vi } from "vitest";
import {
  ANONYMOUS_LOCAL_STORAGE_LIMIT_BYTES,
  formatStorageBytes,
} from "@/lib/app/anonymous-local-case-limit";
import type { AppRuntimeSettings } from "@/lib/chat/agent-types";
import { CaseShell } from "./agent-shell";

const {
  openCaseCreationPageMock,
  openPluginSettingsPageMock,
} = vi.hoisted(() => ({
  openCaseCreationPageMock: vi.fn(),
  openPluginSettingsPageMock: vi.fn(),
}));

vi.mock("@/lib/app/runtime", () => ({
  setActiveCaseId: vi.fn(),
  renameCase: vi.fn(),
  resolveCase: vi.fn(),
  reopenCase: vi.fn(),
  archiveCase: vi.fn(),
  unarchiveCase: vi.fn(),
  deleteCase: vi.fn(),
}));

vi.mock("@/lib/telemetry", () => ({
  trackTelemetryEvent: vi.fn(),
}));

vi.mock("@/lib/app/settings-page", () => ({
  openCaseCreationPage: openCaseCreationPageMock,
  openPluginSettingsPage: openPluginSettingsPageMock,
}));

vi.mock("@/components/MLCProvider", () => ({
  MLCProvider: ({ children }: { children: ReactNode }) => <>{children}</>,
}));

vi.mock("@/components/ui/dropdown-menu", () => ({
  DropdownMenu: ({ children }: { children: ReactNode }) => <div>{children}</div>,
  DropdownMenuContent: ({ children }: { children: ReactNode }) => <div>{children}</div>,
  DropdownMenuGroup: ({ children }: { children: ReactNode }) => <div>{children}</div>,
  DropdownMenuItem: ({
    children,
    onSelect,
  }: {
    children: ReactNode;
    onSelect?: (event: { preventDefault: () => void }) => void;
  }) => (
    <button
      type="button"
      onClick={() =>
        onSelect?.({
          preventDefault() {},
        })
      }
    >
      {children}
    </button>
  ),
  DropdownMenuLabel: ({ children }: { children: ReactNode }) => <div>{children}</div>,
  DropdownMenuRadioGroup: ({ children }: { children: ReactNode }) => <div>{children}</div>,
  DropdownMenuRadioItem: ({ children }: { children: ReactNode }) => <div>{children}</div>,
  DropdownMenuSeparator: () => <hr />,
  DropdownMenuTrigger: ({ children }: { children: ReactNode }) => <div>{children}</div>,
}));

vi.mock("@/components/assistant-ui/thread", () => ({
  Thread: ({ composerToolbar }: { composerToolbar?: ReactNode }) => (
    <div data-testid="thread">
      Thread mounted
      <div data-testid="composer-toolbar">{composerToolbar}</div>
    </div>
  ),
}));

(globalThis as typeof globalThis & { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT =
  true;

function installMockStorage() {
  const storage = new Map<string, string>();
  Object.defineProperty(globalThis, "localStorage", {
    configurable: true,
    value: {
      clear() {
        storage.clear();
      },
      getItem(key: string) {
        return storage.get(key) ?? null;
      },
      key(index: number) {
        return [...storage.keys()][index] ?? null;
      },
      removeItem(key: string) {
        storage.delete(key);
      },
      setItem(key: string, value: string) {
        storage.set(key, value);
      },
      get length() {
        return storage.size;
      },
    } satisfies Storage,
  });
}

function clearBrowserStorage() {
  const storage = window.localStorage;
  if (typeof storage.clear === "function") {
    storage.clear();
    return;
  }

  const keys: string[] = [];
  for (let index = 0; index < storage.length; index += 1) {
    const key = storage.key(index);
    if (key) keys.push(key);
  }
  keys.forEach((key) => storage.removeItem(key));
}

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
        title: "Archived case",
        pluginId: "tidb.ai",
        activityState: "resolved",
        resolvedAt: "2026-03-17T10:00:00.000Z",
        archivedAt: "2026-03-17T10:10:00.000Z",
        createdAt: "2026-03-17T10:00:00.000Z",
        updatedAt: "2026-03-17T10:10:00.000Z",
      },
      {
        id: "case-2",
        title: "Ticket 417",
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

describe("CaseShell", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    installMockStorage();
    clearBrowserStorage();
  });

  test("renders the full thread shell for signed-in users", () => {
    const html = renderToStaticMarkup(
      <CaseShell
        settings={{
          ...buildSettings(),
          googleAuth: {
            accessToken: "google-token",
            clientId: "client-id",
            email: "dev@example.com",
            hostedDomain: "example.com",
            expiresAt: "2026-04-14T16:00:00.000Z",
          },
        }}
      />,
    );

    expect(html).toContain("Thread mounted");
    expect(html).toContain("Ticket 417");
    expect(html).not.toContain("Archived case");
    expect(html).toContain("Rename");
    expect(html).toContain("Resolve");
    expect(html).toContain("Archive");
    expect(html).toContain("Delete");
    expect(html).toContain("Plugin Settings");
  });

  test("reuses the normal thread shell for anonymous users instead of rendering a separate local-only page", () => {
    const html = renderToStaticMarkup(<CaseShell settings={buildSettings()} />);

    expect(html).toContain("Ticket 417");
    expect(html).toContain("Create case");
    expect(html).toContain("Plugin Settings");
    expect(html).toContain("Thread mounted");
    expect(html).not.toContain("stored in this browser");
    expect(html).not.toContain("Sign in to enable agent runs");
  });

  test("renders the anonymous local storage blocker when the browser case limit is reached", () => {
    const container = document.createElement("div");
    document.body.append(container);
    const root = createRoot(container);
    const oversizedText = "x".repeat(Math.ceil(ANONYMOUS_LOCAL_STORAGE_LIMIT_BYTES / 2));
    window.localStorage.setItem(
      "tihc_local_history_v2:case-2",
      JSON.stringify({
        headId: "m-1",
        messages: [
          {
            parentId: null,
            message: {
              id: "m-1",
              role: "user",
              content: [{ type: "text", text: oversizedText }],
              createdAt: "2026-04-14T12:00:00.000Z",
              attachments: [],
              metadata: {},
            },
          },
        ],
      }),
    );
    const settings = buildSettings();

    act(() => {
      root.render(<CaseShell settings={settings} />);
    });

    expect(document.body.textContent).toContain("Delete local cases to continue");
    expect(document.body.textContent).toContain(
      "Anonymous mode is limited by browser storage usage, not case count.",
    );
    expect(document.body.textContent).toContain(formatStorageBytes(ANONYMOUS_LOCAL_STORAGE_LIMIT_BYTES));
    expect(document.body.textContent).toContain("Ticket 417");
    expect(
      Array.from(document.querySelectorAll("button")).some(
        (button) => button.getAttribute("aria-label") === "Delete case Ticket 417",
      ),
    ).toBe(true);

    act(() => {
      root.unmount();
    });
    container.remove();
  });

  test("routes create-case actions through the options page flow", async () => {
    const container = document.createElement("div");
    document.body.append(container);
    const root = createRoot(container);

    await act(async () => {
      root.render(<CaseShell settings={buildSettings()} />);
    });

    const createCaseButton = Array.from(container.querySelectorAll("button")).find((button) =>
      button.textContent?.includes("Create case"),
    );

    expect(createCaseButton).toBeTruthy();

    await act(async () => {
      createCaseButton?.dispatchEvent(new MouseEvent("click", { bubbles: true }));
    });

    expect(openCaseCreationPageMock).toHaveBeenCalledTimes(1);

    await act(async () => {
      root.unmount();
    });
    container.remove();
  });

  test("opens plugin settings from the overflow actions", async () => {
    const container = document.createElement("div");
    document.body.append(container);
    const root = createRoot(container);

    await act(async () => {
      root.render(<CaseShell settings={buildSettings()} />);
    });

    const pluginSettingsButton = Array.from(container.querySelectorAll("button")).find((button) =>
      button.textContent?.includes("Plugin Settings"),
    );

    expect(pluginSettingsButton).toBeTruthy();

    await act(async () => {
      pluginSettingsButton?.dispatchEvent(new MouseEvent("click", { bubbles: true }));
    });

    expect(openPluginSettingsPageMock).toHaveBeenCalledTimes(1);

    await act(async () => {
      root.unmount();
    });
    container.remove();
  });

  test("shows an empty state when no visible cases exist", () => {
    const settings = buildSettings();
    settings.activeCaseId = null;
    settings.cases = settings.cases.map((item) => ({
      ...item,
      archivedAt: item.archivedAt ?? "2026-03-17T10:20:00.000Z",
    }));

    const html = renderToStaticMarkup(<CaseShell settings={settings} />);

    expect(html).toContain("No cases yet");
    expect(html).toContain("Create case");
    expect(html).not.toContain("Thread mounted");
  });
});
