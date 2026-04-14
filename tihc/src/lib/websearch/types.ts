import type { SearchEngine } from "@/lib/chat/agent-types";

export type WebSearchResult = {
  title: string;
  url: string;
  snippet: string;
  pageExcerpt?: string;
};

export type WebSearchBundle = {
  query: string;
  engine: SearchEngine;
  searchedAt: string;
  results: WebSearchResult[];
  verificationRequired?: boolean;
};

export type WebSearchRunRequest = {
  type: "tihc:websearch.run";
  query: string;
  primaryEngine: SearchEngine;
};

export type WebSearchPortEvent =
  | { type: "status"; text: string }
  | { type: "result"; bundle: WebSearchBundle }
  | { type: "error"; message: string };

export type WebSearchExtractSerpMessage = {
  type: "tihc:websearch.extract-serp";
  engine: SearchEngine;
};

export type WebSearchExtractPageMessage = {
  type: "tihc:websearch.extract-page";
};

export type WebSearchExtractSerpResponse = {
  results: WebSearchResult[];
  verificationRequired?: boolean;
};

export type WebSearchExtractPageResponse = {
  excerpt: string;
};
