import type { CodexBridge } from "../../lib/codex-bridge";
import type { AppEnv } from "../../shared/support";
import type { LlmProviderCatalogEntry } from "./chat-types";
import { buildHostedProviderCatalog } from "./provider-registry";

export async function buildLlmProviderCatalog(
  env: AppEnv,
  codexBridge: CodexBridge,
  fetchImpl: typeof fetch,
): Promise<LlmProviderCatalogEntry[]> {
  const providers: LlmProviderCatalogEntry[] = await buildHostedProviderCatalog(env, fetchImpl);

  try {
    const codexModels = await codexBridge.listModels();
    if (codexModels.length > 0) {
      providers.push({
        id: "codex",
        label: "Codex",
        authMode: "codex-oauth",
        configured: true,
        defaultModel: codexModels[0]!.id,
        models: codexModels,
      });
    }
  } catch {
    // Ignore local bridge discovery failures so the API can still serve hosted providers.
  }

  return providers;
}
