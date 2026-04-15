export type ChatMessage = {
  role: string;
  content: string;
};

export type ChatCompletionsRequest = {
  case_id?: string;
  model?: string;
  messages?: ChatMessage[];
  pluginId?: string;
  pluginMode?: string;
  provider?: string;
  stream?: boolean;
};

export type LlmProviderCatalogEntry = {
  id: string;
  label: string;
  authMode: "backend-managed" | "user-api-key" | "codex-oauth";
  configured: boolean;
  defaultModel: string;
  models: Array<{
    id: string;
    label: string;
  }>;
};

export type ChatStreamState = {
  answerText: string;
  emittedText: string;
  streamedText: string[];
};

export type ChatRequestSummary = {
  messageCount: number;
  model: string;
  stream: boolean;
};
