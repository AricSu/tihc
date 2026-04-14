import type { SearchEngine } from "@/lib/chat/agent-types";
import {
  resolveSearchAttemptOrder,
  shouldExtractPageContent,
} from "./policy";
import type {
  WebSearchBundle,
  WebSearchExtractPageResponse,
  WebSearchExtractSerpResponse,
  WebSearchResult,
} from "./types";

const BAIDU_VERIFICATION_MAX_POLLS = 120;
const BAIDU_VERIFICATION_POLL_INTERVAL_MS = 1_000;

type WebSearchStatusHandler = (status: string) => void;

type BackgroundServiceDeps = {
  activateWorkerTab: (tabId: number) => Promise<void>;
  createWorkerTab: () => Promise<number>;
  extractPage: (tabId: number) => Promise<WebSearchExtractPageResponse>;
  extractSerp: (tabId: number, engine: SearchEngine) => Promise<WebSearchExtractSerpResponse>;
  navigateWorkerTab: (tabId: number, url: string) => Promise<void>;
  now: () => string;
  sleep: (ms: number) => Promise<void>;
};

export function buildSearchUrl(engine: SearchEngine, query: string): string {
  const encodedQuery = encodeURIComponent(query);
  if (engine === "duckduckgo") {
    return `https://html.duckduckgo.com/html/?q=${encodedQuery}`;
  }
  if (engine === "bing") {
    return `https://www.bing.com/search?q=${encodedQuery}`;
  }
  return `https://www.baidu.com/s?wd=${encodedQuery}`;
}

export function createWebSearchService(deps: BackgroundServiceDeps) {
  let workerTabId: number | null = null;

  const ensureWorkerTab = async () => {
    if (workerTabId !== null) return workerTabId;
    workerTabId = await deps.createWorkerTab();
    return workerTabId;
  };

  const waitForBaiduVerification = async (tabId: number) => {
    await deps.activateWorkerTab(tabId);

    for (let attempt = 0; attempt < BAIDU_VERIFICATION_MAX_POLLS; attempt += 1) {
      await deps.sleep(BAIDU_VERIFICATION_POLL_INTERVAL_MS);
      const extraction = await deps.extractSerp(tabId, "baidu");
      if (!extraction.verificationRequired) {
        return extraction;
      }
    }

    throw new Error("Baidu verification timed out.");
  };

  const enrichWithPageContent = async (
    tabId: number,
    query: string,
    results: WebSearchResult[],
    onStatus: WebSearchStatusHandler,
  ) => {
    if (!shouldExtractPageContent(query, results)) {
      return results;
    }

    onStatus("Opening result pages");

    const enrichedResults = [...results];
    for (const result of enrichedResults.slice(0, 2)) {
      try {
        await deps.navigateWorkerTab(tabId, result.url);
        const page = await deps.extractPage(tabId);
        if (page.excerpt) {
          result.pageExcerpt = page.excerpt;
        }
      } catch {
        // Best effort enrichment only.
      }
    }

    return enrichedResults;
  };

  return {
    async run(
      request: { primaryEngine: SearchEngine; query: string },
      onStatus: WebSearchStatusHandler,
    ): Promise<WebSearchBundle> {
      const tabId = await ensureWorkerTab();
      const attempts = resolveSearchAttemptOrder(request.primaryEngine);

      for (const [index, engine] of attempts.entries()) {
        onStatus(`Searching web with ${engine}`);
        await deps.navigateWorkerTab(tabId, buildSearchUrl(engine, request.query));

        let extraction = await deps.extractSerp(tabId, engine);
        if (engine === "baidu" && extraction.verificationRequired) {
          onStatus("Waiting for Baidu verification");
          extraction = await waitForBaiduVerification(tabId);
        }

        const results = extraction.results.slice(0, 5);
        if (!results.length) {
          const nextEngine = attempts[index + 1];
          if (nextEngine) {
            onStatus(`Falling back to ${nextEngine}`);
          }
          continue;
        }

        return {
          engine,
          query: request.query,
          results: await enrichWithPageContent(tabId, request.query, results, onStatus),
          searchedAt: deps.now(),
        };
      }

      return {
        engine: attempts.at(-1) ?? request.primaryEngine,
        query: request.query,
        results: [],
        searchedAt: deps.now(),
      };
    },
  };
}
