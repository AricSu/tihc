import { act } from "react";
import { createRoot } from "react-dom/client";
import { beforeEach, describe, expect, test, vi } from "vitest";

const {
  createCaseMock,
  getAppSettingsSnapshotMock,
  setActiveCaseIdMock,
  subscribeAppSettingsMock,
  updateAssistantReplyFontSizeMock,
} = vi.hoisted(() => ({
  createCaseMock: vi.fn(),
  getAppSettingsSnapshotMock: vi.fn(),
  setActiveCaseIdMock: vi.fn(),
  subscribeAppSettingsMock: vi.fn(() => () => {}),
  updateAssistantReplyFontSizeMock: vi.fn(),
}));

vi.mock("@/lib/app/runtime", () => ({
  clearGoogleAuth: vi.fn(),
  createCase: createCaseMock,
  ensureGoogleAuthSession: vi.fn(async () => null),
  getAppSettingsSnapshot: getAppSettingsSnapshotMock,
  refreshGoogleAuth: vi.fn(),
  setActiveCaseId: setActiveCaseIdMock,
  setGoogleAuth: vi.fn(),
  subscribeAppSettings: subscribeAppSettingsMock,
  syncCloudCasesIfNeeded: vi.fn(),
  updateAssistantReplyFontSize: updateAssistantReplyFontSizeMock,
  updateInstalledPluginConfig: vi.fn(),
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
    cases: [
      {
        id: "case-1",
        title: "Primary case",
        pluginId: "tidb.ai",
        activityState: "active",
        resolvedAt: null,
        archivedAt: null,
        createdAt: "2026-03-17T10:00:00.000Z",
        updatedAt: "2026-03-17T10:00:00.000Z",
      },
    ],
    googleAuth: null,
  };
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

describe("options entrypoint", () => {
  beforeEach(() => {
    window.history.replaceState({}, "", "?");
    getAppSettingsSnapshotMock.mockReturnValue(buildSettings());
  });

  test("opens the create-case dialog when the URL requests it", async () => {
    window.history.replaceState({}, "", "?dialog=create-case");

    const { default: Options } = await import("./Options");
    const cleanup = await renderInDom(<Options />);

    expect(document.body.textContent).toContain("Create case");
    expect(document.body.textContent).toContain("Case title");
    expect(document.body.textContent).toContain("Start the investigation");

    await cleanup();
  });

  test("renders the plugin marketplace page when the section is plugin", async () => {
    window.history.replaceState({}, "", "?section=plugin");

    const { default: Options } = await import("./Options");
    const cleanup = await renderInDom(<Options />);

    expect(document.body.textContent).toContain("tihc");
    expect(document.body.textContent).toContain("Plugins");
    expect(document.body.textContent).toContain("Marketplace");
    expect(document.body.textContent).toContain("Installed");
    expect(document.body.textContent).toContain("TiHC Native Supported");
    expect(document.body.textContent).not.toContain("Featured");
    expect(document.body.textContent).not.toContain("Coding");
    expect(document.body.textContent).toContain("tidb.ai");
    expect(document.body.textContent).not.toContain("Browser Automation");
    expect(document.body.textContent).not.toContain("OpenAI Responses");
    expect(document.body.textContent).not.toContain("No plugins match the current filters.");
    expect(document.body.textContent).not.toContain("Make TIHC work your way");
    expect(document.body.textContent).not.toContain("Built by Anyone");

    await cleanup();
  });

  test("renders the dedicated llm settings workspace when the section is llm", async () => {
    window.history.replaceState({}, "", "?section=llm");

    const { default: Options } = await import("./Options");
    const cleanup = await renderInDom(<Options />);

    expect(document.body.textContent).toContain("tihc");
    expect(document.body.textContent).toContain("LLM");
    expect(document.body.textContent).toContain("User LLM Settings");
    expect(document.body.textContent).toContain("Save LLM Settings");

    await cleanup();
  });

  test("renders the dedicated skills workspace when the section is skills", async () => {
    window.history.replaceState({}, "", "?section=skills");

    const { default: Options } = await import("./Options");
    const cleanup = await renderInDom(<Options />);

    expect(document.body.textContent).toContain("tihc");
    expect(document.body.textContent).toContain("Skills");
    expect(document.body.textContent).toContain("Create skill");
    expect(document.body.textContent).toContain("No skills yet");
    expect(document.body.textContent).not.toContain("Your skills");
    expect(document.body.textContent).not.toContain("Choose a saved skill to refine it");
    expect(document.body.textContent).not.toContain("0 listed");
    expect(document.body.textContent).not.toContain("Reusable instructions");
    expect(document.body.textContent).not.toContain("0 skills");
    expect(document.body.textContent).not.toContain("Build a personal library");
    expect(document.body.textContent).not.toContain("Personal library");
    expect(document.body.textContent).not.toContain("Future turns");
    expect(document.body.textContent).not.toContain("Ready state");
    expect(document.body.textContent).not.toContain("Empty library");
    expect(document.body.textContent).not.toContain("Editor");
    expect(document.body.textContent).not.toContain("Write, preview, and split");
    expect(document.body.textContent).not.toContain("Markdown first");
    expect(document.body.textContent).not.toContain("Marketplace");

    await cleanup();
  });

  test("renders the aric-ai style skills editor when the URL opens a new draft", async () => {
    window.history.replaceState({}, "", "?section=skills&editor=new");

    const { default: Options } = await import("./Options");
    const cleanup = await renderInDom(<Options />);

    expect(document.body.textContent).toContain("tihc");
    expect(document.body.textContent).toContain("Create skill");
    expect(document.body.textContent).toContain("Mode");
    expect(document.body.textContent).toContain("Write");
    expect(document.body.textContent).toContain("Preview");
    expect(document.body.textContent).toContain("Split view");
    expect(document.body.textContent).toContain("Reset draft");
    expect(document.body.textContent).not.toContain("How It Works");
    expect(document.body.textContent).not.toContain(
      "Keep each skill focused so the agent can recognize when to apply it.",
    );
    expect(document.body.textContent).not.toContain("Future turns");
    expect(document.body.textContent).not.toContain(
      "The agent can bring this skill into future turns when the request matches the instruction and description.",
    );
    expect(document.body.textContent).not.toContain("Best results");
    expect(document.body.textContent).not.toContain(
      "Use one skill per working style, deliverable pattern, or job to be done.",
    );
    expect(document.body.textContent).not.toContain("Markdown format");
    expect(document.body.textContent).not.toContain(
      "Write in markdown, then switch to preview or split view to refine the final skill.",
    );

    const editor = document.querySelector("textarea[aria-label='Markdown editor']");
    expect(editor).toBeTruthy();

    await cleanup();
  });

  test("creates a skill without an explicit id and auto-generates one from the name and timestamp", async () => {
    vi.useFakeTimers();
    try {
      vi.setSystemTime(new Date("2026-04-15T12:34:56.789Z"));
      window.history.replaceState({}, "", "?section=skills&editor=new");

      const { default: Options } = await import("./Options");
      const cleanup = await renderInDom(<Options />);

      const editor = document.querySelector("textarea[aria-label='Markdown editor']");
      expect(editor).toBeInstanceOf(HTMLTextAreaElement);
      expect((editor as HTMLTextAreaElement).value).not.toContain("id:");

      await act(async () => {
        if (editor instanceof HTMLTextAreaElement) {
          const descriptor = Object.getOwnPropertyDescriptor(HTMLTextAreaElement.prototype, "value");
          descriptor?.set?.call(
            editor,
            [
              "---",
              "name: Briefing Writer",
              "description: Generates structured briefings.",
              "---",
              "# Briefing Writer",
              "Focus on concise updates.",
            ].join("\n"),
          );
          editor.dispatchEvent(new Event("input", { bubbles: true }));
          editor.dispatchEvent(new Event("change", { bubbles: true }));
        }
        await Promise.resolve();
      });

      const createSkillButton = Array.from(document.querySelectorAll("button")).find(
        (button) => button.textContent?.trim() === "Create skill",
      );
      expect(createSkillButton).toBeTruthy();

      await act(async () => {
        createSkillButton?.dispatchEvent(new MouseEvent("click", { bubbles: true, cancelable: true }));
        await Promise.resolve();
        await Promise.resolve();
      });

      expect(window.location.search).toContain("section=skills");
      expect(window.location.search).toContain("savedSkill=1");
      expect(window.location.search).toContain("editor=briefing-writer-1776256496789");

      await cleanup();
    } finally {
      vi.useRealTimers();
    }
  });

  test("falls back to the dashboard for unknown sections", async () => {
    window.history.replaceState({}, "", "?section=targets");

    const { default: Options } = await import("./Options");
    const cleanup = await renderInDom(<Options />);

    expect(document.body.textContent).toContain("tihc");
    expect(document.body.textContent).toContain("Cases");
    expect(document.body.textContent).toContain("Primary case");
    expect(document.body.textContent).not.toContain(
      "Select a case to inspect the latest snapshot without opening the sidepanel.",
    );
    expect(document.body.textContent).not.toContain("Case preview");
    expect(document.body.textContent).not.toContain("No thread activity yet.");
    expect(document.body.textContent).not.toContain("Open cases");
    expect(document.body.textContent).not.toContain("Total Revenue");
    expect(document.body.textContent).not.toContain("Marketplace");

    await cleanup();
  });
});
