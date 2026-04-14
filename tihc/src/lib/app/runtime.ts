import type {
  AssistantReplyFontSize,
  AnalyticsConsentState,
  AppRuntimeSettings,
  GlobalLlmRuntimeConfig,
  GoogleAuthState,
  PluginId,
  StoredAppSettingsRecord,
  TidbAiPluginConfig,
  WebSearchPluginConfig,
} from "@/lib/chat/agent-types";
import { getAppClientId } from "@/lib/app/client-id";
import { getStoredAppSettings, saveStoredAppSettings } from "@/lib/app/cloud-settings";
import {
  createStoredCase,
  deleteStoredCase,
  importStoredCases,
  isCloudSyncEnabled,
  listStoredCases,
  updateStoredCase,
} from "@/lib/app/cloud-cases";
import {
  clearLocalCaseHistory,
  createCaseHistoryAdapter,
  hasStoredCaseHistory,
} from "@/lib/app/thread-history";
import {
  readLocalRuntimeState,
  writeLocalDisplaySettings,
  writeLocalRuntimeState,
} from "@/lib/app/local-browser-persistence";
import { isGoogleOAuthConfigured, refreshGoogleAuthSession } from "@/lib/auth/google-oauth";
import { createRuntimeCaseOps } from "@/lib/app/runtime-case-ops";
import { createRuntimeCloudSync } from "@/lib/app/runtime-cloud-sync";
import {
  buildTidbAiInstalledPlugin,
  cloneRuntimeState,
  createInitialRuntimeState,
  normalizeAnalyticsConsent,
  normalizeAssistantReplyFontSize,
  normalizeGlobalLlmRuntime,
  normalizeNewSettings,
  type RuntimeState,
} from "@/lib/app/runtime-state";
import { createRuntimeStore } from "@/lib/app/runtime-store";

const store = createRuntimeStore(createInitialRuntimeState());
const settings = store.state;
let authBootstrapPromise: Promise<GoogleAuthState | null> | null = null;
let autoAuthDisabled = false;

function buildLocalRuntimeSettingsInput(): Partial<AppRuntimeSettings> {
  const nextState = cloneRuntimeState(settings);
  return {
    activeCaseId: nextState.activeCaseId,
    analyticsConsent: nextState.analyticsConsent,
    cases: nextState.cases,
    cloudSync: {
      importedClientId: null,
      lastHydratedAt: null,
      mode: "local",
    },
    llmRuntime: {
      baseUrl: "",
      providerId: "",
      model: "",
    },
    installedPlugins: nextState.installedPlugins.filter((item) => item.pluginId === "tidb.ai"),
    googleAuth: null,
  };
}

function persistLocalSettings(): void {
  if (store.getSnapshot().googleAuth?.accessToken?.trim()) return;
  writeLocalRuntimeState(buildLocalRuntimeSettingsInput());
}

function persistLocalDisplaySettings(): void {
  writeLocalDisplaySettings({
    assistantReplyFontSize: settings.assistantReplyFontSize,
  });
}

function buildStoredAppSettingsInput(): Omit<StoredAppSettingsRecord, "updatedAt"> {
  const nextState = cloneRuntimeState(settings);
  return {
    activeCaseId: nextState.activeCaseId,
    analyticsConsent: nextState.analyticsConsent,
    llmRuntime: nextState.llmRuntime,
    installedPlugins: nextState.installedPlugins,
  };
}

function persistCloudSettings(): void {
  if (!isCloudSyncEnabled(store.getSnapshot())) return;
  void saveStoredAppSettings(getAppSettingsSnapshot(), buildStoredAppSettingsInput());
}

function persistCloudCaseCreate(caseWorkspace: RuntimeState["cases"][number]): void {
  if (!isCloudSyncEnabled(store.getSnapshot())) return;
  void createStoredCase(getAppSettingsSnapshot(), caseWorkspace);
}

function persistCloudCaseUpdate(
  caseId: string,
  patch: Partial<RuntimeState["cases"][number]>,
): void {
  if (!isCloudSyncEnabled(store.getSnapshot())) return;
  void updateStoredCase(getAppSettingsSnapshot(), caseId, patch);
}

function persistCloudCaseDelete(caseId: string): void {
  if (!isCloudSyncEnabled(store.getSnapshot())) return;
  void deleteStoredCase(getAppSettingsSnapshot(), caseId);
}

const caseOps = createRuntimeCaseOps({
  settings,
  getSnapshot: () => store.getSnapshot(),
  commit: () => store.commit(),
  hasStoredCaseHistory,
  persistLocalSettings,
  persistCloudCaseCreate,
  persistCloudCaseDelete,
  persistCloudCaseUpdate,
  persistCloudSettings,
  clearLocalCaseHistory,
});

const cloudSync = createRuntimeCloudSync({
  settings,
  getSnapshot: () => store.getSnapshot(),
  commit: () => store.commit(),
  readLocalRuntimeState,
  getAppClientId,
  clearLocalCaseHistory,
  createCaseHistoryAdapter,
  importStoredCases,
  listStoredCases,
  getStoredAppSettings,
  persistCloudSettings,
});

Object.assign(
  settings,
  normalizeNewSettings(readLocalRuntimeState() ?? { installedPlugins: [buildTidbAiInstalledPlugin()] }),
);
cloudSync.bootstrapFromLegacyStorage();

export function getAppSettings(): AppRuntimeSettings {
  return store.getPublicSettings();
}

export function getAppSettingsSnapshot(): AppRuntimeSettings {
  return store.getSnapshot();
}

export function setAppSettings(partial: Partial<AppRuntimeSettings>) {
  store.assign(normalizeNewSettings({ ...cloneRuntimeState(settings), ...partial }));
  store.commit();
  persistLocalSettings();
  persistLocalDisplaySettings();
}

export async function ensureGoogleAuthSession(): Promise<GoogleAuthState | null> {
  if (settings.googleAuth?.accessToken?.trim()) {
    return settings.googleAuth;
  }
  if (autoAuthDisabled || !isGoogleOAuthConfigured()) {
    return null;
  }
  if (authBootstrapPromise) {
    return authBootstrapPromise;
  }

  authBootstrapPromise = (async () => {
    try {
      const googleAuth = await refreshGoogleAuthSession();
      settings.googleAuth = { ...googleAuth };
      store.commit();
      return googleAuth;
    } catch {
      return null;
    } finally {
      authBootstrapPromise = null;
    }
  })();

  return authBootstrapPromise;
}

export async function syncCloudCasesIfNeeded(): Promise<void> {
  return cloudSync.syncCloudCasesIfNeeded();
}

export function setAnalyticsConsent(consent: AnalyticsConsentState): void {
  settings.analyticsConsent = normalizeAnalyticsConsent(consent);
  store.commit();
  persistLocalSettings();
  persistCloudSettings();
}

export function createCase(title: string, pluginId?: PluginId) {
  return caseOps.createCase(title, pluginId);
}

export function setActiveCaseId(caseId: string): void {
  caseOps.setActiveCaseId(caseId);
}

export function renameCase(caseId: string, title: string): void {
  caseOps.renameCase(caseId, title);
}

export function resolveCase(caseId: string): void {
  caseOps.resolveCase(caseId);
}

export function reopenCase(caseId: string): void {
  caseOps.reopenCase(caseId);
}

export function archiveCase(caseId: string): void {
  caseOps.archiveCase(caseId);
}

export function unarchiveCase(caseId: string): void {
  caseOps.unarchiveCase(caseId);
}

export function deleteCase(caseId: string): void {
  caseOps.deleteCase(caseId);
}

export function updateInstalledPluginConfig(
  pluginId: "tidb.ai",
  partial: Partial<TidbAiPluginConfig>,
): void;
export function updateInstalledPluginConfig(
  pluginId: "websearch",
  partial: Partial<WebSearchPluginConfig>,
): void;
export function updateInstalledPluginConfig(
  pluginId: PluginId,
  partial: Partial<TidbAiPluginConfig> | Partial<WebSearchPluginConfig>,
): void {
  caseOps.updateInstalledPluginConfig(pluginId, partial);
}

export function updateGlobalLlmRuntime(partial: Partial<GlobalLlmRuntimeConfig>): void {
  caseOps.updateGlobalLlmRuntime(partial);
}

export function updateAssistantReplyFontSize(value: AssistantReplyFontSize): void {
  settings.assistantReplyFontSize = normalizeAssistantReplyFontSize(value);
  store.commit();
  persistLocalDisplaySettings();
}

export function autoTitleCaseFromPrompt(caseId: string, prompt: string): void {
  caseOps.autoTitleCaseFromPrompt(caseId, prompt);
}

export function setGoogleAuth(googleAuth: GoogleAuthState): void {
  autoAuthDisabled = false;
  settings.googleAuth = { ...googleAuth };
  store.commit();
}

export function refreshGoogleAuth(partial: Partial<GoogleAuthState>): void {
  autoAuthDisabled = false;
  if (!settings.googleAuth) {
    if (!partial.accessToken) return;
    settings.googleAuth = {
      accessToken: partial.accessToken,
      clientId: partial.clientId ?? "",
      email: partial.email ?? "",
      hostedDomain: partial.hostedDomain ?? "",
      expiresAt: partial.expiresAt ?? null,
    };
    store.commit();
    return;
  }

  settings.googleAuth = {
    ...settings.googleAuth,
    ...partial,
    accessToken: partial.accessToken ?? settings.googleAuth.accessToken,
    clientId: partial.clientId ?? settings.googleAuth.clientId,
    email: partial.email ?? settings.googleAuth.email,
    hostedDomain: partial.hostedDomain ?? settings.googleAuth.hostedDomain,
    expiresAt:
      partial.expiresAt === undefined
        ? settings.googleAuth.expiresAt
        : partial.expiresAt,
  };
  store.commit();
}

export function clearGoogleAuth(): void {
  autoAuthDisabled = true;
  cloudSync.resetSignedOutState();
}

export function subscribeAppSettings(listener: () => void): () => void {
  return store.subscribe(listener);
}
