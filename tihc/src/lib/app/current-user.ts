import type { AppRuntimeSettings, CurrentUserRecord } from "@/lib/chat/agent-types";
import { resolveBackendEndpoint } from "@/lib/app/backend-endpoint";

export const ANONYMOUS_DISPLAY_NAME = "匿名";

export function deriveDisplayNameFromEmail(email: string): string {
  const trimmed = email.trim();
  if (!trimmed) return "User";

  const [localPart] = trimmed.split("@");
  const normalized = (localPart ?? "")
    .replace(/[._-]+/g, " ")
    .replace(/\s+/g, " ")
    .trim();

  if (!normalized) {
    return trimmed;
  }

  return normalized
    .split(" ")
    .map((segment) =>
      segment ? `${segment.slice(0, 1).toUpperCase()}${segment.slice(1)}` : "",
    )
    .join(" ");
}

export function buildAnonymousCurrentUser(): CurrentUserRecord {
  return {
    id: null,
    authState: "anonymous",
    displayName: ANONYMOUS_DISPLAY_NAME,
    email: "",
    hostedDomain: "",
  };
}

export function buildLocalCurrentUser(settings: AppRuntimeSettings): CurrentUserRecord {
  const email = settings.googleAuth?.email?.trim() ?? "";
  const accessToken = settings.googleAuth?.accessToken?.trim() ?? "";
  if (!accessToken && !email) {
    return buildAnonymousCurrentUser();
  }

  return {
    id: null,
    authState: "authenticated",
    displayName: deriveDisplayNameFromEmail(email),
    email,
    hostedDomain: settings.googleAuth?.hostedDomain?.trim() ?? "",
  };
}

function currentUserEndpoint(settings: AppRuntimeSettings): string | null {
  return resolveBackendEndpoint(settings, "/v1/me");
}

function buildRequestHeaders(settings: AppRuntimeSettings): Record<string, string> | undefined {
  const accessToken = settings.googleAuth?.accessToken?.trim() ?? "";
  return accessToken ? { Authorization: `Bearer ${accessToken}` } : undefined;
}

function sanitizeCurrentUserRecord(value: unknown): CurrentUserRecord | null {
  if (!value || typeof value !== "object" || Array.isArray(value)) return null;

  const record = value as Record<string, unknown>;
  const authState = record.authState === "authenticated" ? "authenticated" : "anonymous";
  const email = typeof record.email === "string" ? record.email.trim() : "";
  const displayName =
    typeof record.displayName === "string" && record.displayName.trim()
      ? record.displayName.trim()
      : authState === "authenticated"
        ? deriveDisplayNameFromEmail(email)
        : ANONYMOUS_DISPLAY_NAME;

  return {
    id: typeof record.id === "string" && record.id.trim() ? record.id.trim() : null,
    authState,
    displayName,
    email,
    hostedDomain: typeof record.hostedDomain === "string" ? record.hostedDomain.trim() : "",
  };
}

export async function getCurrentUser(
  settings: AppRuntimeSettings,
): Promise<CurrentUserRecord> {
  const endpoint = currentUserEndpoint(settings);
  const fallback = buildLocalCurrentUser(settings);
  if (!endpoint) return fallback;

  const response = await fetch(endpoint, {
    method: "GET",
    ...(buildRequestHeaders(settings) ? { headers: buildRequestHeaders(settings) } : {}),
  }).catch(() => null);
  if (!response?.ok) return fallback;

  const payload = await response.json().catch(() => null);
  const user = sanitizeCurrentUserRecord((payload as { user?: unknown } | null)?.user);
  if (!user) return fallback;
  if (user.authState === "anonymous" && fallback.authState === "authenticated") {
    return fallback;
  }
  return user;
}
