import type { ExportedMessageRepository } from "@assistant-ui/react";
import type { AppRuntimeSettings } from "@/lib/chat/agent-types";

export const LOCAL_RUNTIME_STATE_KEY = "tihc_local_runtime_state_v2";
export const LOCAL_DISPLAY_SETTINGS_KEY = "tihc_local_display_settings_v1";
export const LOCAL_HISTORY_PREFIX = "tihc_local_history_v2:";

function getBrowserStorage(): Storage | null {
  try {
    const storage = globalThis.localStorage;
    if (
      storage &&
      typeof storage.getItem === "function" &&
      typeof storage.setItem === "function" &&
      typeof storage.removeItem === "function" &&
      typeof storage.key === "function"
    ) {
      return storage;
    }
    return null;
  } catch {
    return null;
  }
}

function parseJson<T>(raw: string | null): T | null {
  if (!raw?.trim()) return null;

  try {
    return JSON.parse(raw) as T;
  } catch {
    return null;
  }
}

export function readLocalRuntimeState(): Partial<AppRuntimeSettings> | null {
  const runtimeState =
    parseJson<Partial<AppRuntimeSettings>>(getBrowserStorage()?.getItem(LOCAL_RUNTIME_STATE_KEY) ?? null) ??
    {};
  const displaySettings = readLocalDisplaySettings() ?? {};
  const merged = {
    ...runtimeState,
    ...displaySettings,
  };
  return Object.keys(merged).length ? merged : null;
}

export function writeLocalRuntimeState(state: Partial<AppRuntimeSettings>): void {
  const storage = getBrowserStorage();
  if (!storage) return;

  storage.setItem(LOCAL_RUNTIME_STATE_KEY, JSON.stringify(state));
}

export function removeLocalRuntimeState(): void {
  getBrowserStorage()?.removeItem(LOCAL_RUNTIME_STATE_KEY);
}

export function readLocalDisplaySettings(): Partial<Pick<AppRuntimeSettings, "assistantReplyFontSize">> | null {
  return parseJson<Partial<Pick<AppRuntimeSettings, "assistantReplyFontSize">>>(
    getBrowserStorage()?.getItem(LOCAL_DISPLAY_SETTINGS_KEY) ?? null,
  );
}

export function writeLocalDisplaySettings(
  settings: Partial<Pick<AppRuntimeSettings, "assistantReplyFontSize">>,
): void {
  const storage = getBrowserStorage();
  if (!storage) return;

  storage.setItem(LOCAL_DISPLAY_SETTINGS_KEY, JSON.stringify(settings));
}

function historyKey(scope: string): string {
  return `${LOCAL_HISTORY_PREFIX}${scope}`;
}

export function readLocalHistory(scope: string): ExportedMessageRepository | null {
  return parseJson<ExportedMessageRepository>(getBrowserStorage()?.getItem(historyKey(scope)) ?? null);
}

export function writeLocalHistory(scope: string, repository: ExportedMessageRepository): void {
  const storage = getBrowserStorage();
  if (!storage) return;

  storage.setItem(historyKey(scope), JSON.stringify(repository));
}

export function removeLocalHistory(scope: string): void {
  getBrowserStorage()?.removeItem(historyKey(scope));
}

export function listLocalHistoryScopes(): string[] {
  const storage = getBrowserStorage();
  if (!storage) return [];

  const scopes: string[] = [];
  for (let index = 0; index < storage.length; index += 1) {
    const key = storage.key(index);
    if (!key?.startsWith(LOCAL_HISTORY_PREFIX)) continue;
    scopes.push(key.slice(LOCAL_HISTORY_PREFIX.length));
  }
  return scopes;
}
