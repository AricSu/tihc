import type {
  AssistantReplyFontSize,
  AnalyticsConsentState,
  AppRuntimeSettings,
  CaseActivityState,
  CaseWorkspace,
  CloudSyncState,
  GlobalLlmRuntimeConfig,
  GoogleAuthState,
  InstalledPlugin,
  PluginId,
  SearchEngine,
  StoredAppSettingsRecord,
  StoredCaseRecord,
  TidbAiPluginConfig,
  WebSearchMode,
  WebSearchPluginConfig,
} from "@/lib/chat/agent-types";

export const TIDB_PLUGIN_ID = "tidb.ai" satisfies PluginId;
export const WEBSEARCH_PLUGIN_ID = "websearch" satisfies PluginId;
export const DEFAULT_ASSISTANT_REPLY_FONT_SIZE = "default" satisfies AssistantReplyFontSize;
export const UNTITLED_CASE_TITLES = new Set(["", "new case", "untitled case", "untitled"]);
export const LEGACY_STORAGE_KEYS = [
  "tihc_app_settings_v1",
  "tihc_local_mode_state_v1",
  "tihc_app_client_id_v1",
  "tihc_telemetry_client_id_v1",
  "tihc_telemetry_session_id_v1",
  "tihc_telemetry_session_seen_at_v1",
] as const;
export const LEGACY_STORAGE_PREFIXES = ["tihc_case_history_v3:", "tihc_thread_history_v2:"] as const;

export type RuntimeState = AppRuntimeSettings;

const EMPTY_LLM_RUNTIME: GlobalLlmRuntimeConfig = {
  baseUrl: resolveDefaultBackendBaseUrl(),
  providerId: "",
  model: "",
};

function resolveDefaultBackendBaseUrl(): string {
  const env = import.meta.env as Record<string, string | undefined>;
  return env.VITE_BACKEND_BASE_URL?.trim() || "";
}

function createId(): string {
  if (typeof globalThis.crypto?.randomUUID === "function") {
    return globalThis.crypto.randomUUID();
  }
  return `case-${Date.now()}-${Math.random().toString(16).slice(2, 10)}`;
}

function asString(value: unknown): string | null {
  return typeof value === "string" ? value : null;
}

function asRecord(value: unknown): Record<string, unknown> | null {
  if (!value || typeof value !== "object" || Array.isArray(value)) return null;
  return value as Record<string, unknown>;
}

function cloneGoogleAuth(googleAuth: GoogleAuthState | null): GoogleAuthState | null {
  if (!googleAuth) return null;
  return { ...googleAuth };
}

function cloneCloudSync(cloudSync: CloudSyncState): CloudSyncState {
  return { ...cloudSync };
}

function cloneLlmRuntime(llmRuntime: GlobalLlmRuntimeConfig): GlobalLlmRuntimeConfig {
  return { ...llmRuntime };
}

export function normalizeAssistantReplyFontSize(value: unknown): AssistantReplyFontSize {
  if (value === "small" || value === "large") {
    return value;
  }
  return DEFAULT_ASSISTANT_REPLY_FONT_SIZE;
}

function clonePlugin(plugin: InstalledPlugin): InstalledPlugin {
  if (plugin.pluginId === WEBSEARCH_PLUGIN_ID) {
    return {
      ...plugin,
      capabilities: ["mcp"],
      config: { ...plugin.config },
    };
  }

  return {
    ...plugin,
    capabilities: ["mcp"],
    config: { ...plugin.config },
  };
}

export function cloneCaseWorkspace(caseWorkspace: CaseWorkspace): CaseWorkspace {
  return { ...caseWorkspace };
}

export function cloneRuntimeState(settings: RuntimeState): RuntimeState {
  return {
    activeCaseId: settings.activeCaseId,
    assistantReplyFontSize: normalizeAssistantReplyFontSize(settings.assistantReplyFontSize),
    analyticsConsent: settings.analyticsConsent,
    cases: settings.cases.map(cloneCaseWorkspace),
    cloudSync: cloneCloudSync(settings.cloudSync),
    llmRuntime: cloneLlmRuntime(settings.llmRuntime),
    installedPlugins: settings.installedPlugins.map(clonePlugin),
    googleAuth: cloneGoogleAuth(settings.googleAuth),
  };
}

export function clonePublicSettings(settings: RuntimeState): AppRuntimeSettings {
  return cloneRuntimeState(settings);
}

function resolveWebSearchEnabled(config: Partial<WebSearchPluginConfig> = {}): boolean {
  return config.enabled ?? true;
}

function resolveWebSearchMode(config: Partial<WebSearchPluginConfig> = {}): WebSearchMode {
  return config.mode === "off" ? "off" : "aggressive";
}

function resolveWebSearchPrimaryEngine(
  config: Partial<WebSearchPluginConfig> = {},
): SearchEngine {
  if (config.primaryEngine === "baidu" || config.primaryEngine === "bing") {
    return config.primaryEngine;
  }
  return "duckduckgo";
}

export function normalizeGlobalLlmRuntime(
  value: Partial<GlobalLlmRuntimeConfig> | null | undefined,
): GlobalLlmRuntimeConfig {
  return {
    baseUrl: value?.baseUrl?.trim() || "",
    providerId: value?.providerId?.trim() || "",
    model: value?.model?.trim() || "",
  };
}

export function buildTidbAiInstalledPlugin(
  config: Partial<TidbAiPluginConfig> = {},
): InstalledPlugin {
  return {
    pluginId: TIDB_PLUGIN_ID,
    label: "tidb.ai",
    kind: "mcp",
    capabilities: ["mcp"],
    config: {
      baseUrl: config.baseUrl?.trim() || resolveDefaultBackendBaseUrl(),
    },
  };
}

export function buildWebSearchInstalledPlugin(
  config: Partial<WebSearchPluginConfig> = {},
): InstalledPlugin {
  return {
    pluginId: WEBSEARCH_PLUGIN_ID,
    label: "Web Search",
    kind: "mcp",
    capabilities: ["mcp"],
    config: {
      enabled: resolveWebSearchEnabled(config),
      mode: resolveWebSearchMode(config),
      primaryEngine: resolveWebSearchPrimaryEngine(config),
    },
  };
}

function normalizeActivityState(value: unknown): CaseActivityState {
  if (value === "active" || value === "resolved") return value;
  return "ready";
}

export function buildCaseWorkspace(overrides: Partial<CaseWorkspace> = {}): CaseWorkspace {
  const now = new Date().toISOString();
  return {
    id: overrides.id?.trim() || createId(),
    title: overrides.title?.trim() || "New Case",
    pluginId: TIDB_PLUGIN_ID,
    activityState: normalizeActivityState(overrides.activityState),
    resolvedAt: overrides.resolvedAt ?? null,
    archivedAt: overrides.archivedAt ?? null,
    createdAt: overrides.createdAt ?? now,
    updatedAt: overrides.updatedAt ?? now,
  };
}

export function deriveThreadTitleFromPrompt(prompt: string): string {
  const normalized = prompt.replace(/\s+/g, " ").trim();
  if (!normalized) return "New Case";
  return normalized.length > 72 ? `${normalized.slice(0, 69).trimEnd()}...` : normalized;
}

function compareByUpdatedAtDesc(a: CaseWorkspace, b: CaseWorkspace): number {
  return Date.parse(b.updatedAt) - Date.parse(a.updatedAt);
}

export function selectFallbackCaseId(
  cases: CaseWorkspace[],
  preferredId?: string | null,
): string | null {
  const visibleCases = cases.filter((item) => item.archivedAt === null).sort(compareByUpdatedAtDesc);
  if (!visibleCases.length) return null;
  if (preferredId && visibleCases.some((item) => item.id === preferredId)) {
    return preferredId;
  }
  return visibleCases[0]?.id ?? null;
}

function normalizeInstalledPlugins(partial: Partial<AppRuntimeSettings>): InstalledPlugin[] {
  if (!Array.isArray(partial.installedPlugins)) {
    return [buildTidbAiInstalledPlugin()];
  }

  const installedPlugins = partial.installedPlugins
    .map((item) => asRecord(item))
    .filter((item): item is Record<string, unknown> => !!item)
    .map((item) => {
      const pluginId = asString(item.pluginId);
      const config = asRecord(item.config);
      if (pluginId === WEBSEARCH_PLUGIN_ID) {
        return buildWebSearchInstalledPlugin(config as Partial<WebSearchPluginConfig> | undefined);
      }
      if (pluginId === TIDB_PLUGIN_ID) {
        return buildTidbAiInstalledPlugin(config as Partial<TidbAiPluginConfig> | undefined);
      }
      return null;
    })
    .filter((item): item is InstalledPlugin => !!item);

  const tidbPlugin =
    installedPlugins.find((item) => item.pluginId === TIDB_PLUGIN_ID) ??
    buildTidbAiInstalledPlugin();
  const webSearchPlugin = installedPlugins.find((item) => item.pluginId === WEBSEARCH_PLUGIN_ID);

  return webSearchPlugin ? [tidbPlugin, webSearchPlugin] : [tidbPlugin];
}

function normalizeLlmRuntime(partial: Partial<AppRuntimeSettings>): GlobalLlmRuntimeConfig {
  const record = asRecord(partial.llmRuntime);
  return normalizeGlobalLlmRuntime(record as Partial<GlobalLlmRuntimeConfig> | null | undefined);
}

function normalizeCases(partial: Partial<AppRuntimeSettings>): CaseWorkspace[] {
  if (!Array.isArray(partial.cases)) return [];

  return partial.cases
    .map((item) => asRecord(item))
    .filter((item): item is Record<string, unknown> => !!item)
    .map((item) =>
      buildCaseWorkspace({
        id: asString(item.id) ?? undefined,
        title: asString(item.title) ?? undefined,
        pluginId: TIDB_PLUGIN_ID,
        activityState: normalizeActivityState(item.activityState),
        resolvedAt: asString(item.resolvedAt),
        archivedAt: asString(item.archivedAt),
        createdAt: asString(item.createdAt) ?? undefined,
        updatedAt: asString(item.updatedAt) ?? undefined,
      }),
    );
}

function normalizeGoogleAuth(partial: Partial<AppRuntimeSettings>): GoogleAuthState | null {
  const explicitGoogleAuth = asRecord(partial.googleAuth);
  if (explicitGoogleAuth) {
    const accessToken = asString(explicitGoogleAuth.accessToken) ?? "";
    if (!accessToken) return null;
    return {
      accessToken,
      clientId: asString(explicitGoogleAuth.clientId) ?? "",
      email: asString(explicitGoogleAuth.email) ?? "",
      hostedDomain: asString(explicitGoogleAuth.hostedDomain) ?? "",
      expiresAt: asString(explicitGoogleAuth.expiresAt),
    };
  }
  return null;
}

export function normalizeAnalyticsConsent(value: unknown): AnalyticsConsentState {
  if (value === "granted" || value === "denied") return value;
  return "unknown";
}

function normalizeCloudSync(partial: Partial<AppRuntimeSettings>): CloudSyncState {
  const cloudSync = asRecord(partial.cloudSync);
  return {
    importedClientId: asString(cloudSync?.importedClientId),
    lastHydratedAt: asString(cloudSync?.lastHydratedAt),
    mode: cloudSync?.mode === "cloud" ? "cloud" : "local",
  };
}

export function toCaseWorkspace(storedCase: StoredCaseRecord): CaseWorkspace {
  return buildCaseWorkspace({
    id: storedCase.id,
    title: storedCase.title,
    pluginId: storedCase.pluginId,
    activityState: storedCase.activityState,
    resolvedAt: storedCase.resolvedAt,
    archivedAt: storedCase.archivedAt,
    createdAt: storedCase.createdAt,
    updatedAt: storedCase.updatedAt,
  });
}

export function normalizeNewSettings(partial: Partial<AppRuntimeSettings>): RuntimeState {
  const installedPlugins = normalizeInstalledPlugins(partial);
  const cases = normalizeCases(partial);
  const activeCaseCandidate = asString(partial.activeCaseId);
  const activeCaseId = selectFallbackCaseId(cases, activeCaseCandidate);

  return {
    activeCaseId,
    assistantReplyFontSize: normalizeAssistantReplyFontSize(partial.assistantReplyFontSize),
    analyticsConsent: normalizeAnalyticsConsent(partial.analyticsConsent),
    cases,
    cloudSync: normalizeCloudSync(partial),
    llmRuntime: normalizeLlmRuntime(partial),
    installedPlugins,
    googleAuth: normalizeGoogleAuth(partial),
  };
}

export function createInitialRuntimeState(): RuntimeState {
  return {
    activeCaseId: null,
    assistantReplyFontSize: DEFAULT_ASSISTANT_REPLY_FONT_SIZE,
    analyticsConsent: "unknown",
    cases: [],
    cloudSync: {
      importedClientId: null,
      lastHydratedAt: null,
      mode: "local",
    },
    llmRuntime: { ...EMPTY_LLM_RUNTIME },
    installedPlugins: [buildTidbAiInstalledPlugin()],
    googleAuth: null,
  };
}

export function parseLegacySettingsPayload(raw: string | null): Partial<AppRuntimeSettings> | null {
  if (!raw?.trim()) return null;
  try {
    const parsed = JSON.parse(raw) as unknown;
    if (!parsed || typeof parsed !== "object" || Array.isArray(parsed)) return null;
    return parsed as Partial<AppRuntimeSettings>;
  } catch {
    return null;
  }
}
