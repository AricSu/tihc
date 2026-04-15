import * as React from "react";
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
  openGeneralSettingsPageMock,
  openPluginSettingsPageMock,
  deleteCaseMock,
  renameCaseMock,
  setActiveCaseIdMock,
} = vi.hoisted(() => ({
  openCaseCreationPageMock: vi.fn(),
  openGeneralSettingsPageMock: vi.fn(),
  openPluginSettingsPageMock: vi.fn(),
  deleteCaseMock: vi.fn(),
  renameCaseMock: vi.fn(),
  setActiveCaseIdMock: vi.fn(),
}));

vi.mock("@/lib/app/runtime", () => ({
  deleteCase: deleteCaseMock,
  setActiveCaseId: setActiveCaseIdMock,
  renameCase: renameCaseMock,
  resolveCase: vi.fn(),
  reopenCase: vi.fn(),
  archiveCase: vi.fn(),
  unarchiveCase: vi.fn(),
}));

vi.mock("@/lib/telemetry", () => ({
  trackTelemetryEvent: vi.fn(),
}));

vi.mock("@/lib/app/settings-page", () => ({
  openCaseCreationPage: openCaseCreationPageMock,
  openGeneralSettingsPage: openGeneralSettingsPageMock,
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
    onContextMenu,
  }: {
    children: ReactNode;
    onSelect?: (event: { preventDefault: () => void }) => void;
    onContextMenu?: (event: { preventDefault: () => void }) => void;
  }) => (
    <button
      type="button"
      onContextMenu={() =>
        onContextMenu?.({
          preventDefault() {},
        })
      }
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

vi.mock("@/components/ui/context-menu", () => {
  const Context = React.createContext(false)
  const SetContext = React.createContext<((open: boolean) => void) | null>(null)

  return {
    ContextMenu: ({ children }: { children: ReactNode }) => {
      const [open, setOpen] = React.useState(false)
      return (
        <SetContext.Provider value={setOpen}>
          <Context.Provider value={open}>{children}</Context.Provider>
        </SetContext.Provider>
      )
    },
    ContextMenuTrigger: ({ children }: { children: ReactNode }) => {
      const setOpen = React.useContext(SetContext)
      return (
        <div
          onContextMenu={(event) => {
            event.preventDefault()
            setOpen?.(true)
          }}
        >
          {children}
        </div>
      )
    },
    ContextMenuContent: ({ children }: { children: ReactNode }) => {
      const open = React.useContext(Context)
      return open ? <div>{children}</div> : null
    },
  ContextMenuItem: ({
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
  }
});

vi.mock("@/components/assistant-ui/thread", () => ({
  Thread: ({
    composerStart,
    composerToolbar,
  }: {
    composerStart?: ReactNode;
    composerToolbar?: ReactNode;
  }) => (
    <div data-testid="thread">
      Thread mounted
      <div data-testid="composer-start-state">
        {composerStart ? "Custom start slot" : "Default start slot"}
      </div>
      <div data-testid="composer-start">{composerStart}</div>
      <div data-testid="composer-toolbar-state">
        {composerToolbar ? "Custom toolbar" : "Default toolbar"}
      </div>
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
    expect(html).toContain("Custom start slot");
    expect(html).toContain("aria-label=\"Select case\"");
    expect(html).toContain("aria-label=\"Open settings\"");
    expect(html).toContain("New case");
    expect(html).not.toContain("aria-label=\"Create new case\"");
    expect(html).toContain("Ticket 417");
    expect(html).toContain("Default toolbar");
    expect(html).not.toContain("Archived case");
    expect(html).not.toContain("Rename");
    expect(html).not.toContain("Resolve");
    expect(html).not.toContain("Archive");
    expect(html).not.toContain("Delete");
    expect(html).not.toContain("Plugin Settings");
  });

  test("reuses the normal thread shell for anonymous users without extra sidepanel controls", () => {
    const html = renderToStaticMarkup(<CaseShell settings={buildSettings()} />);

    expect(html).toContain("Thread mounted");
    expect(html).toContain("Custom start slot");
    expect(html).toContain("aria-label=\"Select case\"");
    expect(html).toContain("aria-label=\"Open settings\"");
    expect(html).toContain("New case");
    expect(html).not.toContain("aria-label=\"Create new case\"");
    expect(html).toContain("Ticket 417");
    expect(html).toContain("Default toolbar");
    expect(html).not.toContain("Create case");
    expect(html).not.toContain("Plugin Settings");
    expect(html).not.toContain("stored in this browser");
    expect(html).not.toContain("Sign in to enable agent runs");
  });

  test("switches to another visible case from the selector", async () => {
    const container = document.createElement("div");
    document.body.append(container);
    const root = createRoot(container);
    const settings = buildSettings();
    settings.cases = [
      ...settings.cases,
      {
        id: "case-3",
        title: "Database timeout",
        pluginId: "tidb.ai",
        activityState: "ready",
        resolvedAt: null,
        archivedAt: null,
        createdAt: "2026-03-17T10:15:00.000Z",
        updatedAt: "2026-03-17T10:15:00.000Z",
      },
    ];

    await act(async () => {
      root.render(<CaseShell settings={settings} />);
    });

    const switchTarget = Array.from(container.querySelectorAll("button")).find(
      (button) => button.textContent?.includes("Database timeout"),
    );

    expect(switchTarget).toBeTruthy();

    await act(async () => {
      switchTarget?.dispatchEvent(new MouseEvent("click", { bubbles: true }));
    });

    expect(setActiveCaseIdMock).toHaveBeenCalledWith("case-3");

    await act(async () => {
      root.unmount();
    });
    container.remove();
  });

  test("shows a delete action for selector cases on right click and deletes only after clicking it", async () => {
    const container = document.createElement("div");
    document.body.append(container);
    const root = createRoot(container);
    const settings = buildSettings();
    settings.cases = [
      ...settings.cases,
      {
        id: "case-3",
        title: "Database timeout",
        pluginId: "tidb.ai",
        activityState: "ready",
        resolvedAt: null,
        archivedAt: null,
        createdAt: "2026-03-17T10:15:00.000Z",
        updatedAt: "2026-03-17T10:15:00.000Z",
      },
    ];

    await act(async () => {
      root.render(<CaseShell settings={settings} />);
    });

    expect(container.textContent).not.toContain("Delete case");

    const caseItem = Array.from(container.querySelectorAll("button")).find(
      (button) => button.textContent?.includes("Database timeout"),
    );

    expect(caseItem).toBeTruthy();

    await act(async () => {
      caseItem?.dispatchEvent(new MouseEvent("contextmenu", { bubbles: true, cancelable: true }));
      await Promise.resolve();
    });

    expect(container.textContent).toContain("Delete case");

    const deleteAction = Array.from(container.querySelectorAll("button")).find(
      (button) => button.textContent?.trim() === "Delete case",
    );

    expect(deleteAction).toBeTruthy();

    await act(async () => {
      deleteAction?.dispatchEvent(new MouseEvent("click", { bubbles: true, cancelable: true }));
    });

    expect(deleteCaseMock).toHaveBeenCalledWith("case-3");

    await act(async () => {
      root.unmount();
    });
    container.remove();
  });

  test("renames a case from the selector context menu", async () => {
    const container = document.createElement("div");
    document.body.append(container);
    const root = createRoot(container);
    const settings = buildSettings();
    settings.cases = [
      ...settings.cases,
      {
        id: "case-3",
        title: "Database timeout",
        pluginId: "tidb.ai",
        activityState: "ready",
        resolvedAt: null,
        archivedAt: null,
        createdAt: "2026-03-17T10:15:00.000Z",
        updatedAt: "2026-03-17T10:15:00.000Z",
      },
    ];

    await act(async () => {
      root.render(<CaseShell settings={settings} />);
    });

    const caseItem = Array.from(container.querySelectorAll("button")).find(
      (button) => button.textContent?.includes("Database timeout"),
    );

    expect(caseItem).toBeTruthy();

    await act(async () => {
      caseItem?.dispatchEvent(new MouseEvent("contextmenu", { bubbles: true, cancelable: true }));
      await Promise.resolve();
    });

    const renameAction = Array.from(container.querySelectorAll("button")).find(
      (button) => button.textContent?.trim() === "Rename case",
    );

    expect(renameAction).toBeTruthy();

    await act(async () => {
      renameAction?.dispatchEvent(new MouseEvent("click", { bubbles: true, cancelable: true }));
      await Promise.resolve();
    });

    const input = document.querySelector('input[aria-label="Case name"]') as HTMLInputElement | null;
    expect(input).toBeTruthy();

    await act(async () => {
      if (input) {
        const descriptor = Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, "value");
        descriptor?.set?.call(input, "Renamed timeout");
        input.dispatchEvent(new Event("input", { bubbles: true }));
      }
      await Promise.resolve();
    });

    const renameButton = Array.from(document.querySelectorAll("button")).find(
      (button) => button.textContent?.trim() === "Rename",
    );
    expect(renameButton).toBeTruthy();

    await act(async () => {
      renameButton?.dispatchEvent(new MouseEvent("click", { bubbles: true, cancelable: true }));
      await Promise.resolve();
    });

    expect(renameCaseMock).toHaveBeenCalledWith("case-3", "Renamed timeout");

    await act(async () => {
      root.unmount();
    });
    container.remove();
  });

  test("opens the quick create dialog from the composer controls", async () => {
    const container = document.createElement("div");
    document.body.append(container);
    const root = createRoot(container);

    await act(async () => {
      root.render(<CaseShell settings={buildSettings()} />);
    });

    const createButton = Array.from(container.querySelectorAll("button")).find(
      (button) => button.textContent?.trim() === "New case",
    );

    expect(createButton).toBeTruthy();

    await act(async () => {
      createButton?.dispatchEvent(new MouseEvent("click", { bubbles: true }));
    });

    expect(openCaseCreationPageMock).toHaveBeenCalledTimes(1);

    await act(async () => {
      root.unmount();
    });
    container.remove();
  });

  test("opens the options page from the composer settings button", async () => {
    const container = document.createElement("div");
    document.body.append(container);
    const root = createRoot(container);

    await act(async () => {
      root.render(<CaseShell settings={buildSettings()} />);
    });

    const settingsButton = Array.from(container.querySelectorAll("button")).find(
      (button) => button.getAttribute("aria-label") === "Open settings",
    );

    expect(settingsButton).toBeTruthy();

    await act(async () => {
      settingsButton?.dispatchEvent(new MouseEvent("click", { bubbles: true }));
    });

    expect(openGeneralSettingsPageMock).toHaveBeenCalledTimes(1);

    await act(async () => {
      root.unmount();
    });
    container.remove();
  });

  test("does not render a dedicated sidepanel header for case switching", () => {
    const html = renderToStaticMarkup(<CaseShell settings={buildSettings()} />);

    expect(html).not.toContain("border-b px-3 py-2");
    expect(html).toContain("Custom start slot");
    expect(html).not.toContain("Default end slot");
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
    expect(document.body.textContent).toContain("Thread mounted");
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

  test("does not render sidepanel case-management actions inside the thread chrome", async () => {
    const container = document.createElement("div");
    document.body.append(container);
    const root = createRoot(container);

    await act(async () => {
      root.render(<CaseShell settings={buildSettings()} />);
    });

    expect(container.textContent).not.toContain("Create case");
    expect(container.textContent).not.toContain("Rename");
    expect(container.textContent).not.toContain("Resolve");
    expect(container.textContent).not.toContain("Archive");
    expect(container.textContent).not.toContain("Delete");
    expect(openCaseCreationPageMock).not.toHaveBeenCalled();

    await act(async () => {
      root.unmount();
    });
    container.remove();
  });

  test("does not render plugin-settings actions inside the sidepanel thread", async () => {
    const container = document.createElement("div");
    document.body.append(container);
    const root = createRoot(container);

    await act(async () => {
      root.render(<CaseShell settings={buildSettings()} />);
    });

    expect(container.textContent).not.toContain("Plugin Settings");
    expect(openPluginSettingsPageMock).not.toHaveBeenCalled();

    await act(async () => {
      root.unmount();
    });
    container.remove();
  });

  test("does not render a custom no-cases empty state", () => {
    const settings = buildSettings();
    settings.activeCaseId = null;
    settings.cases = settings.cases.map((item) => ({
      ...item,
      archivedAt: item.archivedAt ?? "2026-03-17T10:20:00.000Z",
    }));

    const html = renderToStaticMarkup(<CaseShell settings={settings} />);

    expect(html).not.toContain("No cases yet");
    expect(html).not.toContain("Create case");
    expect(html).not.toContain("Thread mounted");
  });
});
