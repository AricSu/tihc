import { detectBaiduVerificationPage, extractPageExcerpt, extractSerpResults } from "./dom-extract";
import type {
  WebSearchExtractPageMessage,
  WebSearchExtractPageResponse,
  WebSearchExtractSerpMessage,
  WebSearchExtractSerpResponse,
} from "./types";

type BrowserApi = {
  runtime?: {
    onMessage?: {
      addListener?: (
        listener: (
          message: WebSearchExtractSerpMessage | WebSearchExtractPageMessage,
        ) =>
          | Promise<WebSearchExtractPageResponse | WebSearchExtractSerpResponse | undefined>
          | WebSearchExtractPageResponse
          | WebSearchExtractSerpResponse
          | undefined,
      ) => void;
    };
  };
};

function sleep(ms: number) {
  return new Promise<void>((resolve) => {
    globalThis.setTimeout(resolve, ms);
  });
}

async function waitForSerpResults(engine: WebSearchExtractSerpMessage["engine"]) {
  for (let attempt = 0; attempt < 10; attempt += 1) {
    const verificationRequired = engine === "baidu" && detectBaiduVerificationPage(document);
    const results = extractSerpResults(document, engine);
    if (verificationRequired || results.length) {
      return {
        results,
        verificationRequired,
      } satisfies WebSearchExtractSerpResponse;
    }
    await sleep(250);
  }

  return {
    results: extractSerpResults(document, engine),
    verificationRequired: engine === "baidu" && detectBaiduVerificationPage(document),
  } satisfies WebSearchExtractSerpResponse;
}

function resolveBrowserApi(): BrowserApi {
  return (
    (globalThis as typeof globalThis & { browser?: BrowserApi }).browser ??
    (globalThis as typeof globalThis & { chrome?: BrowserApi }).chrome ??
    {}
  );
}

export function registerWebSearchContentBridge(browserApi: BrowserApi = resolveBrowserApi()) {
  browserApi.runtime?.onMessage?.addListener?.(async (message) => {
    if (message.type === "tihc:websearch.extract-serp") {
      return await waitForSerpResults(message.engine);
    }
    if (message.type === "tihc:websearch.extract-page") {
      return {
        excerpt: extractPageExcerpt(document),
      } satisfies WebSearchExtractPageResponse;
    }
    return undefined;
  });
}
