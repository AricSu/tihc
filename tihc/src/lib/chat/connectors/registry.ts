import type { AgentTemplateId } from "@/lib/chat/agent-types";
import { genericHttpSseConnector, openAICompatibleConnector } from "./http-sse";
import { genericWebSocketConnector } from "./generic-websocket";
import type { AgentConnector } from "./types";

const CONNECTORS: Record<AgentTemplateId, AgentConnector> = {
  "generic-http-sse": genericHttpSseConnector,
  "generic-websocket": genericWebSocketConnector,
  "openai-compatible": openAICompatibleConnector,
};

export function getConnectorForTemplate(templateId: AgentTemplateId): AgentConnector {
  return CONNECTORS[templateId];
}
