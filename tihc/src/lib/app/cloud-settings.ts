import type {
  AppRuntimeSettings,
  LlmProviderCatalogEntry,
  StoredLlmCredentialStatus,
  StoredAppSettingsRecord,
} from "@/lib/chat/agent-types";
import { resolveBackendEndpoint } from "@/lib/app/backend-endpoint";

function buildRequestHeaders(
  settings: AppRuntimeSettings,
  includeContentType = false,
): Record<string, string> | null {
  const accessToken = settings.googleAuth?.accessToken?.trim() ?? "";
  if (!accessToken) return null;

  const headers: Record<string, string> = {
    Authorization: `Bearer ${accessToken}`,
  };
  if (includeContentType) {
    headers["Content-Type"] = "application/json";
  }
  return headers;
}

function cloudSettingsEndpoint(settings: AppRuntimeSettings): string | null {
  return resolveBackendEndpoint(settings, "/v1/settings");
}

function llmProvidersEndpoint(settings: AppRuntimeSettings): string | null {
  return resolveBackendEndpoint(settings, "/v1/llm/providers");
}

function llmCredentialEndpoint(settings: AppRuntimeSettings, providerId: string): string | null {
  return resolveBackendEndpoint(settings, `/v1/llm/credentials/${encodeURIComponent(providerId)}`);
}

export async function getStoredAppSettings(
  settings: AppRuntimeSettings,
): Promise<StoredAppSettingsRecord | null> {
  const endpoint = cloudSettingsEndpoint(settings);
  const headers = buildRequestHeaders(settings);
  if (!endpoint || !headers) return null;

  const response = await fetch(endpoint, {
    method: "GET",
    headers,
  }).catch(() => null);
  if (!response?.ok) return null;

  const payload = await response.json().catch(() => null);
  const storedSettings = (payload as { settings?: unknown } | null)?.settings;
  return storedSettings && typeof storedSettings === "object"
    ? (storedSettings as StoredAppSettingsRecord)
    : null;
}

export async function saveStoredAppSettings(
  settings: AppRuntimeSettings,
  nextSettings: Omit<StoredAppSettingsRecord, "updatedAt">,
): Promise<StoredAppSettingsRecord | null> {
  const endpoint = cloudSettingsEndpoint(settings);
  const headers = buildRequestHeaders(settings, true);
  if (!endpoint || !headers) return null;

  const response = await fetch(endpoint, {
    method: "PUT",
    headers,
    body: JSON.stringify(nextSettings),
  }).catch(() => null);
  if (!response?.ok) return null;

  const payload = await response.json().catch(() => null);
  const storedSettings = (payload as { settings?: unknown } | null)?.settings;
  return storedSettings && typeof storedSettings === "object"
    ? (storedSettings as StoredAppSettingsRecord)
    : null;
}

export async function listLlmProviders(
  settings: AppRuntimeSettings,
): Promise<LlmProviderCatalogEntry[]> {
  const endpoint = llmProvidersEndpoint(settings);
  if (!endpoint) return [];

  const headers = buildRequestHeaders(settings) ?? undefined;
  const response = await fetch(endpoint, {
    method: "GET",
    ...(headers ? { headers } : {}),
  }).catch(() => null);
  if (!response?.ok) return [];

  const payload = await response.json().catch(() => null);
  const providers = (payload as { providers?: unknown } | null)?.providers;
  return Array.isArray(providers) ? (providers as LlmProviderCatalogEntry[]) : [];
}

export async function getStoredLlmCredentialStatus(
  settings: AppRuntimeSettings,
  providerId: string,
): Promise<StoredLlmCredentialStatus | null> {
  const endpoint = llmCredentialEndpoint(settings, providerId);
  const headers = buildRequestHeaders(settings);
  if (!endpoint || !headers) return null;

  const response = await fetch(endpoint, {
    method: "GET",
    headers,
  }).catch(() => null);
  if (!response?.ok) return null;

  const payload = await response.json().catch(() => null);
  const credential = (payload as { credential?: unknown } | null)?.credential;
  return credential && typeof credential === "object"
    ? (credential as StoredLlmCredentialStatus)
    : null;
}

export async function saveStoredLlmCredential(
  settings: AppRuntimeSettings,
  input: { providerId: string; apiKey: string },
): Promise<StoredLlmCredentialStatus | null> {
  const endpoint = resolveBackendEndpoint(settings, "/v1/llm/credentials");
  const headers = buildRequestHeaders(settings, true);
  if (!endpoint || !headers) return null;

  const response = await fetch(endpoint, {
    method: "PUT",
    headers,
    body: JSON.stringify(input),
  }).catch(() => null);
  if (!response?.ok) return null;

  const payload = await response.json().catch(() => null);
  const credential = (payload as { credential?: unknown } | null)?.credential;
  return credential && typeof credential === "object"
    ? (credential as StoredLlmCredentialStatus)
    : null;
}
