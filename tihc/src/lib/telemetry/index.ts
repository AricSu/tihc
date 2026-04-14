import { getAppSettingsSnapshot } from "@/lib/app/runtime";
import { resolveBackendEndpoint } from "@/lib/app/backend-endpoint";

type TelemetryEventName =
  | "tihc_ext_consent_updated"
  | "tihc_ext_surface_viewed"
  | "tihc_ext_case_created"
  | "tihc_ext_case_switched"
  | "tihc_ext_case_status_changed"
  | "tihc_ext_plugin_settings_saved"
  | "tihc_ext_plugin_connection_tested"
  | "tihc_ext_chat_submitted"
  | "tihc_ext_chat_completed"
  | "tihc_ext_chat_failed"
  | "tihc_ext_outbound_click";

type TelemetryPayload = {
  context?: Record<string, string>;
  debug?: boolean;
  params?: Record<string, string>;
};

const SESSION_TTL_MS = 30 * 60 * 1000;
let memoryClientId: string | null = null;
let memorySessionId: string | null = null;
let memorySessionSeenAt = 0;

function createId(prefix: string) {
  if (typeof globalThis.crypto?.randomUUID === "function") {
    return globalThis.crypto.randomUUID();
  }
  return `${prefix}-${Date.now()}-${Math.random().toString(16).slice(2, 10)}`;
}

async function currentClientId(): Promise<string> {
  if (memoryClientId) return memoryClientId;
  memoryClientId = createId("telemetry");
  return memoryClientId;
}

async function currentSessionId(): Promise<string> {
  const now = Date.now();
  if (memorySessionId && memorySessionSeenAt && now - memorySessionSeenAt < SESSION_TTL_MS) {
    memorySessionSeenAt = now;
    return memorySessionId;
  }

  memorySessionId = String(Math.floor(now / 1000));
  memorySessionSeenAt = now;
  return memorySessionId;
}

function extensionVersion(): string {
  const browserApi = (globalThis as typeof globalThis & {
    chrome?: {
      runtime?: {
        getManifest?: () => { version?: string };
      };
    };
    browser?: {
      runtime?: {
        getManifest?: () => { version?: string };
      };
    };
  }).browser;
  const version =
    browserApi?.runtime?.getManifest?.().version ??
    (globalThis as typeof globalThis & {
      chrome?: {
        runtime?: {
          getManifest?: () => { version?: string };
        };
      };
    }).chrome?.runtime?.getManifest?.().version;

  return typeof version === "string" && version.trim() ? version.trim() : "0.0.0";
}

function authState(): "authenticated" | "anonymous" {
  return getAppSettingsSnapshot().googleAuth?.accessToken ? "authenticated" : "anonymous";
}

export async function trackTelemetryEvent(
  event: TelemetryEventName,
  payload: TelemetryPayload = {},
): Promise<void> {
  const settings = getAppSettingsSnapshot();
  if (settings.analyticsConsent !== "granted") return;

  const endpoint = resolveBackendEndpoint(settings, "/v1/telemetry");
  if (!endpoint) return;

  const context = {
    auth_state: authState(),
    extension_version: extensionVersion(),
    plugin_id: payload.context?.plugin_id ?? "global-runtime",
    ...(settings.activeCaseId ? { case_id: settings.activeCaseId } : {}),
    ...(payload.context ?? {}),
  };

  const headers: Record<string, string> = {
    "Content-Type": "application/json",
  };
  if (settings.googleAuth?.accessToken) {
    headers.Authorization = `Bearer ${settings.googleAuth.accessToken}`;
  }

  await fetch(endpoint, {
    method: "POST",
    headers,
    body: JSON.stringify({
      event,
      params: payload.params ?? {},
      context,
      identifiers: {
        client_id: await currentClientId(),
        session_id: await currentSessionId(),
      },
      debug: payload.debug ?? false,
    }),
  }).catch(() => undefined);
}

export async function trackOutboundClick(rawUrl: string): Promise<void> {
  let parsed: URL;
  try {
    parsed = new URL(rawUrl);
  } catch {
    return;
  }

  await trackTelemetryEvent("tihc_ext_outbound_click", {
    params: {
      link_source: "chat_link",
      target_domain: parsed.hostname,
      target_path: parsed.pathname || "/",
    },
  });
}
