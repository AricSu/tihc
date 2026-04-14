import type { AgentInstance, AgentTemplateId, AgentTransport } from "./agent-types";

export type AgentTemplateDefinition = {
  id: AgentTemplateId;
  label: string;
  description: string;
  transport: AgentTransport;
  defaults: Omit<AgentInstance, "id" | "name">;
};

export const AGENT_TEMPLATES: AgentTemplateDefinition[] = [
  {
    id: "openai-compatible",
    label: "OpenAI-Compatible",
    description: "POST chat messages to a compatible chat completions endpoint.",
    transport: "http",
    defaults: {
      templateId: "openai-compatible",
      transport: "http",
      endpoint: "",
      model: "",
      apiKey: "",
      headersJson: "{}",
      extraBodyJson: "{}",
      responseMode: "delta",
      deltaPath: "choices.0.delta.content",
      snapshotPath: "choices.0.message.content",
      donePath: "",
      doneSentinel: "[DONE]",
    },
  },
  {
    id: "generic-http-sse",
    label: "Generic HTTP SSE",
    description: "POST JSON and parse text or JSON data frames from an SSE stream.",
    transport: "http",
    defaults: {
      templateId: "generic-http-sse",
      transport: "http",
      endpoint: "",
      model: "",
      apiKey: "",
      headersJson: "{}",
      extraBodyJson: "{}",
      responseMode: "delta",
      deltaPath: "delta",
      snapshotPath: "text",
      donePath: "done",
      doneSentinel: "[DONE]",
    },
  },
  {
    id: "generic-websocket",
    label: "Generic WebSocket",
    description: "Send one JSON request and parse text deltas or snapshots from messages.",
    transport: "websocket",
    defaults: {
      templateId: "generic-websocket",
      transport: "websocket",
      endpoint: "",
      model: "",
      apiKey: "",
      headersJson: "{}",
      extraBodyJson: "{}",
      responseMode: "delta",
      deltaPath: "delta",
      snapshotPath: "text",
      donePath: "done",
      doneSentinel: "[DONE]",
    },
  },
];

export function getAgentTemplate(templateId: AgentTemplateId): AgentTemplateDefinition {
  return (
    AGENT_TEMPLATES.find((template) => template.id === templateId) ?? AGENT_TEMPLATES[0]!
  );
}
