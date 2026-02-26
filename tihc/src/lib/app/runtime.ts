export type ChatBackend = "serverless";

export type AppRuntimeSettings = {
  backend: ChatBackend;
  serverlessBaseUrl: string;
  chatEngine: string;
  googleClientId: string;
  googleToken: string;
};

const STORAGE_KEY = "tihc_app_settings_v1";

const settings: AppRuntimeSettings = {
  backend: "serverless",
  serverlessBaseUrl: "",
  chatEngine: "tidb",
  googleClientId: "",
  googleToken: "",
};

function hasLocalStorage(): boolean {
  return typeof globalThis !== "undefined" && typeof globalThis.localStorage !== "undefined";
}

function loadFromStorage() {
  if (!hasLocalStorage()) return;
  const raw = globalThis.localStorage.getItem(STORAGE_KEY);
  if (!raw) return;
  try {
    const parsed = JSON.parse(raw) as Partial<AppRuntimeSettings>;
    // Force serverless-only mode for now (WebLLM temporarily hidden).
    settings.backend = "serverless";
    if (typeof parsed.serverlessBaseUrl === "string") settings.serverlessBaseUrl = parsed.serverlessBaseUrl;
    if (typeof parsed.chatEngine === "string") settings.chatEngine = parsed.chatEngine;
    if (typeof parsed.googleClientId === "string") settings.googleClientId = parsed.googleClientId;
    // Back-compat: previously stored as googleAccessToken.
    const legacy = (parsed as unknown as { googleAccessToken?: unknown }).googleAccessToken;
    if (typeof legacy === "string") settings.googleToken = legacy;
    if (typeof parsed.googleToken === "string") settings.googleToken = parsed.googleToken;
  } catch {
    // ignore
  }
}

function saveToStorage() {
  if (!hasLocalStorage()) return;
  try {
    globalThis.localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
  } catch {
    // ignore
  }
}

loadFromStorage();

export function getAppSettings(): AppRuntimeSettings {
  return { ...settings };
}

export function setAppSettings(partial: Partial<AppRuntimeSettings>) {
  settings.backend = "serverless";
  if (typeof partial.serverlessBaseUrl === "string") settings.serverlessBaseUrl = partial.serverlessBaseUrl;
  if (typeof partial.chatEngine === "string") settings.chatEngine = partial.chatEngine;
  if (typeof partial.googleClientId === "string") settings.googleClientId = partial.googleClientId;
  if (typeof partial.googleToken === "string") settings.googleToken = partial.googleToken;
  saveToStorage();
}

export function clearGoogleAuth() {
  settings.googleToken = "";
  saveToStorage();
}
