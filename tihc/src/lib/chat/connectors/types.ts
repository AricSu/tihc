import type { AgentInstance, AgentTemplateId } from "@/lib/chat/agent-types";

export type ChatMessage = {
  role: "user" | "assistant";
  content: string;
};

export type UnifiedChatRequest = {
  messages: ChatMessage[];
  abortSignal: AbortSignal;
};

export type AgentEvent =
  | { type: "status"; text: string }
  | { type: "text-delta"; text: string }
  | { type: "replace-text"; text: string }
  | { type: "error"; message: string }
  | { type: "done" };

export type AgentConnectionTestResult = {
  ok: boolean;
  message: string;
};

export type AgentConnector = {
  id: AgentTemplateId;
  stream(
    agent: AgentInstance,
    request: UnifiedChatRequest,
  ): AsyncGenerator<AgentEvent>;
  testConnection(agent: AgentInstance): Promise<AgentConnectionTestResult>;
};
