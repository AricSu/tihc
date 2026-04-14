import { describe, expect, test } from "vitest";

import {
  detectBaiduVerificationPage,
  extractPageExcerpt,
  extractSerpResults,
} from "./dom-extract";

function parse(html: string) {
  return new DOMParser().parseFromString(html, "text/html");
}

describe("websearch DOM extraction", () => {
  test("extracts normalized DuckDuckGo SERP results", () => {
    const document = parse(`
      <div class="results">
        <div class="result">
          <a class="result__a" href="https://docs.pingcap.com/tidb/stable">TiDB Docs</a>
          <a class="result__snippet">Official TiDB documentation.</a>
        </div>
      </div>
    `);

    expect(extractSerpResults(document, "duckduckgo")).toEqual([
      {
        title: "TiDB Docs",
        url: "https://docs.pingcap.com/tidb/stable",
        snippet: "Official TiDB documentation.",
      },
    ]);
  });

  test("extracts normalized Bing SERP results", () => {
    const document = parse(`
      <ol id="b_results">
        <li class="b_algo">
          <h2><a href="https://www.pingcap.com">PingCAP</a></h2>
          <div class="b_caption"><p>Company page.</p></div>
        </li>
      </ol>
    `);

    expect(extractSerpResults(document, "bing")).toEqual([
      {
        title: "PingCAP",
        url: "https://www.pingcap.com",
        snippet: "Company page.",
      },
    ]);
  });

  test("extracts normalized Baidu SERP results", () => {
    const document = parse(`
      <div id="content_left">
        <div class="result c-container">
          <h3><a href="https://www.pingcap.com">PingCAP 官网</a></h3>
          <div class="c-abstract">PingCAP 与 TiDB 介绍。</div>
        </div>
      </div>
    `);

    expect(extractSerpResults(document, "baidu")).toEqual([
      {
        title: "PingCAP 官网",
        url: "https://www.pingcap.com",
        snippet: "PingCAP 与 TiDB 介绍。",
      },
    ]);
  });

  test("detects the Baidu security verification page", () => {
    const document = parse(`
      <html>
        <head><title>百度安全验证</title></head>
        <body><div>网络不给力，请稍后重试</div></body>
      </html>
    `);

    expect(detectBaiduVerificationPage(document)).toBe(true);
  });

  test("extracts page excerpt from article content before falling back to body text", () => {
    const document = parse(`
      <body>
        <main>
          <article>
            <h1>TiDB deployment</h1>
            <p>TiDB supports Kubernetes deployments.</p>
            <p>Use TiUP for local clusters.</p>
          </article>
        </main>
        <footer>ignored footer</footer>
      </body>
    `);

    expect(extractPageExcerpt(document)).toContain("TiDB supports Kubernetes deployments.");
    expect(extractPageExcerpt(document)).not.toContain("ignored footer");
  });
});
