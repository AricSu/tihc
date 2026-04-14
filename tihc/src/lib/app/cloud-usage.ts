import type {
  AppRuntimeSettings,
  StoredUsageSummaryRecord,
  StoredUsageTimeseriesPoint,
} from "@/lib/chat/agent-types";
import { resolveBackendEndpoint } from "@/lib/app/backend-endpoint";

function buildRequestHeaders(settings: AppRuntimeSettings): Record<string, string> | null {
  const accessToken = settings.googleAuth?.accessToken?.trim() ?? "";
  if (!accessToken) return null;

  return {
    Authorization: `Bearer ${accessToken}`,
  };
}

function usageSummaryEndpoint(settings: AppRuntimeSettings, days: number): string | null {
  return resolveBackendEndpoint(settings, `/v1/usage/summary?days=${encodeURIComponent(String(days))}`);
}

function usageTimeseriesEndpoint(settings: AppRuntimeSettings, days: number): string | null {
  return resolveBackendEndpoint(settings, `/v1/usage/timeseries?days=${encodeURIComponent(String(days))}`);
}

export async function getStoredUsageSummary(
  settings: AppRuntimeSettings,
  days = 30,
): Promise<StoredUsageSummaryRecord | null> {
  const endpoint = usageSummaryEndpoint(settings, days);
  const headers = buildRequestHeaders(settings);
  if (!endpoint || !headers) return null;

  const response = await fetch(endpoint, {
    method: "GET",
    headers,
  }).catch(() => null);
  if (!response?.ok) return null;

  const payload = await response.json().catch(() => null);
  const summary = (payload as { summary?: unknown } | null)?.summary;
  return summary && typeof summary === "object"
    ? (summary as StoredUsageSummaryRecord)
    : null;
}

export async function getStoredUsageTimeseries(
  settings: AppRuntimeSettings,
  days = 90,
): Promise<StoredUsageTimeseriesPoint[]> {
  const endpoint = usageTimeseriesEndpoint(settings, days);
  const headers = buildRequestHeaders(settings);
  if (!endpoint || !headers) return [];

  const response = await fetch(endpoint, {
    method: "GET",
    headers,
  }).catch(() => null);
  if (!response?.ok) return [];

  const payload = await response.json().catch(() => null);
  const points = (payload as { points?: unknown } | null)?.points;
  return Array.isArray(points) ? (points as StoredUsageTimeseriesPoint[]) : [];
}
