import { act } from "react";
import { createRoot } from "react-dom/client";
import { renderToStaticMarkup } from "react-dom/server";
import { beforeEach, describe, expect, test, vi } from "vitest";
import {
  ANONYMOUS_LOCAL_STORAGE_LIMIT_BYTES,
  formatStorageBytes,
} from "@/lib/app/anonymous-local-case-limit";

const {
  createCaseMock,
  deleteCaseMock,
  ensureGoogleAuthSessionMock,
  getAppSettingsSnapshotMock,
  listDashboardCasesMock,
  listUsageSummaryMock,
  listUsageTimeseriesMock,
  subscribeAppSettingsMock,
  syncCloudCasesIfNeededMock,
  updateAssistantReplyFontSizeMock,
} = vi.hoisted(() => ({
  createCaseMock: vi.fn(),
  deleteCaseMock: vi.fn(),
  ensureGoogleAuthSessionMock: vi.fn(async () => null),
  getAppSettingsSnapshotMock: vi.fn(),
  listDashboardCasesMock: vi.fn(),
  listUsageSummaryMock: vi.fn(),
  listUsageTimeseriesMock: vi.fn(),
  subscribeAppSettingsMock: vi.fn(() => () => {}),
  syncCloudCasesIfNeededMock: vi.fn(),
  updateAssistantReplyFontSizeMock: vi.fn(),
}));

vi.mock("@/lib/app/runtime", () => ({
  clearGoogleAuth: vi.fn(),
  createCase: createCaseMock,
  deleteCase: deleteCaseMock,
  ensureGoogleAuthSession: ensureGoogleAuthSessionMock,
  getAppSettingsSnapshot: getAppSettingsSnapshotMock,
  refreshGoogleAuth: vi.fn(),
  setGoogleAuth: vi.fn(),
  syncCloudCasesIfNeeded: syncCloudCasesIfNeededMock,
  subscribeAppSettings: subscribeAppSettingsMock,
  updateAssistantReplyFontSize: updateAssistantReplyFontSizeMock,
  updateInstalledPluginConfig: vi.fn(),
}));

vi.mock("@/lib/app/cases-api", () => ({
  listDashboardCases: listDashboardCasesMock,
}));

vi.mock("@/lib/app/cloud-usage", () => ({
  getStoredUsageSummary: listUsageSummaryMock,
  getStoredUsageTimeseries: listUsageTimeseriesMock,
}));

(globalThis as typeof globalThis & { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT =
  true;

Object.defineProperty(window, "matchMedia", {
  configurable: true,
  writable: true,
  value: vi.fn().mockImplementation((query: string) => ({
    matches: false,
    media: query,
    onchange: null,
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    addListener: vi.fn(),
    removeListener: vi.fn(),
    dispatchEvent: vi.fn(),
  })),
});

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

async function renderInDom(element: React.ReactNode): Promise<() => Promise<void>> {
  const container = document.createElement("div");
  document.body.append(container);
  const root = createRoot(container);

  await act(async () => {
    root.render(element);
  });

  return async () => {
    await act(async () => {
      root.unmount();
    });
    container.remove();
  };
}

function buildSettings() {
  return {
    activeCaseId: "case-1",
    assistantReplyFontSize: "default",
    analyticsConsent: "unknown",
    cloudSync: {
      importedClientId: null,
      lastHydratedAt: null,
      mode: "local",
    },
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
    cases: [],
    googleAuth: null,
  };
}

function buildDashboardCases() {
  return [
    {
      id: "case-1",
      title: "Primary case from API",
      status: "Investigating",
      priority: "Hot",
      channel: "tidb.ai",
      updatedAt: "2026-04-14T12:00:00.000Z",
      executionTarget: "tidb.ai",
      owner: "TIHC",
      summary: "Fetched through the dashboard cases API.",
      signals: [
        "Customer checkout fails after card entry.",
        "The latest assistant note points to auth refresh latency.",
      ],
      messages: [
        {
          role: "operator",
          text: "Customer checkout fails after card entry.",
        },
        {
          role: "tihc",
          text: "The latest assistant note points to auth refresh latency.",
        },
      ],
    },
  ];
}

async function flushAsyncWork() {
  await act(async () => {
    await Promise.resolve();
  });
}

describe("dashboard shell", () => {
  beforeEach(() => {
    installMockStorage();
    clearBrowserStorage();
    getAppSettingsSnapshotMock.mockReturnValue(buildSettings());
    listDashboardCasesMock.mockResolvedValue(buildDashboardCases());
    listUsageSummaryMock.mockResolvedValue(null);
    listUsageTimeseriesMock.mockResolvedValue([]);
  });

  test("renders dashboard cases without the sidepanel-style case snapshot preview", async () => {
    const { DashboardShell } = await import("./dashboard-shell");
    const cleanup = await renderInDom(<DashboardShell />);

    await flushAsyncWork();

    expect(document.body.textContent).toContain("Acme Inc.");
    expect(document.body.textContent).toContain("Plugins");
    expect(document.body.textContent).toContain("Skills");
    expect(document.body.textContent).toContain("Cases");
    expect(document.body.textContent).toContain("Primary case from API");
    expect(document.body.textContent).toContain("Fetched through the dashboard cases API.");
    expect(document.body.textContent).not.toContain(
      "Select a case to inspect the latest snapshot without opening the sidepanel.",
    );
    expect(document.body.textContent).not.toContain("Recent signals");
    expect(document.body.textContent).not.toContain("Case preview");
    expect(document.body.textContent).not.toContain("Owner");
    expect(document.body.textContent).not.toContain("Case ID");
    expect(document.body.textContent).not.toContain("Open cases");
    expect(document.body.textContent).not.toContain("Watching");
    expect(document.body.textContent).not.toContain("Closed cases");
    expect(document.body.textContent).not.toContain("Total Revenue");
    expect(document.body.textContent).not.toContain("Total Visitors");
    expect(document.body.textContent).not.toContain("Lifecycle");
    expect(document.body.textContent).not.toContain("Analytics");
    expect(document.body.textContent).not.toContain("Projects");
    expect(document.body.textContent).not.toContain("Team");
    expect(document.body.textContent).not.toContain("Search");

    await cleanup();
  });

  test("renders the create-case dialog when the options page is opened in create-case mode", async () => {
    const { DashboardShell } = await import("./dashboard-shell");
    const cleanup = await renderInDom(<DashboardShell initialDialog="create-case" />);

    expect(document.body.textContent).toContain("Create case");
    expect(document.body.textContent).toContain("Case title");
    expect(document.body.textContent).toContain("Start the investigation");

    await cleanup();
  });

  test("blocks anonymous usage with a delete dialog once browser storage usage exceeds the limit", async () => {
    const oversizedText = "x".repeat(Math.ceil(ANONYMOUS_LOCAL_STORAGE_LIMIT_BYTES / 2));
    window.localStorage.setItem(
      "tihc_local_history_v2:case-1",
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
    const cases = [
      {
        id: "case-1",
        title: "Browser case 1",
        pluginId: "tidb.ai" as const,
        activityState: "active" as const,
        resolvedAt: null,
        archivedAt: null,
        createdAt: "2026-04-14T12:00:00.000Z",
        updatedAt: "2026-04-14T12:00:00.000Z",
      },
    ];
    getAppSettingsSnapshotMock.mockReturnValue({
      ...buildSettings(),
      activeCaseId: cases[0]?.id ?? null,
      cases,
    });
    vi.stubGlobal("confirm", vi.fn(() => true));

    const { DashboardShell } = await import("./dashboard-shell");
    const cleanup = await renderInDom(<DashboardShell />);

    await flushAsyncWork();

    expect(document.body.textContent).toContain("Delete local cases to continue");
    expect(document.body.textContent).toContain(
      "Anonymous mode is limited by browser storage usage, not case count.",
    );
    expect(document.body.textContent).toContain(formatStorageBytes(ANONYMOUS_LOCAL_STORAGE_LIMIT_BYTES));
    expect(document.body.textContent).toContain("Browser case 1");

    const deleteButton = Array.from(document.querySelectorAll("button")).find((button) =>
      button.getAttribute("aria-label") === "Delete case Browser case 1",
    );

    expect(deleteButton).toBeTruthy();

    await act(async () => {
      deleteButton?.dispatchEvent(new MouseEvent("click", { bubbles: true }));
    });

    expect(deleteCaseMock).toHaveBeenCalledWith("case-1");

    await cleanup();
  });

  test("opens the create-case dialog from the options page query string", async () => {
    window.history.replaceState({}, "", "?dialog=create-case");
    const { DashboardShell } = await import("./dashboard-shell");
    const cleanup = await renderInDom(<DashboardShell />);

    expect(document.body.textContent).toContain("Create case");
    expect(document.body.textContent).toContain("Case title");

    await cleanup();
  });

  test("renders the plugin marketplace inside the dashboard shell when the plugin section is active", async () => {
    const { DashboardShell } = await import("./dashboard-shell");

    const html = renderToStaticMarkup(<DashboardShell initialSection="plugin" />);

    expect(html).toContain("Acme Inc.");
    expect(html).toContain("Plugins");
    expect(html).toContain("Marketplace");
    expect(html).not.toContain("Manage");
    expect(html).not.toContain("Make TIHC work your way");
    expect(html).not.toContain("Total Revenue");
  });

  test("renders a dedicated skills workspace when the skills section is active", async () => {
    const { DashboardShell } = await import("./dashboard-shell");

    const html = renderToStaticMarkup(<DashboardShell initialSection={"skills" as never} />);

    expect(html).toContain("Skills");
    expect(html).toContain("Create skill");
    expect(html).toContain("No skills yet");
    expect(html).not.toContain("Your skills");
    expect(html).not.toContain("Choose a saved skill to refine it");
    expect(html).not.toContain("0 listed");
    expect(html).not.toContain("Reusable instructions");
    expect(html).not.toContain("0 skills");
    expect(html).not.toContain("Build a personal library");
    expect(html).not.toContain("Personal library");
    expect(html).not.toContain("Future turns");
    expect(html).not.toContain("Ready state");
    expect(html).not.toContain("Empty library");
    expect(html).not.toContain("Editor");
    expect(html).not.toContain("Write, preview, and split");
    expect(html).not.toContain("Markdown first");
    expect(html).not.toContain("Marketplace");
    expect(html).not.toContain("TiHC Native Supported");
    expect(html).not.toContain("Case preview");
  });

  test("renders a dedicated llm settings workspace when the llm section is active", async () => {
    const { DashboardShell } = await import("./dashboard-shell");

    const html = renderToStaticMarkup(<DashboardShell initialSection="llm" />);

    expect(html).toContain("LLM");
    expect(html).toContain("Per-user settings");
    expect(html).toContain("User LLM Settings");
    expect(html).not.toContain("Marketplace");
    expect(html).not.toContain("Case preview");
  });

  test("renders a usage workspace without the token detail list when the usage section is active", async () => {
    const { DashboardShell } = await import("./dashboard-shell");

    const html = renderToStaticMarkup(<DashboardShell initialSection="usage" />);

    expect(html).toContain("Usage");
    expect(html).toContain("Overview");
    expect(html).toContain("Total Tokens");
    expect(html).toContain("Token activity");
    expect(html).not.toContain("Model usage");
    expect(html).not.toContain("Case 417 / gpt-5.4-mini");
    expect(html).not.toContain("Case preview");
    expect(html).not.toContain("Total Revenue");
  });

  test("switches sections inline when the plugins nav item is clicked", async () => {
    window.history.replaceState({}, "", "?");

    const { DashboardShell } = await import("./dashboard-shell");
    const cleanup = await renderInDom(<DashboardShell initialSection="dashboard" />);

    const pluginsLink = Array.from(document.querySelectorAll("a")).find(
      (link) => link.textContent?.trim() === "Plugins",
    );

    expect(document.body.textContent).toContain("Cases");
    expect(document.body.textContent).toContain("Primary case from API");
    expect(pluginsLink).toBeTruthy();

    await act(async () => {
      pluginsLink?.dispatchEvent(new MouseEvent("click", { bubbles: true, cancelable: true }));
    });

    expect(window.location.search).toBe("?section=plugin");
    expect(document.body.textContent).toContain("Plugins");
    expect(document.body.textContent).toContain("Marketplace");
    expect(document.body.textContent).not.toContain("Search plugins");
    expect(document.body.textContent).not.toContain("Recent activity");
    expect(document.body.textContent).not.toContain(
      "Select a case to inspect the latest snapshot without opening the sidepanel.",
    );

    await cleanup();
  });

  test("switches sections inline when the llm nav item is clicked", async () => {
    window.history.replaceState({}, "", "?");

    const { DashboardShell } = await import("./dashboard-shell");
    const cleanup = await renderInDom(<DashboardShell initialSection="dashboard" />);

    const llmLink = Array.from(document.querySelectorAll("a")).find(
      (link) => link.textContent?.trim() === "LLM",
    );

    expect(llmLink).toBeTruthy();

    await act(async () => {
      llmLink?.dispatchEvent(new MouseEvent("click", { bubbles: true }));
    });

    expect(document.body.textContent).toContain("User LLM Settings");
    expect(window.location.search).toBe("?section=llm");

    await cleanup();
  });

  test("switches sections inline when the usage nav item is clicked", async () => {
    window.history.replaceState({}, "", "?");

    const { DashboardShell } = await import("./dashboard-shell");
    const cleanup = await renderInDom(<DashboardShell initialSection="dashboard" />);

    const usageLink = Array.from(document.querySelectorAll("a")).find(
      (link) => link.textContent?.trim() === "Usage",
    );

    expect(document.body.textContent).toContain("Cases");
    expect(document.body.textContent).toContain("Primary case from API");
    expect(usageLink).toBeTruthy();

    await act(async () => {
      usageLink?.dispatchEvent(new MouseEvent("click", { bubbles: true, cancelable: true }));
    });

    expect(window.location.search).toBe("?section=usage");
    expect(document.body.textContent).toContain("Usage");
    expect(document.body.textContent).toContain("Overview");
    expect(document.body.textContent).toContain("Total Tokens");
    expect(document.body.textContent).not.toContain("Model usage");
    expect(document.body.textContent).not.toContain("Case 417 / gpt-5.4-mini");
    expect(document.body.textContent).not.toContain("Case preview");
    expect(document.body.textContent).not.toContain("Recent activity");

    await cleanup();
  });

  test("switches sections inline when the skills nav item is clicked", async () => {
    window.history.replaceState({}, "", "?");

    const { DashboardShell } = await import("./dashboard-shell");
    const cleanup = await renderInDom(<DashboardShell initialSection="dashboard" />);

    const skillsLink = Array.from(document.querySelectorAll("a")).find(
      (link) => link.textContent?.trim() === "Skills",
    );

    expect(document.body.textContent).toContain("Cases");
    expect(document.body.textContent).toContain("Primary case from API");
    expect(skillsLink).toBeTruthy();

    await act(async () => {
      skillsLink?.dispatchEvent(new MouseEvent("click", { bubbles: true, cancelable: true }));
    });

    expect(window.location.search).toBe("?section=skills");
    expect(document.body.textContent).toContain("Skills");
    expect(document.body.textContent).toContain("Library");
    expect(document.body.textContent).not.toContain("Marketplace");
    expect(document.body.textContent).not.toContain("Recent activity");

    await cleanup();
  });
});
