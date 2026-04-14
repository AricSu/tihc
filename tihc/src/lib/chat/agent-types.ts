export type AgentTemplateId = "openai-compatible" | "generic-http-sse" | "generic-websocket";

export type AgentTransport = "http" | "websocket";

export type AgentInstance = {
  id: string;
  name: string;
  templateId: AgentTemplateId;
  transport: AgentTransport;
  endpoint: string;
  model: string;
  apiKey: string;
  headersJson: string;
  extraBodyJson: string;
  responseMode: "delta" | "snapshot";
  deltaPath: string;
  snapshotPath: string;
  donePath: string;
  doneSentinel: string;
};

export type PluginId = "tidb.ai" | "websearch";
export type PluginKind = "agent" | "mcp";
export type PluginCapability = "chat" | "mcp";
export type SearchEngine = "duckduckgo" | "bing" | "baidu";
export type WebSearchMode = "off" | "aggressive";
export type AssistantReplyFontSize = "small" | "default" | "large";

export type GlobalLlmRuntimeConfig = {
  baseUrl: string;
  providerId: string;
  model: string;
};

export type LlmProviderAuthMode = "backend-managed" | "user-api-key" | "codex-oauth";

export type LlmProviderCatalogEntry = {
  id: string;
  label: string;
  authMode: LlmProviderAuthMode;
  configured: boolean;
  defaultModel: string;
  models: Array<{
    id: string;
    label: string;
  }>;
};

export type StoredLlmCredentialStatus = {
  providerId: string;
  hasSecret: boolean;
  updatedAt: string | null;
};

export type TidbAiPluginConfig = {
  baseUrl: string;
};

export type WebSearchPluginConfig = {
  enabled: boolean;
  mode: WebSearchMode;
  primaryEngine: SearchEngine;
};

export type PluginConfigValue = string | boolean;

export type PluginSettingsField = {
  key: string;
  label: string;
  type: "url" | "text" | "select" | "checkbox";
  description?: string;
  placeholder?: string;
  options?: Array<{ label: string; value: string }>;
};

export type PluginManifest = {
  pluginId: PluginId;
  label: string;
  kind: PluginKind;
  capabilities: PluginCapability[];
  settingsFields: PluginSettingsField[];
};

export type PluginMarketplaceStatus = "installed" | "available" | "coming-soon";
export type PluginMarketplaceGroup = "Featured" | "Coding";

export type PluginMarketplaceInclude = {
  name: string;
  type: "App" | "Skill" | "Connector";
  description: string;
  enabled: boolean;
};

export type PluginMarketplaceInfo = {
  label: string;
  value: string;
  href?: string;
};

export type PluginMarketplaceEntry = {
  catalogId: string;
  title: string;
  provider: string;
  builtBy: string;
  group: PluginMarketplaceGroup;
  kind: PluginKind;
  capabilities: PluginCapability[];
  status: PluginMarketplaceStatus;
  installedPluginId: PluginId | null;
  summary: string;
  description: string;
  heroPrompt: string;
  tags: string[];
  highlights: string[];
  includes: PluginMarketplaceInclude[];
  information: PluginMarketplaceInfo[];
};

export type TidbAiInstalledPlugin = {
  pluginId: "tidb.ai";
  label: string;
  kind: "mcp";
  capabilities: ["mcp"];
  config: TidbAiPluginConfig;
};

export type WebSearchInstalledPlugin = {
  pluginId: "websearch";
  label: string;
  kind: "mcp";
  capabilities: ["mcp"];
  config: WebSearchPluginConfig;
};

export type InstalledPlugin = TidbAiInstalledPlugin | WebSearchInstalledPlugin;

export type GoogleAuthState = {
  accessToken: string;
  clientId: string;
  email: string;
  hostedDomain: string;
  expiresAt: string | null;
};

export type AnalyticsConsentState = "unknown" | "granted" | "denied";

export type CloudSyncState = {
  importedClientId: string | null;
  lastHydratedAt: string | null;
  mode: "local" | "cloud";
};

export type CaseActivityState = "ready" | "active" | "resolved";

export type CaseWorkspace = {
  id: string;
  title: string;
  pluginId: PluginId;
  activityState: CaseActivityState;
  resolvedAt: string | null;
  archivedAt: string | null;
  createdAt: string;
  updatedAt: string;
};

export type StoredCaseMessage = {
  role: "operator" | "tihc";
  text: string;
};

export type StoredCaseRecord = CaseWorkspace & {
  summary: string;
  signals: string[];
  messagesPreview: StoredCaseMessage[];
};

export type StoredAppSettingsRecord = {
  activeCaseId: string | null;
  analyticsConsent: AnalyticsConsentState;
  llmRuntime: GlobalLlmRuntimeConfig;
  installedPlugins: InstalledPlugin[];
  updatedAt: string;
};

export type UsagePeriodTotals = {
  requestCount: number;
  inputTokens: number;
  outputTokens: number;
  totalTokens: number;
  cachedInputTokens: number;
  reasoningTokens: number;
  costUsd: number;
};

export type StoredUsageSummaryRecord = {
  windowDays: number;
  current: UsagePeriodTotals;
  previous: UsagePeriodTotals;
};

export type StoredUsageTimeseriesPoint = UsagePeriodTotals & {
  date: string;
};

export type AppRuntimeSettings = {
  activeCaseId: string | null;
  assistantReplyFontSize?: AssistantReplyFontSize;
  analyticsConsent: AnalyticsConsentState;
  cases: CaseWorkspace[];
  cloudSync: CloudSyncState;
  llmRuntime: GlobalLlmRuntimeConfig;
  installedPlugins: InstalledPlugin[];
  googleAuth: GoogleAuthState | null;
};
