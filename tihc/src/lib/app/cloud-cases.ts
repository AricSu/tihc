import type { ExportedMessageRepository } from "@assistant-ui/react";
import type {
  AppRuntimeSettings,
  CaseWorkspace,
  StoredCaseRecord,
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

function cloudCasesEndpoint(settings: AppRuntimeSettings): string | null {
  return resolveBackendEndpoint(settings, "/v1/cases");
}

function cloudCaseHistoryEndpoint(settings: AppRuntimeSettings, caseId: string): string | null {
  return resolveBackendEndpoint(settings, `/v1/cases/${caseId}/history`);
}

function cloudCaseImportEndpoint(settings: AppRuntimeSettings): string | null {
  return resolveBackendEndpoint(settings, "/v1/cases/import");
}

export function isCloudSyncEnabled(settings: AppRuntimeSettings): boolean {
  return settings.cloudSync.mode === "cloud" && Boolean(settings.googleAuth?.accessToken?.trim());
}

export async function listStoredCases(
  settings: AppRuntimeSettings,
): Promise<StoredCaseRecord[] | null> {
  const endpoint = cloudCasesEndpoint(settings);
  const headers = buildRequestHeaders(settings);
  if (!endpoint || !headers) return null;

  const response = await fetch(endpoint, {
    method: "GET",
    headers,
  }).catch(() => null);
  if (!response?.ok) return null;

  const payload = await response.json().catch(() => null);
  const cases = (payload as { cases?: unknown } | null)?.cases;
  return Array.isArray(cases) ? (cases as StoredCaseRecord[]) : null;
}

export async function createStoredCase(
  settings: AppRuntimeSettings,
  caseWorkspace: CaseWorkspace,
): Promise<StoredCaseRecord | null> {
  const endpoint = cloudCasesEndpoint(settings);
  const headers = buildRequestHeaders(settings, true);
  if (!endpoint || !headers) return null;

  const response = await fetch(endpoint, {
    method: "POST",
    headers,
    body: JSON.stringify(caseWorkspace),
  }).catch(() => null);
  if (!response?.ok) return null;

  const payload = await response.json().catch(() => null);
  const storedCase = (payload as { case?: unknown } | null)?.case;
  return storedCase && typeof storedCase === "object" ? (storedCase as StoredCaseRecord) : null;
}

export async function updateStoredCase(
  settings: AppRuntimeSettings,
  caseId: string,
  patch: Partial<CaseWorkspace>,
): Promise<StoredCaseRecord | null> {
  const endpoint = resolveBackendEndpoint(settings, `/v1/cases/${caseId}`);
  const headers = buildRequestHeaders(settings, true);
  if (!endpoint || !headers) return null;

  const response = await fetch(endpoint, {
    method: "PATCH",
    headers,
    body: JSON.stringify(patch),
  }).catch(() => null);
  if (!response?.ok) return null;

  const payload = await response.json().catch(() => null);
  const storedCase = (payload as { case?: unknown } | null)?.case;
  return storedCase && typeof storedCase === "object" ? (storedCase as StoredCaseRecord) : null;
}

export async function deleteStoredCase(
  settings: AppRuntimeSettings,
  caseId: string,
): Promise<boolean> {
  const endpoint = resolveBackendEndpoint(settings, `/v1/cases/${caseId}`);
  const headers = buildRequestHeaders(settings);
  if (!endpoint || !headers) return false;

  const response = await fetch(endpoint, {
    method: "DELETE",
    headers,
  }).catch(() => null);
  return Boolean(response?.ok);
}

export async function getStoredCaseHistory(
  settings: AppRuntimeSettings,
  caseId: string,
): Promise<ExportedMessageRepository | null> {
  const endpoint = cloudCaseHistoryEndpoint(settings, caseId);
  const headers = buildRequestHeaders(settings);
  if (!endpoint || !headers) return null;

  const response = await fetch(endpoint, {
    method: "GET",
    headers,
  }).catch(() => null);
  if (!response?.ok) return null;

  const payload = await response.json().catch(() => null);
  const repository = (payload as { repository?: unknown } | null)?.repository;
  return repository && typeof repository === "object"
    ? (repository as ExportedMessageRepository)
    : null;
}

export async function saveStoredCaseHistory(
  settings: AppRuntimeSettings,
  caseId: string,
  repository: ExportedMessageRepository,
): Promise<StoredCaseRecord | null> {
  const endpoint = cloudCaseHistoryEndpoint(settings, caseId);
  const headers = buildRequestHeaders(settings, true);
  if (!endpoint || !headers) return null;

  const response = await fetch(endpoint, {
    method: "PUT",
    headers,
    body: JSON.stringify({ repository }),
  }).catch(() => null);
  if (!response?.ok) return null;

  const payload = await response.json().catch(() => null);
  const storedCase = (payload as { case?: unknown } | null)?.case;
  return storedCase && typeof storedCase === "object" ? (storedCase as StoredCaseRecord) : null;
}

export async function importStoredCases(
  settings: AppRuntimeSettings,
  clientId: string,
  cases: CaseWorkspace[],
  historiesByCaseId: Record<string, ExportedMessageRepository>,
): Promise<{ alreadyImported: boolean; importedCases: number } | null> {
  const endpoint = cloudCaseImportEndpoint(settings);
  const headers = buildRequestHeaders(settings, true);
  if (!endpoint || !headers) return null;

  const response = await fetch(endpoint, {
    method: "POST",
    headers,
    body: JSON.stringify({
      clientId,
      cases,
      historiesByCaseId,
    }),
  }).catch(() => null);
  if (!response?.ok) return null;

  const payload = await response.json().catch(() => null);
  if (!payload || typeof payload !== "object") return null;
  return {
    alreadyImported: Boolean((payload as { alreadyImported?: unknown }).alreadyImported),
    importedCases:
      typeof (payload as { importedCases?: unknown }).importedCases === "number"
        ? ((payload as { importedCases?: number }).importedCases ?? 0)
        : 0,
  };
}
