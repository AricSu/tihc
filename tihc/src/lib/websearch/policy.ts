import type {
  SearchEngine,
  WebSearchPluginConfig,
  WebSearchMode,
} from "@/lib/chat/agent-types";
import type { ChatMessage } from "@/lib/chat/connectors/types";
import type { WebSearchBundle, WebSearchResult } from "./types";

const EXPLICIT_SKIP_PATTERNS = [
  /不要搜索/i,
  /不用搜索/i,
  /别联网/i,
  /离线回答/i,
  /\bdo not search\b/i,
  /\bdon't search\b/i,
  /\bwithout web search\b/i,
  /\boffline only\b/i,
] as const;

const DEEP_INTENT_PATTERNS = [
  /官网/i,
  /文档/i,
  /最新/i,
  /价格/i,
  /对比/i,
  /\bhow to\b/i,
  /\bdocumentation\b/i,
  /\bpricing\b/i,
  /\bcompare\b/i,
] as const;

const SEARCH_ATTEMPT_ORDER: Record<SearchEngine, SearchEngine[]> = {
  baidu: ["baidu", "bing", "duckduckgo"],
  bing: ["bing", "duckduckgo"],
  duckduckgo: ["duckduckgo", "bing"],
};

function normalizeWhitespace(text: string): string {
  return text.replace(/\s+/g, " ").trim();
}

export function shouldSkipWebSearch(text: string): boolean {
  return EXPLICIT_SKIP_PATTERNS.some((pattern) => pattern.test(text));
}

export function shouldRunWebSearch(config: WebSearchPluginConfig, text: string): boolean {
  return (
    config.enabled &&
    config.mode !== ("off" satisfies WebSearchMode) &&
    !shouldSkipWebSearch(text)
  );
}

export function resolveSearchAttemptOrder(primary: SearchEngine): SearchEngine[] {
  return [...SEARCH_ATTEMPT_ORDER[primary]];
}

export function shouldExtractPageContent(
  query: string,
  results: Array<Pick<WebSearchResult, "snippet">>,
): boolean {
  if (DEEP_INTENT_PATTERNS.some((pattern) => pattern.test(query))) {
    return true;
  }

  const nonEmptySnippets = results
    .map((result) => normalizeWhitespace(result.snippet))
    .filter(Boolean);

  if (nonEmptySnippets.length < 3) {
    return true;
  }

  const totalSnippetLength = nonEmptySnippets.reduce((sum, snippet) => sum + snippet.length, 0);
  return totalSnippetLength < 60;
}

export function buildWebSearchPrompt(userRequest: string, bundle: WebSearchBundle): string {
  const sources = bundle.results.map((result, index) => {
    const lines = [
      `${index + 1}. [${result.title}](${result.url})`,
      `   Snippet: ${result.snippet || "No snippet available."}`,
    ];
    if (result.pageExcerpt) {
      lines.push(`   Page Excerpt: ${result.pageExcerpt}`);
    }
    return lines.join("\n");
  });

  return [
    "[Web Search Context]",
    `Query: ${bundle.query}`,
    `Engine: ${bundle.engine}`,
    `Searched At: ${bundle.searchedAt}`,
    "Sources:",
    ...sources,
    "Instruction:",
    "- Use this search context for freshness and factual grounding.",
    "- Cite sources with markdown links.",
    "- If search results are insufficient or conflicting, say so explicitly.",
    "[/Web Search Context]",
    "",
    "User request:",
    userRequest,
  ]
    .join("\n")
    .trim();
}

export function injectWebSearchContext(
  messages: ChatMessage[],
  bundle: WebSearchBundle,
): ChatMessage[] {
  const lastUserIndex = [...messages]
    .map((message, index) => ({ index, message }))
    .reverse()
    .find((entry) => entry.message.role === "user")?.index;

  if (lastUserIndex === undefined) return messages;

  return messages.map((message, index) =>
    index === lastUserIndex
      ? {
          ...message,
          content: buildWebSearchPrompt(message.content, bundle),
        }
      : message,
  );
}
