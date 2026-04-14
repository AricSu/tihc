import { beforeEach, describe, expect, test, vi } from "vitest";

type MockTab = {
  id?: number;
};

function installBrowserMock(existingTabs: MockTab[] = []) {
  const create = vi.fn<(options: { url: string }) => Promise<void>>().mockResolvedValue();
  const query = vi
    .fn<(options: { url?: string }) => Promise<MockTab[]>>()
    .mockResolvedValue(existingTabs);
  const update = vi
    .fn<(tabId: number, options: { active?: boolean; url?: string }) => Promise<void>>()
    .mockResolvedValue();
  const getURL = vi.fn((path: string) => `extension://test/${path}`);
  Object.defineProperty(globalThis, "browser", {
    configurable: true,
    value: {
      tabs: {
        create,
        query,
        update,
      },
      runtime: {
        getURL,
      },
    },
  });
  return { create, getURL, query, update };
}

async function loadModule() {
  vi.resetModules();
  return import("./settings-page");
}

describe("settings page navigation", () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  test("opens the plugin settings page in a tab", async () => {
    const browserMock = installBrowserMock();
    const settingsPage = await loadModule();

    await settingsPage.openPluginSettingsPage();

    expect(browserMock.getURL).toHaveBeenCalledWith("options.html?section=plugin");
    expect(browserMock.create).toHaveBeenCalledWith({
      url: "extension://test/options.html?section=plugin",
    });
  });

  test("opens the llm settings page in a tab", async () => {
    const browserMock = installBrowserMock();
    const settingsPage = await loadModule();

    await settingsPage.openLlmSettingsPage();

    expect(browserMock.getURL).toHaveBeenCalledWith("options.html?section=llm");
    expect(browserMock.create).toHaveBeenCalledWith({
      url: "extension://test/options.html?section=llm",
    });
  });

  test("opens the extension options page in create-case mode", async () => {
    const browserMock = installBrowserMock();
    const settingsPage = await loadModule();

    await settingsPage.openCaseCreationPage();

    expect(browserMock.getURL).toHaveBeenCalledWith("options.html?dialog=create-case");
    expect(browserMock.create).toHaveBeenCalledWith({
      url: "extension://test/options.html?dialog=create-case",
    });
  });

  test("reuses an existing options tab instead of creating a new one", async () => {
    const browserMock = installBrowserMock([{ id: 42 }]);
    const settingsPage = await loadModule();

    await settingsPage.openPluginSettingsPage();

    expect(browserMock.query).toHaveBeenCalledWith({
      url: "extension://test/options.html*",
    });
    expect(browserMock.update).toHaveBeenCalledWith(42, {
      active: true,
      url: "extension://test/options.html?section=plugin",
    });
    expect(browserMock.create).not.toHaveBeenCalled();
  });
});
