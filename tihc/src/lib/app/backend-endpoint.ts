import type { AppRuntimeSettings } from "@/lib/chat/agent-types";

export function resolveRuntimeBackendBaseUrl(settings: AppRuntimeSettings): string {
  return settings.llmRuntime?.baseUrl?.trim() || resolveTidbAiBaseUrl(settings);
}

export function resolveTidbAiBaseUrl(settings: AppRuntimeSettings): string {
  return (
    settings.installedPlugins.find((plugin) => plugin.pluginId === "tidb.ai")?.config.baseUrl?.trim() ??
    ""
  );
}

export function resolveBackendEndpoint(
  settings: AppRuntimeSettings,
  path: string,
): string | null {
  const baseUrl = resolveRuntimeBackendBaseUrl(settings);
  if (!baseUrl) return null;

  try {
    return new URL(path, baseUrl).toString();
  } catch {
    return null;
  }
}
