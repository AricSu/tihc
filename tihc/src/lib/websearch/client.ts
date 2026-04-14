import type { SearchEngine } from "@/lib/chat/agent-types";
import type {
  WebSearchBundle,
  WebSearchPortEvent,
  WebSearchRunRequest,
} from "./types";

type RuntimePort = {
  disconnect: () => void;
  onDisconnect?: {
    addListener?: (listener: () => void) => void;
    removeListener?: (listener: () => void) => void;
  };
  onMessage?: {
    addListener?: (listener: (message: WebSearchPortEvent) => void) => void;
    removeListener?: (listener: (message: WebSearchPortEvent) => void) => void;
  };
  postMessage: (message: WebSearchRunRequest) => void;
};

function resolveRuntimeApi() {
  return (
    (globalThis as typeof globalThis & {
      browser?: {
        runtime?: {
          connect?: (options?: { name?: string }) => RuntimePort;
        };
      };
      chrome?: {
        runtime?: {
          connect?: (options?: { name?: string }) => RuntimePort;
        };
      };
    }).browser?.runtime ??
    (globalThis as typeof globalThis & {
      chrome?: {
        runtime?: {
          connect?: (options?: { name?: string }) => RuntimePort;
        };
      };
    }).chrome?.runtime
  );
}

export async function runWebSearch({
  abortSignal,
  onStatus,
  primaryEngine,
  query,
}: {
  abortSignal?: AbortSignal;
  onStatus?: (status: string) => void;
  primaryEngine: SearchEngine;
  query: string;
}): Promise<WebSearchBundle | null> {
  const runtime = resolveRuntimeApi();
  if (!runtime?.connect) {
    return null;
  }

  const port = runtime.connect({ name: "tihc:websearch.run" });

  return await new Promise<WebSearchBundle | null>((resolve, reject) => {
    let settled = false;

    const cleanup = () => {
      port.onMessage?.removeListener?.(handleMessage);
      port.onDisconnect?.removeListener?.(handleDisconnect);
      abortSignal?.removeEventListener("abort", handleAbort);
      port.disconnect();
    };

    const settle = (callback: () => void) => {
      if (settled) return;
      settled = true;
      cleanup();
      callback();
    };

    const handleAbort = () => {
      settle(() => reject(new DOMException("Websearch aborted.", "AbortError")));
    };

    const handleDisconnect = () => {
      settle(() => resolve(null));
    };

    const handleMessage = (message: WebSearchPortEvent) => {
      if (message.type === "status") {
        onStatus?.(message.text);
        return;
      }
      if (message.type === "result") {
        settle(() => resolve(message.bundle));
        return;
      }
      settle(() => reject(new Error(message.message)));
    };

    port.onMessage?.addListener?.(handleMessage);
    port.onDisconnect?.addListener?.(handleDisconnect);
    abortSignal?.addEventListener("abort", handleAbort, { once: true });
    port.postMessage({
      primaryEngine,
      query,
      type: "tihc:websearch.run",
    });
  });
}
