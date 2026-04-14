import { describe, expect, test } from "vitest";

import {
  buildWebSearchPrompt,
  resolveSearchAttemptOrder,
  shouldExtractPageContent,
  shouldSkipWebSearch,
} from "./policy";

describe("websearch policy", () => {
  test("detects explicit skip phrases in Chinese and English", () => {
    expect(shouldSkipWebSearch("请不要搜索，直接离线回答")).toBe(true);
    expect(shouldSkipWebSearch("Explain TiDB internals, offline only.")).toBe(true);
    expect(shouldSkipWebSearch("What is TiDB?")).toBe(false);
  });

  test("returns the fixed fallback chain for each primary engine", () => {
    expect(resolveSearchAttemptOrder("duckduckgo")).toEqual(["duckduckgo", "bing"]);
    expect(resolveSearchAttemptOrder("bing")).toEqual(["bing", "duckduckgo"]);
    expect(resolveSearchAttemptOrder("baidu")).toEqual(["baidu", "bing", "duckduckgo"]);
  });

  test("opens result pages when snippets are sparse or the query signals deep intent", () => {
    expect(
      shouldExtractPageContent("latest TiDB pricing documentation", [
        { snippet: "short" },
      ]),
    ).toBe(true);
    expect(
      shouldExtractPageContent("What is TiDB", [
        { snippet: "one long enough snippet" },
        { snippet: "second long enough snippet" },
        { snippet: "third long enough snippet" },
      ]),
    ).toBe(false);
  });

  test("builds a grounded prompt with citations and the original request", () => {
    const prompt = buildWebSearchPrompt("latest TiDB docs", {
      engine: "duckduckgo",
      query: "latest TiDB docs",
      results: [
        {
          title: "TiDB Docs",
          url: "https://docs.pingcap.com/tidb/stable",
          snippet: "Official TiDB docs.",
          pageExcerpt: "The official docs include deployment guides.",
        },
      ],
      searchedAt: "2026-04-14T12:00:00.000Z",
    });

    expect(prompt).toContain("[Web Search Context]");
    expect(prompt).toContain("[TiDB Docs](https://docs.pingcap.com/tidb/stable)");
    expect(prompt).toContain("User request:\nlatest TiDB docs");
  });
});
