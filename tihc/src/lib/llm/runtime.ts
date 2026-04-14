import type { AgentInstance, AppRuntimeSettings, GlobalLlmRuntimeConfig } from "@/lib/chat/agent-types";
import { resolveBackendEndpoint, resolveRuntimeBackendBaseUrl } from "@/lib/app/backend-endpoint";
import { getAppSettingsSnapshot } from "@/lib/app/runtime";
import { openAICompatibleConnector } from "@/lib/chat/connectors/http-sse";
import type {
  AgentConnectionTestResult,
  AgentEvent,
  UnifiedChatRequest,
} from "@/lib/chat/connectors/types";

export function hasCompleteGlobalLlmRuntime(llmRuntime: GlobalLlmRuntimeConfig): boolean {
  return Boolean(llmRuntime.providerId.trim() && llmRuntime.model.trim());
}

export function buildGlobalRuntimeAgent(
  settings: AppRuntimeSettings = getAppSettingsSnapshot(),
): AgentInstance {
  const llmRuntime = settings.llmRuntime;
  return {
    id: "global-llm-runtime",
    name: "Global LLM Runtime",
    templateId: "openai-compatible",
    transport: "http",
    endpoint: resolveBackendEndpoint(settings, "/v1/chat/completions") ?? "",
    model: llmRuntime.model.trim(),
    apiKey: settings.googleAuth?.accessToken ?? "",
    headersJson: "{}",
    extraBodyJson: JSON.stringify({
      provider: llmRuntime.providerId.trim(),
    }),
    responseMode: "delta",
    deltaPath: "",
    snapshotPath: "",
    donePath: "",
    doneSentinel: "[DONE]",
  };
}

export async function* streamGlobalRuntime(
  request: UnifiedChatRequest,
  settings: AppRuntimeSettings = getAppSettingsSnapshot(),
): AsyncGenerator<AgentEvent> {
  yield* openAICompatibleConnector.stream(buildGlobalRuntimeAgent(settings), request);
}

export async function testGlobalRuntimeConnection(
  settings: AppRuntimeSettings = getAppSettingsSnapshot(),
): Promise<AgentConnectionTestResult> {
  if (!hasCompleteGlobalLlmRuntime(settings.llmRuntime)) {
    return {
      ok: false,
      message: "Configure the global provider and model first.",
    };
  }
  if (!resolveRuntimeBackendBaseUrl(settings)) {
    return {
      ok: false,
      message: "Configure the runtime backend base URL first.",
    };
  }

  return openAICompatibleConnector.testConnection(buildGlobalRuntimeAgent(settings));
}
