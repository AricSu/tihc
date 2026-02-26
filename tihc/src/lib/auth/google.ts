import { clearGoogleAuth, getAppSettings, setAppSettings } from "@/lib/app/runtime";

type LaunchWebAuthFlowParams = { url: string; interactive: boolean };

function getChromeIdentity(): {
  getRedirectURL: (path?: string) => string;
  launchWebAuthFlow: (
    params: LaunchWebAuthFlowParams,
    callback: (responseUrl?: string) => void,
  ) => void;
} | null {
  const chromeAny = (globalThis as unknown as { chrome?: { identity?: unknown } }).chrome;
  const identity = chromeAny?.identity as unknown;
  if (!identity) return null;
  return identity as {
    getRedirectURL: (path?: string) => string;
    launchWebAuthFlow: (
      params: LaunchWebAuthFlowParams,
      callback: (responseUrl?: string) => void,
    ) => void;
  };
}

function parseFragmentParams(url: string): Record<string, string> {
  const hashIndex = url.indexOf("#");
  if (hashIndex < 0) return {};
  const fragment = url.slice(hashIndex + 1);
  const params = new URLSearchParams(fragment);
  const out: Record<string, string> = {};
  for (const [k, v] of params.entries()) out[k] = v;
  return out;
}

export async function signInWithGoogle(): Promise<string> {
  const identity = getChromeIdentity();
  if (!identity) {
    throw new Error("chrome.identity is not available (need Chrome + identity permission).");
  }

  const { googleClientId } = getAppSettings();
  if (!googleClientId.trim()) {
    throw new Error("Missing Google Client ID (set it in the UI first).");
  }

  const redirectUri = identity.getRedirectURL("oauth2");
  const state = `${Date.now()}-${Math.random().toString(16).slice(2)}`;
  const nonce = state;
  const authUrl = new URL("https://accounts.google.com/o/oauth2/v2/auth");
  authUrl.searchParams.set("client_id", googleClientId.trim());
  authUrl.searchParams.set("redirect_uri", redirectUri);
  authUrl.searchParams.set("response_type", "id_token token");
  authUrl.searchParams.set("scope", "openid email profile");
  authUrl.searchParams.set("include_granted_scopes", "true");
  authUrl.searchParams.set("prompt", "consent");
  authUrl.searchParams.set("state", state);
  authUrl.searchParams.set("nonce", nonce);

  const responseUrl = await new Promise<string>((resolve, reject) => {
    identity.launchWebAuthFlow({ url: authUrl.toString(), interactive: true }, (url) => {
      if (!url) reject(new Error("No response URL from launchWebAuthFlow"));
      else resolve(url);
    });
  });

  const fragment = parseFragmentParams(responseUrl);
  const idToken = fragment["id_token"];
  const accessToken = fragment["access_token"];
  const token = idToken || accessToken;
  if (!token) {
    const err = fragment["error"] || "unknown_error";
    const desc = fragment["error_description"] || "";
    throw new Error(`Google OAuth failed: ${err} ${desc}`.trim());
  }

  setAppSettings({ googleToken: token });
  return token;
}

export function signOutGoogle() {
  clearGoogleAuth();
}
