import type { ExportedMessageRepository } from "@assistant-ui/react";
import type {
  AppRuntimeSettings,
  CaseWorkspace,
  GoogleAuthState,
  StoredAppSettingsRecord,
  StoredCaseRecord,
} from "@/lib/chat/agent-types";
import {
  LEGACY_STORAGE_KEYS,
  LEGACY_STORAGE_PREFIXES,
  cloneCaseWorkspace,
  normalizeNewSettings,
  parseLegacySettingsPayload,
  selectFallbackCaseId,
  toCaseWorkspace,
  type RuntimeState,
} from "@/lib/app/runtime-state";

type ExtensionStorageAreaLike = {
  remove?: (
    keys: string | string[],
    callback?: () => void,
  ) => Promise<void> | void;
};

type RuntimeCloudSyncDeps = {
  settings: RuntimeState;
  getSnapshot(): AppRuntimeSettings;
  commit(): void;
  readLocalRuntimeState(): Partial<AppRuntimeSettings> | null;
  getAppClientId(): Promise<string>;
  clearLocalCaseHistory(caseId?: string): void;
  createCaseHistoryAdapter(caseId: string): {
    load(): Promise<ExportedMessageRepository>;
  };
  importStoredCases(
    settings: AppRuntimeSettings,
    clientId: string,
    cases: CaseWorkspace[],
    historiesByCaseId: Record<string, ExportedMessageRepository>,
  ): Promise<unknown>;
  listStoredCases(settings: AppRuntimeSettings): Promise<StoredCaseRecord[] | null>;
  getStoredAppSettings(settings: AppRuntimeSettings): Promise<StoredAppSettingsRecord | null>;
  persistCloudSettings(): void;
};

function clearLegacyLocalPersistence() {
  const root = globalThis as typeof globalThis & {
    browser?: {
      storage?: {
        local?: ExtensionStorageAreaLike;
        session?: ExtensionStorageAreaLike;
      };
    };
    chrome?: {
      storage?: {
        local?: ExtensionStorageAreaLike;
        session?: ExtensionStorageAreaLike;
      };
    };
    localStorage?: Storage;
  };

  try {
    const storage = root.localStorage;
    if (storage && typeof storage.removeItem === "function") {
      // v1 now hydrates from runtime memory and cloud storage, so stale local keys
      // would re-introduce phantom cases/settings after sign-in.
      for (const key of LEGACY_STORAGE_KEYS) {
        storage.removeItem(key);
      }

      const keysToRemove: string[] = [];
      for (let index = 0; index < storage.length; index += 1) {
        const key = storage.key(index);
        if (!key) continue;
        if (LEGACY_STORAGE_PREFIXES.some((prefix) => key.startsWith(prefix))) {
          keysToRemove.push(key);
        }
      }
      for (const key of keysToRemove) {
        storage.removeItem(key);
      }
    }
  } catch {
    // ignore legacy cleanup failures
  }

  const removeFromArea = (area: ExtensionStorageAreaLike | undefined, keys: string[]) => {
    if (!area?.remove) return;
    try {
      const raw = area.remove(keys);
      if (raw && typeof (raw as Promise<void>).catch === "function") {
        void (raw as Promise<void>).catch(() => undefined);
      }
    } catch {
      // ignore
    }
  };

  const extensionLocal = root.browser?.storage?.local ?? root.chrome?.storage?.local;
  const extensionSession = root.browser?.storage?.session ?? root.chrome?.storage?.session;
  removeFromArea(extensionLocal, [
    "tihc_app_client_id_v1",
    "tihc_telemetry_client_id_v1",
  ]);
  removeFromArea(extensionSession, [
    "tihc_telemetry_session_id_v1",
    "tihc_telemetry_session_seen_at_v1",
  ]);
}

function readLegacyLocalSettings(): Partial<AppRuntimeSettings> | null {
  const root = globalThis as typeof globalThis & { localStorage?: Storage };
  const storage = root.localStorage;
  if (!storage || typeof storage.getItem !== "function") return null;

  return (
    parseLegacySettingsPayload(storage.getItem("tihc_app_settings_v1")) ??
    parseLegacySettingsPayload(storage.getItem("tihc_local_mode_state_v1"))
  );
}

export function createRuntimeCloudSync({
  settings,
  getSnapshot,
  commit,
  readLocalRuntimeState,
  getAppClientId,
  clearLocalCaseHistory,
  createCaseHistoryAdapter,
  importStoredCases,
  listStoredCases,
  getStoredAppSettings,
  persistCloudSettings,
}: RuntimeCloudSyncDeps) {
  let cloudSyncPromise: Promise<void> | null = null;

  const switchToCloudState = (
    storedCases: StoredCaseRecord[],
    storedSettings: StoredAppSettingsRecord | null,
    clientId: string,
  ): void => {
    Object.assign(
      settings,
      normalizeNewSettings({
        activeCaseId: storedSettings?.activeCaseId ?? settings.activeCaseId,
        analyticsConsent: storedSettings?.analyticsConsent ?? settings.analyticsConsent,
        cloudSync: {
          importedClientId: clientId,
          lastHydratedAt: new Date().toISOString(),
          mode: "cloud",
        },
        googleAuth: settings.googleAuth,
        llmRuntime: storedSettings?.llmRuntime ?? settings.llmRuntime,
        installedPlugins: storedSettings?.installedPlugins ?? settings.installedPlugins,
        cases: storedCases.map(toCaseWorkspace),
      }),
    );
    settings.activeCaseId = selectFallbackCaseId(
      settings.cases,
      storedSettings?.activeCaseId ?? settings.activeCaseId,
    );
    commit();
  };

  const resetSignedOutState = (): void => {
    Object.assign(settings, normalizeNewSettings(readLocalRuntimeState() ?? {}));
    settings.googleAuth = null;
    commit();
  };

  const collectLocalHistories = async (
    cases: CaseWorkspace[],
  ): Promise<Record<string, ExportedMessageRepository>> => {
    const historiesByCaseId: Record<string, ExportedMessageRepository> = {};

    for (const caseWorkspace of cases) {
      historiesByCaseId[caseWorkspace.id] = await createCaseHistoryAdapter(caseWorkspace.id).load();
    }

    return historiesByCaseId;
  };

  return {
    bootstrapFromLegacyStorage(): void {
      const legacySettings = readLegacyLocalSettings();
      if (legacySettings) {
        Object.assign(settings, normalizeNewSettings(legacySettings));
      }
      clearLegacyLocalPersistence();
      commit();
    },

    resetSignedOutState,

    async syncCloudCasesIfNeeded(): Promise<void> {
      if (cloudSyncPromise) return cloudSyncPromise;

      cloudSyncPromise = (async () => {
        if (!settings.googleAuth?.accessToken?.trim()) {
          if (settings.cloudSync.mode === "cloud") {
            resetSignedOutState();
          }
          return;
        }

        const localCases =
          settings.cloudSync.mode === "local" ? settings.cases.map(cloneCaseWorkspace) : [];
        const clientId = await getAppClientId();

        if (
          settings.cloudSync.mode === "local" &&
          settings.cloudSync.importedClientId !== clientId &&
          localCases.length
        ) {
          // Import only once per client id so repeated auth refreshes do not duplicate local cases.
          const importResult = await importStoredCases(
            getSnapshot(),
            clientId,
            localCases,
            await collectLocalHistories(localCases),
          );
          if (!importResult) return;
          settings.cloudSync.importedClientId = clientId;
        }

        const [storedCases, storedSettings] = await Promise.all([
          listStoredCases(getSnapshot()),
          getStoredAppSettings(getSnapshot()),
        ]);
        if (!storedCases) return;

        const hadStoredSettings = Boolean(storedSettings);
        switchToCloudState(storedCases, storedSettings, clientId);
        if (!hadStoredSettings) {
          persistCloudSettings();
        }
      })().finally(() => {
        cloudSyncPromise = null;
      });

      return cloudSyncPromise;
    },
  };
}
