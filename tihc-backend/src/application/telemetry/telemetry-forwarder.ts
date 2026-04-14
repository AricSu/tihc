import { createHash } from "node:crypto";
import { extractBearerToken, verifyGoogleToken } from "../../lib/google-auth";
import type { AppLogger } from "../../lib/logger";
import { jsonError } from "../../interfaces/http/http";
import {
  asString,
  ga4DebugEnabled,
  ga4Enabled,
  resolveEnvValue,
  truncateValue,
  type AppEnv,
} from "../../shared/support";

export type TelemetryRequest = {
  context?: Record<string, unknown>;
  debug?: boolean;
  event?: string;
  identifiers?: {
    client_id?: string;
    session_id?: string;
  };
  params?: Record<string, unknown>;
};

const TELEMETRY_EVENT_ALLOWLIST = new Set([
  "tihc_ext_consent_updated",
  "tihc_ext_surface_viewed",
  "tihc_ext_case_created",
  "tihc_ext_case_switched",
  "tihc_ext_case_status_changed",
  "tihc_ext_plugin_settings_saved",
  "tihc_ext_plugin_connection_tested",
  "tihc_ext_chat_submitted",
  "tihc_ext_chat_completed",
  "tihc_ext_chat_failed",
  "tihc_ext_outbound_click",
]);
const SURFACE_ALLOWLIST = new Set(["sidepanel", "options"]);
const AUTH_STATE_ALLOWLIST = new Set(["anonymous", "authenticated"]);
const CONSENT_STATUS_ALLOWLIST = new Set(["granted"]);
const CASE_STATUS_ALLOWLIST = new Set(["ready", "resolved", "archived", "deleted"]);
const CONNECTION_STATUS_ALLOWLIST = new Set(["success", "error"]);
const FAILURE_KIND_ALLOWLIST = new Set(["auth", "network", "upstream", "timeout", "unknown"]);

function sanitizeStringValue(value: unknown, maxLength: number): string | undefined {
  return truncateValue(asString(value), maxLength);
}

function sanitizeEnumValue(
  value: unknown,
  allowlist: Set<string>,
  maxLength = 32,
): string | undefined {
  const normalized = sanitizeStringValue(value, maxLength);
  if (!normalized || !allowlist.has(normalized)) return undefined;
  return normalized;
}

function sanitizeSessionId(value: unknown): number | undefined {
  const normalized = sanitizeStringValue(value, 32);
  if (!normalized || !/^\d+$/.test(normalized)) return undefined;

  const sessionId = Number.parseInt(normalized, 10);
  if (!Number.isSafeInteger(sessionId)) return undefined;
  return sessionId;
}

function buildCommonTelemetryParams(payload: TelemetryRequest): Record<string, unknown> {
  const merged = {
    ...(payload.params ?? {}),
    ...(payload.context ?? {}),
  };
  const params: Record<string, unknown> = {};

  const surface = sanitizeEnumValue(merged.surface, SURFACE_ALLOWLIST);
  if (surface) params.surface = surface;

  const extensionVersion = sanitizeStringValue(merged.extension_version, 32);
  if (extensionVersion) params.extension_version = extensionVersion;

  const pluginId = sanitizeStringValue(merged.plugin_id, 128);
  if (pluginId) params.plugin_id = pluginId;

  const caseId = sanitizeStringValue(merged.case_id, 128);
  if (caseId) params.case_id = caseId;

  const authState = sanitizeEnumValue(merged.auth_state, AUTH_STATE_ALLOWLIST);
  if (authState) params.auth_state = authState;

  return params;
}

function buildEventSpecificTelemetryParams(
  eventName: string,
  payload: TelemetryRequest,
): Record<string, unknown> {
  const merged = {
    ...(payload.params ?? {}),
    ...(payload.context ?? {}),
  };

  if (eventName === "tihc_ext_consent_updated") {
    const status = sanitizeEnumValue(merged.status, CONSENT_STATUS_ALLOWLIST);
    return status ? { status } : {};
  }

  if (eventName === "tihc_ext_case_status_changed") {
    const status = sanitizeEnumValue(merged.status, CASE_STATUS_ALLOWLIST);
    return status ? { status } : {};
  }

  if (eventName === "tihc_ext_plugin_connection_tested") {
    const status = sanitizeEnumValue(merged.status, CONNECTION_STATUS_ALLOWLIST);
    return status ? { status } : {};
  }

  if (eventName === "tihc_ext_chat_failed") {
    const failureKind = sanitizeEnumValue(merged.failure_kind, FAILURE_KIND_ALLOWLIST);
    return failureKind ? { failure_kind: failureKind } : {};
  }

  if (eventName === "tihc_ext_outbound_click") {
    const params: Record<string, unknown> = {};
    const linkSource = sanitizeStringValue((payload.params ?? {}).link_source, 32);
    if (linkSource) params.link_source = linkSource;

    const targetDomain = sanitizeStringValue((payload.params ?? {}).target_domain, 128);
    if (targetDomain) params.target_domain = targetDomain;

    const targetPath = sanitizeStringValue((payload.params ?? {}).target_path, 256);
    if (targetPath) params.target_path = targetPath;

    return params;
  }

  return {};
}

function buildTelemetryParams(eventName: string, payload: TelemetryRequest): Record<string, unknown> {
  const sessionId = sanitizeSessionId(payload.identifiers?.session_id);

  return {
    ...buildCommonTelemetryParams(payload),
    ...buildEventSpecificTelemetryParams(eventName, payload),
    ...(sessionId !== undefined ? { session_id: sessionId } : {}),
  };
}

function hashTelemetryUserId(subject: string, salt: string): string {
  return createHash("sha256")
    .update(`${salt}:${subject}`)
    .digest("hex");
}

async function resolveTelemetryUserId(
  request: Request,
  env: AppEnv,
  fetchImpl: typeof fetch,
): Promise<string | undefined> {
  const accessToken = extractBearerToken(request.headers.get("authorization"));
  if (!accessToken) return undefined;

  try {
    const tokenInfo = await verifyGoogleToken({
      expectedAudience: resolveEnvValue(env, "GOOGLE_CLIENT_ID"),
      expectedWorkspaceDomain: resolveEnvValue(env, "GOOGLE_WORKSPACE_DOMAIN"),
      fetchImpl,
      token: accessToken,
    });

    const salt = resolveEnvValue(env, "GA4_USER_ID_SALT");
    const subject = tokenInfo.sub?.trim() || tokenInfo.email?.trim() || "";
    if (!salt || !subject) return undefined;
    return hashTelemetryUserId(subject, salt);
  } catch {
    return undefined;
  }
}

export async function forwardTelemetry(
  request: Request,
  payload: TelemetryRequest,
  env: AppEnv,
  fetchImpl: typeof fetch,
  logger: AppLogger,
  requestId: string,
): Promise<Response> {
  const measurementId = resolveEnvValue(env, "GA4_MEASUREMENT_ID");
  const apiSecret = resolveEnvValue(env, "GA4_API_SECRET");

  if (!ga4Enabled(env) || !measurementId || !apiSecret) {
    logger.debug("telemetry.skipped", {
      ga4_api_secret_present: Boolean(apiSecret),
      ga4_enabled: ga4Enabled(env),
      ga4_measurement_id_present: Boolean(measurementId),
      request_id: requestId,
    });
    return new Response(null, {
      headers: {
        "Cache-Control": "no-store",
      },
      status: 204,
    });
  }

  const eventName = payload.event?.trim() ?? "";
  if (!TELEMETRY_EVENT_ALLOWLIST.has(eventName)) {
    logger.warn("telemetry.invalid_event", {
      event: eventName || "unknown",
      request_id: requestId,
    });
    return jsonError(400, `Invalid telemetry event: ${eventName || "unknown"}`);
  }

  const userId = await resolveTelemetryUserId(request, env, fetchImpl);
  const debugMode = ga4DebugEnabled(env) || payload.debug === true;
  const ga4Payload = {
    client_id: sanitizeStringValue(payload.identifiers?.client_id, 128) ?? "anonymous-client",
    events: [
      {
        name: eventName,
        params: buildTelemetryParams(eventName, payload),
      },
    ],
    ...(userId ? { user_id: userId } : {}),
  };

  const ga4Url = new URL(
    debugMode
      ? "https://www.google-analytics.com/debug/mp/collect"
      : "https://www.google-analytics.com/mp/collect",
  );
  ga4Url.searchParams.set("measurement_id", measurementId);
  ga4Url.searchParams.set("api_secret", apiSecret);

  const ga4Response = await fetchImpl(ga4Url.toString(), {
    body: JSON.stringify(ga4Payload),
    headers: {
      "Content-Type": "application/json",
    },
    method: "POST",
  });

  if (debugMode) {
    const debugPayload = await ga4Response.json().catch(() => ({
      validationMessages: [
        {
          description: "Invalid GA4 debug response",
        },
      ],
    }));

    logger.info("telemetry.debug_forwarded", {
      event: eventName,
      ga4_status: ga4Response.status,
      request_id: requestId,
      user_id_attached: Boolean(userId),
      validation_message_count: Array.isArray(
        (debugPayload as { validationMessages?: unknown[] }).validationMessages,
      )
        ? ((debugPayload as { validationMessages?: unknown[] }).validationMessages?.length ?? 0)
        : 0,
    });

    return Response.json(debugPayload, {
      headers: {
        "Cache-Control": "no-store",
      },
      status: ga4Response.ok ? 200 : 502,
    });
  }

  logger.info("telemetry.forwarded", {
    debug: false,
    event: eventName,
    ga4_status: ga4Response.status,
    request_id: requestId,
    user_id_attached: Boolean(userId),
  });
  return new Response(null, {
    headers: {
      "Cache-Control": "no-store",
    },
    status: 204,
  });
}
