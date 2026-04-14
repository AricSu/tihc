import type { AgentInstance } from "@/lib/chat/agent-types";
import { getConnectorForTemplate } from "./registry";
import type { AgentConnectionTestResult } from "./types";

export async function runAgentConnectionTest(
  agent: AgentInstance,
): Promise<AgentConnectionTestResult> {
  const connector = getConnectorForTemplate(agent.templateId);
  return connector.testConnection(agent);
}
