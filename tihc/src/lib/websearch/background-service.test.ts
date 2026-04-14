import { beforeEach, describe, expect, test, vi } from "vitest";

import { createWebSearchService } from "./background-service";

describe("websearch background service", () => {
  const createWorkerTab = vi.fn();
  const navigateWorkerTab = vi.fn();
  const activateWorkerTab = vi.fn();
  const extractSerp = vi.fn();
  const extractPage = vi.fn();
  const sleep = vi.fn(async () => {});

  beforeEach(() => {
    vi.clearAllMocks();
    createWorkerTab.mockResolvedValue(41);
    navigateWorkerTab.mockResolvedValue(undefined);
    activateWorkerTab.mockResolvedValue(undefined);
    extractPage.mockResolvedValue({ excerpt: "Expanded page content." });
  });

  test("reuses the fixed worker tab across multiple searches", async () => {
    extractSerp.mockResolvedValue({
      results: [{ title: "TiDB Docs", url: "https://docs.pingcap.com", snippet: "Docs." }],
    });

    const service = createWebSearchService({
      activateWorkerTab,
      createWorkerTab,
      extractPage,
      extractSerp,
      navigateWorkerTab,
      now: () => "2026-04-14T12:00:00.000Z",
      sleep,
    });

    await service.run({ primaryEngine: "duckduckgo", query: "TiDB docs" }, () => {});
    await service.run({ primaryEngine: "duckduckgo", query: "TiDB pricing" }, () => {});

    expect(createWorkerTab).toHaveBeenCalledTimes(1);
    expect(navigateWorkerTab.mock.calls.filter(([tabId]) => tabId === 41).length).toBeGreaterThanOrEqual(2);
  });

  test("falls back through the fixed engine chain when the primary engine yields no results", async () => {
    extractSerp
      .mockResolvedValueOnce({ results: [] })
      .mockResolvedValueOnce({
        results: [{ title: "Bing TiDB", url: "https://bing.example/tidb", snippet: "Bing result." }],
      });

    const statuses: string[] = [];
    const service = createWebSearchService({
      activateWorkerTab,
      createWorkerTab,
      extractPage,
      extractSerp,
      navigateWorkerTab,
      now: () => "2026-04-14T12:00:00.000Z",
      sleep,
    });

    const bundle = await service.run(
      { primaryEngine: "baidu", query: "TiDB latest docs" },
      (status) => statuses.push(status),
    );

    expect(bundle.engine).toBe("bing");
    expect(statuses).toContain("Searching web with baidu");
    expect(statuses).toContain("Falling back to bing");
  });

  test("activates the worker tab and retries when Baidu requires manual verification", async () => {
    extractSerp
      .mockResolvedValueOnce({ results: [], verificationRequired: true })
      .mockResolvedValueOnce({
        results: [{ title: "PingCAP", url: "https://www.pingcap.com", snippet: "PingCAP official site." }],
      });

    const statuses: string[] = [];
    const service = createWebSearchService({
      activateWorkerTab,
      createWorkerTab,
      extractPage,
      extractSerp,
      navigateWorkerTab,
      now: () => "2026-04-14T12:00:00.000Z",
      sleep,
    });

    const bundle = await service.run(
      { primaryEngine: "baidu", query: "TiDB 官网" },
      (status) => statuses.push(status),
    );

    expect(activateWorkerTab).toHaveBeenCalledTimes(1);
    expect(sleep).toHaveBeenCalled();
    expect(statuses).toContain("Waiting for Baidu verification");
    expect(bundle.results[0]?.title).toBe("PingCAP");
  });
});
