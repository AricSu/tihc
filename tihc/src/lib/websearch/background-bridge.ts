import type { SearchEngine } from "@/lib/chat/agent-types";
import { createWebSearchService } from "./background-service";
import type {
  WebSearchExtractPageMessage,
  WebSearchExtractPageResponse,
  WebSearchExtractSerpMessage,
  WebSearchExtractSerpResponse,
  WebSearchPortEvent,
  WebSearchRunRequest,
} from "./types";

type BrowserTabsApi = {
  create: (createProperties: { active?: boolean; pinned?: boolean; url?: string }) => Promise<{ id?: number; status?: string; url?: string }>;
  onUpdated: {
    addListener: (listener: (tabId: number, changeInfo: { status?: string }) => void) => void;
    removeListener: (listener: (tabId: number, changeInfo: { status?: string }) => void) => void;
  };
  sendMessage: <T>(tabId: number, message: WebSearchExtractSerpMessage | WebSearchExtractPageMessage) => Promise<T>;
  update: (
    tabId: number,
    updateProperties: { active?: boolean; url?: string },
  ) => Promise<{ id?: number; status?: string; url?: string }>;
};

type BrowserRuntimeApi = {
  onConnect: {
    addListener: (
      listener: (port: {
        disconnect: () => void;
        name?: string;
        onDisconnect?: { addListener?: (listener: () => void) => void };
        onMessage?: {
          addListener?: (listener: (message: WebSearchRunRequest) => void) => void;
        };
        postMessage: (message: WebSearchPortEvent) => void;
      }) => void,
    ) => void;
  };
};

type BrowserApi = {
  runtime?: BrowserRuntimeApi;
  tabs?: BrowserTabsApi;
};

function sleep(ms: number) {
  return new Promise<void>((resolve) => {
    globalThis.setTimeout(resolve, ms);
  });
}

function createTabCompleteWaiter(tabs: BrowserTabsApi, tabId: number) {
  let settled = false;
  let resolvePromise: (() => void) | null = null;

  const cleanup = () => {
    if (settled) return;
    settled = true;
    globalThis.clearTimeout(timeoutId);
    tabs.onUpdated.removeListener(listener);
    resolvePromise?.();
    resolvePromise = null;
  };

  const listener = (updatedTabId: number, changeInfo: { status?: string }) => {
    if (updatedTabId !== tabId || changeInfo.status !== "complete") return;
    cleanup();
  };

  const timeoutId = globalThis.setTimeout(() => {
    cleanup();
  }, 5_000);

  tabs.onUpdated.addListener(listener);

  return {
    cancel: cleanup,
    done: new Promise<void>((resolve) => {
      resolvePromise = resolve;
    }),
  };
}

async function sendTabMessageWithRetry<T>(
  tabs: BrowserTabsApi,
  tabId: number,
  message: WebSearchExtractSerpMessage | WebSearchExtractPageMessage,
): Promise<T> {
  for (let attempt = 0; attempt < 10; attempt += 1) {
    try {
      return await tabs.sendMessage<T>(tabId, message);
    } catch (error) {
      if (attempt === 9) throw error;
      await sleep(200);
    }
  }

  throw new Error("Unable to reach the websearch content script.");
}

function resolveBrowserApi(): BrowserApi {
  return (
    (globalThis as typeof globalThis & { browser?: BrowserApi }).browser ??
    (globalThis as typeof globalThis & { chrome?: BrowserApi }).chrome ??
    {}
  );
}

export function registerWebSearchBackgroundBridge(browserApi: BrowserApi = resolveBrowserApi()) {
  const tabs = browserApi.tabs;
  const runtime = browserApi.runtime;
  if (!tabs || !runtime) return;

  const service = createWebSearchService({
    activateWorkerTab: async (tabId) => {
      await tabs.update(tabId, { active: true });
    },
    createWorkerTab: async () => {
      const tab = await tabs.create({
        active: false,
        pinned: true,
        url: "about:blank",
      });
      if (typeof tab.id !== "number") {
        throw new Error("Failed to create the TIHC Search Worker tab.");
      }
      return tab.id;
    },
    extractPage: async (tabId) =>
      await sendTabMessageWithRetry<WebSearchExtractPageResponse>(tabs, tabId, {
        type: "tihc:websearch.extract-page",
      }),
    extractSerp: async (tabId, engine) =>
      await sendTabMessageWithRetry<WebSearchExtractSerpResponse>(tabs, tabId, {
        engine,
        type: "tihc:websearch.extract-serp",
      }),
    navigateWorkerTab: async (tabId, url) => {
      const waitForLoad = createTabCompleteWaiter(tabs, tabId);
      const updatedTab = await tabs.update(tabId, { active: false, url });
      if (updatedTab.status === "complete" && updatedTab.url === url) {
        waitForLoad.cancel();
        return;
      }
      await waitForLoad.done;
    },
    now: () => new Date().toISOString(),
    sleep,
  });

  runtime.onConnect.addListener((port) => {
    if (port.name !== "tihc:websearch.run") return;

    port.onMessage?.addListener?.(async (message) => {
      if (message.type !== "tihc:websearch.run") return;

      try {
        const bundle = await service.run(
          {
            primaryEngine: message.primaryEngine,
            query: message.query,
          },
          (status) => port.postMessage({ text: status, type: "status" }),
        );
        port.postMessage({ bundle, type: "result" });
      } catch (error) {
        port.postMessage({
          message: error instanceof Error ? error.message : String(error),
          type: "error",
        });
      }
    });
  });
}
