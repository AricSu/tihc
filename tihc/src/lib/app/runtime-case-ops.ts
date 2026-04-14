import type {
  AppRuntimeSettings,
  CaseWorkspace,
  GlobalLlmRuntimeConfig,
  PluginId,
  TidbAiPluginConfig,
  WebSearchPluginConfig,
} from "@/lib/chat/agent-types";
import {
  TIDB_PLUGIN_ID,
  UNTITLED_CASE_TITLES,
  WEBSEARCH_PLUGIN_ID,
  buildCaseWorkspace,
  buildTidbAiInstalledPlugin,
  buildWebSearchInstalledPlugin,
  cloneCaseWorkspace,
  deriveThreadTitleFromPrompt,
  normalizeGlobalLlmRuntime,
  selectFallbackCaseId,
  type RuntimeState,
} from "@/lib/app/runtime-state";
import { isAnonymousLocalStorageLimitReached } from "@/lib/app/anonymous-local-case-limit";

type RuntimeCaseOpsDeps = {
  settings: RuntimeState;
  getSnapshot(): AppRuntimeSettings;
  commit(): void;
  hasStoredCaseHistory(caseId: string): boolean;
  persistLocalSettings(): void;
  persistCloudCaseCreate(caseWorkspace: CaseWorkspace): void;
  persistCloudCaseDelete(caseId: string): void;
  persistCloudCaseUpdate(caseId: string, patch: Partial<CaseWorkspace>): void;
  persistCloudSettings(): void;
  clearLocalCaseHistory(caseId: string): void;
};

type PersistOptions = {
  persistSettings?: boolean;
};

export function createRuntimeCaseOps({
  settings,
  getSnapshot,
  commit,
  hasStoredCaseHistory,
  persistLocalSettings,
  persistCloudCaseCreate,
  persistCloudCaseDelete,
  persistCloudCaseUpdate,
  persistCloudSettings,
  clearLocalCaseHistory,
}: RuntimeCaseOpsDeps) {
  const updateCase = (
    caseId: string,
    updater: (current: CaseWorkspace) => CaseWorkspace,
  ): CaseWorkspace | null => {
    const index = settings.cases.findIndex((item) => item.id === caseId);
    if (index < 0) return null;
    const nextCase = updater(settings.cases[index]!);
    settings.cases[index] = nextCase;
    return nextCase;
  };

  const updateCaseWithTimestamp = (
    caseId: string,
    updater: (current: CaseWorkspace, now: string) => Partial<CaseWorkspace>,
  ): CaseWorkspace | null => {
    const now = new Date().toISOString();
    return updateCase(caseId, (current) =>
      buildCaseWorkspace({
        ...current,
        id: current.id,
        ...updater(current, now),
        updatedAt: now,
      }),
    );
  };

  const commitCaseUpdate = (
    caseWorkspace: CaseWorkspace | null,
    patch: Partial<CaseWorkspace>,
    options: PersistOptions = {},
  ): void => {
    if (!caseWorkspace) return;
    commit();
    persistLocalSettings();
    persistCloudCaseUpdate(caseWorkspace.id, patch);
    if (options.persistSettings) {
      persistCloudSettings();
    }
  };

  const getDefaultPluginId = (): PluginId =>
    settings.installedPlugins.find((item) => item.pluginId === TIDB_PLUGIN_ID)?.pluginId ??
    TIDB_PLUGIN_ID;

  return {
    createCase(title: string, pluginId: PluginId = getDefaultPluginId()): CaseWorkspace | null {
      if (!settings.installedPlugins.some((item) => item.pluginId === pluginId)) return null;

      const nextCase = buildCaseWorkspace({
        title,
        pluginId,
      });
      if (
        isAnonymousLocalStorageLimitReached({
          ...getSnapshot(),
          activeCaseId: nextCase.id,
          cases: [...settings.cases, nextCase],
        })
      ) {
        return null;
      }
      settings.cases.push(nextCase);
      settings.activeCaseId = nextCase.id;
      commit();
      persistLocalSettings();
      persistCloudCaseCreate(nextCase);
      persistCloudSettings();
      return cloneCaseWorkspace(nextCase);
    },

    setActiveCaseId(caseId: string): void {
      if (!settings.cases.some((item) => item.id === caseId)) return;
      settings.activeCaseId = caseId;
      commit();
      persistLocalSettings();
      persistCloudSettings();
    },

    renameCase(caseId: string, title: string): void {
      const nextTitle = title.trim();
      if (!nextTitle) return;
      const nextCase = updateCaseWithTimestamp(caseId, () => ({
        title: nextTitle,
      }));
      if (!nextCase) return;
      commitCaseUpdate(nextCase, {
        title: nextCase.title,
        updatedAt: nextCase.updatedAt,
      });
    },

    resolveCase(caseId: string): void {
      const nextCase = updateCaseWithTimestamp(caseId, (_, now) => ({
        activityState: "resolved",
        resolvedAt: now,
      }));
      if (!nextCase) return;
      commitCaseUpdate(nextCase, {
        activityState: nextCase.activityState,
        resolvedAt: nextCase.resolvedAt,
        updatedAt: nextCase.updatedAt,
      });
    },

    reopenCase(caseId: string): void {
      const hasHistory = getSnapshot().cloudSync.mode === "cloud" ? true : hasStoredCaseHistory(caseId);
      const nextCase = updateCaseWithTimestamp(caseId, () => ({
        activityState: hasHistory ? "active" : "ready",
        resolvedAt: null,
      }));
      if (!nextCase) return;
      commitCaseUpdate(nextCase, {
        activityState: nextCase.activityState,
        resolvedAt: nextCase.resolvedAt,
        updatedAt: nextCase.updatedAt,
      });
    },

    archiveCase(caseId: string): void {
      const nextCase = updateCaseWithTimestamp(caseId, (_, now) => ({
        archivedAt: now,
      }));
      if (!nextCase) return;
      if (settings.activeCaseId === nextCase.id) {
        settings.activeCaseId = selectFallbackCaseId(settings.cases, null);
      }
      commitCaseUpdate(
        nextCase,
        {
          archivedAt: nextCase.archivedAt,
          updatedAt: nextCase.updatedAt,
        },
        { persistSettings: true },
      );
    },

    unarchiveCase(caseId: string): void {
      const nextCase = updateCaseWithTimestamp(caseId, () => ({
        archivedAt: null,
      }));
      if (!nextCase) return;
      if (!settings.activeCaseId) {
        settings.activeCaseId = nextCase.id;
      }
      commitCaseUpdate(
        nextCase,
        {
          archivedAt: nextCase.archivedAt,
          updatedAt: nextCase.updatedAt,
        },
        { persistSettings: true },
      );
    },

    deleteCase(caseId: string): void {
      const existing = settings.cases.find((item) => item.id === caseId);
      if (!existing) return;

      clearLocalCaseHistory(caseId);
      settings.cases = settings.cases.filter((item) => item.id !== caseId);
      if (settings.activeCaseId === caseId) {
        settings.activeCaseId = selectFallbackCaseId(settings.cases, null);
      } else if (
        settings.activeCaseId &&
        !settings.cases.some((item) => item.id === settings.activeCaseId)
      ) {
        settings.activeCaseId = selectFallbackCaseId(settings.cases, null);
      }
      commit();
      persistLocalSettings();
      persistCloudCaseDelete(caseId);
      persistCloudSettings();
    },

    updateInstalledPluginConfig(
      pluginId: PluginId,
      partial: Partial<TidbAiPluginConfig> | Partial<WebSearchPluginConfig>,
    ): void {
      const index = settings.installedPlugins.findIndex((item) => item.pluginId === pluginId);
      if (index < 0) return;
      const currentPlugin = settings.installedPlugins[index]!;
      if (pluginId === TIDB_PLUGIN_ID) {
        settings.installedPlugins[index] = buildTidbAiInstalledPlugin({
          ...(currentPlugin.pluginId === TIDB_PLUGIN_ID ? currentPlugin.config : {}),
          ...(partial as Partial<TidbAiPluginConfig>),
        });
      } else {
        settings.installedPlugins[index] = buildWebSearchInstalledPlugin({
          ...(currentPlugin.pluginId === WEBSEARCH_PLUGIN_ID ? currentPlugin.config : {}),
          ...(partial as Partial<WebSearchPluginConfig>),
        });
      }
      commit();
      persistLocalSettings();
      persistCloudSettings();
    },

    updateGlobalLlmRuntime(partial: Partial<GlobalLlmRuntimeConfig>): void {
      settings.llmRuntime = normalizeGlobalLlmRuntime({
        ...settings.llmRuntime,
        ...partial,
      });
      commit();
      persistLocalSettings();
      persistCloudSettings();
    },

    autoTitleCaseFromPrompt(caseId: string, prompt: string): void {
      const nextCase = updateCaseWithTimestamp(caseId, (current) => ({
        title: UNTITLED_CASE_TITLES.has(current.title.trim().toLowerCase())
          ? deriveThreadTitleFromPrompt(prompt)
          : current.title,
        activityState: current.activityState === "ready" ? "active" : current.activityState,
      }));
      if (!nextCase) return;
      commitCaseUpdate(nextCase, {
        activityState: nextCase.activityState,
        title: nextCase.title,
        updatedAt: nextCase.updatedAt,
      });
    },
  };
}
