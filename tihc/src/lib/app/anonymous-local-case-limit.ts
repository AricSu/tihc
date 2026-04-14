import {
  LOCAL_HISTORY_PREFIX,
  LOCAL_RUNTIME_STATE_KEY,
} from "@/lib/app/local-browser-persistence";
import type { AppRuntimeSettings, CaseWorkspace } from "@/lib/chat/agent-types";

export const ANONYMOUS_LOCAL_STORAGE_LIMIT_BYTES = 2 * 1024 * 1024;

function parseUpdatedAt(value: string): number {
  const parsed = Date.parse(value);
  return Number.isNaN(parsed) ? 0 : parsed;
}

function getBrowserStorage(): Storage | null {
  try {
    const storage = globalThis.localStorage;
    if (
      storage &&
      typeof storage.getItem === "function" &&
      typeof storage.key === "function" &&
      typeof storage.length === "number"
    ) {
      return storage;
    }
    return null;
  } catch {
    return null;
  }
}

function estimateEntryBytes(key: string, raw: string): number {
  return (key.length + raw.length) * 2;
}

function estimateJsonEntryBytes(key: string, value: unknown): number {
  return estimateEntryBytes(key, JSON.stringify(value));
}

function buildAnonymousLocalRuntimeState(
  settings: Pick<
    AppRuntimeSettings,
    "activeCaseId" | "analyticsConsent" | "cases" | "installedPlugins"
  >,
): Partial<AppRuntimeSettings> {
  return {
    activeCaseId: settings.activeCaseId,
    analyticsConsent: settings.analyticsConsent,
    cases: settings.cases.map((item) => ({ ...item })),
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
    installedPlugins: settings.installedPlugins.filter((item) => item.pluginId === "tidb.ai"),
    googleAuth: null,
  };
}

function listHistoryEntries(): Array<{ key: string; raw: string }> {
  const storage = getBrowserStorage();
  if (!storage) return [];

  const entries: Array<{ key: string; raw: string }> = [];
  for (let index = 0; index < storage.length; index += 1) {
    const key = storage.key(index);
    if (!key?.startsWith(LOCAL_HISTORY_PREFIX)) continue;
    const raw = storage.getItem(key);
    if (typeof raw !== "string") continue;
    entries.push({ key, raw });
  }
  return entries;
}

export function isAnonymousLocalMode(
  settings: Pick<AppRuntimeSettings, "googleAuth">,
): boolean {
  return !settings.googleAuth?.accessToken?.trim();
}

export function estimateAnonymousLocalStorageUsageBytes(
  settings: Pick<
    AppRuntimeSettings,
    "activeCaseId" | "analyticsConsent" | "cases" | "googleAuth" | "installedPlugins"
  >,
): number {
  const runtimeBytes = estimateJsonEntryBytes(
    LOCAL_RUNTIME_STATE_KEY,
    buildAnonymousLocalRuntimeState(settings),
  );
  const historyBytes = listHistoryEntries().reduce(
    (total, entry) => total + estimateEntryBytes(entry.key, entry.raw),
    0,
  );

  return runtimeBytes + historyBytes;
}

export function estimateCaseLocalStorageBytes(caseWorkspace: CaseWorkspace): number {
  const metadataBytes = estimateJsonEntryBytes(`case:${caseWorkspace.id}`, caseWorkspace);
  const historyBytes = listHistoryEntries()
    .filter(
      (entry) =>
        entry.key === `${LOCAL_HISTORY_PREFIX}${caseWorkspace.id}` ||
        entry.key.startsWith(`${LOCAL_HISTORY_PREFIX}${caseWorkspace.id}:`),
    )
    .reduce((total, entry) => total + estimateEntryBytes(entry.key, entry.raw), 0);

  return metadataBytes + historyBytes;
}

export function isAnonymousLocalStorageLimitReached(
  settings: Pick<
    AppRuntimeSettings,
    "activeCaseId" | "analyticsConsent" | "cases" | "googleAuth" | "installedPlugins"
  >,
): boolean {
  return (
    isAnonymousLocalMode(settings) &&
    estimateAnonymousLocalStorageUsageBytes(settings) >= ANONYMOUS_LOCAL_STORAGE_LIMIT_BYTES
  );
}

export function formatStorageBytes(bytes: number): string {
  if (bytes >= 1024 * 1024) {
    return `${(bytes / (1024 * 1024)).toFixed(bytes >= 10 * 1024 * 1024 ? 1 : 2)} MB`;
  }
  if (bytes >= 1024) {
    return `${(bytes / 1024).toFixed(bytes >= 10 * 1024 ? 1 : 2)} KB`;
  }
  return `${bytes} B`;
}

export function sortCasesByLocalStorageBytesDesc(cases: CaseWorkspace[]): CaseWorkspace[] {
  return [...cases].sort((left, right) => {
    const sizeDelta = estimateCaseLocalStorageBytes(right) - estimateCaseLocalStorageBytes(left);
    if (sizeDelta !== 0) return sizeDelta;
    return parseUpdatedAt(right.updatedAt) - parseUpdatedAt(left.updatedAt);
  });
}
