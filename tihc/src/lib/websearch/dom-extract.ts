import type { SearchEngine } from "@/lib/chat/agent-types";
import type { WebSearchResult } from "./types";

function normalizeWhitespace(text: string): string {
  return text.replace(/\s+/g, " ").trim();
}

function extractText(element: Element | null): string {
  return normalizeWhitespace(element?.textContent ?? "");
}

function extractHref(element: Element | null): string {
  const href = element?.getAttribute("href") ?? "";
  return normalizeWhitespace(href);
}

function clipText(text: string, limit: number): string {
  if (text.length <= limit) return text;
  return `${text.slice(0, limit - 3).trimEnd()}...`;
}

function pruneContainer(root: Element): Element {
  const clone = root.cloneNode(true) as Element;
  clone
    .querySelectorAll("script, style, noscript, svg, nav, footer, header, form, aside")
    .forEach((node) => node.remove());
  return clone;
}

export function detectBaiduVerificationPage(document: Document): boolean {
  const title = normalizeWhitespace(document.title);
  const bodyText = normalizeWhitespace(document.body?.textContent ?? "");
  return (
    title.includes("百度安全验证") ||
    bodyText.includes("百度安全验证") ||
    bodyText.includes("网络不给力，请稍后重试")
  );
}

export function extractSerpResults(document: Document, engine: SearchEngine): WebSearchResult[] {
  if (engine === "duckduckgo") {
    return [...document.querySelectorAll(".result")]
      .map((node) => {
        const anchor = node.querySelector(".result__a");
        return {
          title: extractText(anchor),
          url: extractHref(anchor),
          snippet: extractText(node.querySelector(".result__snippet")),
        };
      })
      .filter((result) => result.title && result.url)
      .slice(0, 5);
  }

  if (engine === "bing") {
    return [...document.querySelectorAll("li.b_algo")]
      .map((node) => {
        const anchor = node.querySelector("h2 a");
        return {
          title: extractText(anchor),
          url: extractHref(anchor),
          snippet: extractText(node.querySelector(".b_caption p")),
        };
      })
      .filter((result) => result.title && result.url)
      .slice(0, 5);
  }

  return [...document.querySelectorAll("#content_left .result, #content_left .c-container")]
    .map((node) => {
      const anchor = node.querySelector("h3 a");
      return {
        title: extractText(anchor),
        url: extractHref(anchor),
        snippet: extractText(
          node.querySelector(".c-abstract, .content-right_8Zs40, .c-span-last p, .c-color-text"),
        ),
      };
    })
    .filter((result) => result.title && result.url)
    .slice(0, 5);
}

export function extractPageExcerpt(document: Document): string {
  const root =
    document.querySelector("article") ??
    document.querySelector("main") ??
    document.querySelector("[role='main']") ??
    document.body;

  if (!root) return "";

  const text = normalizeWhitespace(pruneContainer(root).textContent ?? "");
  return clipText(text, 600);
}
