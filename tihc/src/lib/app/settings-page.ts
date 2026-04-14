export type SettingsSection = "general" | "plugin" | "skills" | "llm" | "about";

type MaybePromise<T> = T | Promise<T>;
type TabInfo = { id?: number };

async function reuseOptionsTab(
  tabs:
    | {
        query?: (options: { url?: string }) => MaybePromise<TabInfo[]>;
        update?: (
          tabId: number,
          options: { active?: boolean; url?: string },
        ) => MaybePromise<void>;
      }
    | undefined,
  targetUrl: string,
): Promise<boolean> {
  if (!tabs?.query || !tabs.update) return false;

  const baseUrl = targetUrl.split("?")[0] ?? targetUrl;
  const existingTabs = await tabs.query({ url: `${baseUrl}*` });
  const existingTab = existingTabs.find((tab) => typeof tab.id === "number");
  if (existingTab?.id == null) return false;

  await tabs.update(existingTab.id, {
    active: true,
    url: targetUrl,
  });
  return true;
}

async function openOptionsPage(path: string): Promise<void> {
  const browserApi = (globalThis as typeof globalThis & {
    browser?: {
      tabs?: {
        create?: (options: { url: string }) => Promise<void>;
        query?: (options: { url?: string }) => Promise<TabInfo[]>;
        update?: (tabId: number, options: { active?: boolean; url?: string }) => Promise<void>;
      };
      runtime?: {
        getURL?: (path: string) => string;
      };
    };
    chrome?: {
      tabs?: {
        create?: (options: { url: string }) => void;
        query?: (options: { url?: string }) => MaybePromise<TabInfo[]>;
        update?: (tabId: number, options: { active?: boolean; url?: string }) => MaybePromise<void>;
      };
      runtime?: {
        getURL?: (path: string) => string;
      };
    };
  }).browser;

  const browserUrl = browserApi?.runtime?.getURL?.(path);
  if (browserUrl && browserApi?.tabs?.create) {
    if (await reuseOptionsTab(browserApi.tabs, browserUrl)) return;
    await browserApi.tabs.create({ url: browserUrl });
    return;
  }

  const chromeApi = (globalThis as typeof globalThis & {
    chrome?: {
      tabs?: {
        create?: (options: { url: string }) => void;
        query?: (options: { url?: string }) => MaybePromise<TabInfo[]>;
        update?: (tabId: number, options: { active?: boolean; url?: string }) => MaybePromise<void>;
      };
      runtime?: {
        getURL?: (path: string) => string;
      };
    };
  }).chrome;

  const chromeUrl = chromeApi?.runtime?.getURL?.(path);
  if (chromeUrl && chromeApi?.tabs?.create) {
    if (await reuseOptionsTab(chromeApi.tabs, chromeUrl)) return;
    chromeApi.tabs.create({ url: chromeUrl });
    return;
  }

  window.open(`/${path}`, "_blank", "noopener,noreferrer");
}

export async function openCaseCreationPage(): Promise<void> {
  await openOptionsPage("options.html?dialog=create-case");
}

export async function openPluginSettingsPage(): Promise<void> {
  await openOptionsPage("options.html?section=plugin");
}

export async function openSkillsSettingsPage(): Promise<void> {
  await openOptionsPage("options.html?section=skills");
}

export async function openLlmSettingsPage(): Promise<void> {
  await openOptionsPage("options.html?section=llm");
}
