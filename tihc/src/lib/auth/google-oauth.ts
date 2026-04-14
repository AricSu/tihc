import type { GoogleAuthState } from "@/lib/chat/agent-types";

type GoogleOAuthConfig = {
  chromeClientId?: string;
  defaultClientId?: string;
  firefoxClientId?: string;
  scopes?: string[];
};

type BrowserIdentityApi = {
  getRedirectURL(path?: string): string;
  launchWebAuthFlow(details: { interactive: boolean; url: string }): Promise<string | undefined>;
};

type GoogleTokenResponse = {
  access_token?: string;
  expires_in?: number;
};

type GoogleTokenInfoResponse = {
  aud?: string;
  email?: string;
  hd?: string;
};

type GoogleOAuthOptions = {
  browserApi?: BrowserIdentityApi;
  config?: GoogleOAuthConfig;
  fetchImpl?: typeof fetch;
  interactive?: boolean;
  now?: () => number;
  stateFactory?: () => string;
  userAgent?: string;
};

const DEFAULT_SCOPES = ["openid", "email", "profile"];

function toBase64Url(input: Uint8Array): string {
  if (typeof btoa !== "function") {
    throw new Error("Base64 encoding is unavailable in this environment.");
  }
  return btoa(String.fromCharCode(...input))
    .replace(/\+/g, "-")
    .replace(/\//g, "_")
    .replace(/=+$/g, "");
}

function resolveEnvConfig(): GoogleOAuthConfig {
  const env = import.meta.env as Record<string, string | undefined>;
  return {
    chromeClientId: env.WXT_GOOGLE_OAUTH_CHROME_CLIENT_ID ?? env.WXT_GOOGLE_OAUTH_CLIENT_ID,
    defaultClientId: env.WXT_GOOGLE_OAUTH_CLIENT_ID,
    firefoxClientId: env.WXT_GOOGLE_OAUTH_FIREFOX_CLIENT_ID ?? env.WXT_GOOGLE_OAUTH_CLIENT_ID,
    scopes: DEFAULT_SCOPES,
  };
}

function resolveUserAgent(userAgent?: string): string {
  if (userAgent) return userAgent;
  if (typeof navigator !== "undefined") return navigator.userAgent;
  return "";
}

function isFirefox(userAgent: string): boolean {
  return /firefox/i.test(userAgent);
}

export function resolveGoogleOAuthClientId(
  config: GoogleOAuthConfig = resolveEnvConfig(),
  userAgent = resolveUserAgent(),
): string {
  if (isFirefox(userAgent)) {
    return config.firefoxClientId?.trim() || config.defaultClientId?.trim() || "";
  }
  return config.chromeClientId?.trim() || config.defaultClientId?.trim() || "";
}

export function isGoogleOAuthConfigured(
  config: GoogleOAuthConfig = resolveEnvConfig(),
  userAgent = resolveUserAgent(),
): boolean {
  return Boolean(resolveGoogleOAuthClientId(config, userAgent));
}

function resolveRuntimeBrowserApi(): BrowserIdentityApi {
  const root = globalThis as typeof globalThis & {
    browser?: {
      identity?: {
        getRedirectURL?: (path?: string) => string;
        launchWebAuthFlow?: (
          details: { interactive: boolean; url: string },
        ) => Promise<string | undefined> | void;
      };
    };
    chrome?: {
      identity?: {
        getRedirectURL?: (path?: string) => string;
        launchWebAuthFlow?: (
          details: { interactive: boolean; url: string },
          callback?: (redirectUrl?: string) => void,
        ) => Promise<string | undefined> | void;
      };
      runtime?: {
        lastError?: { message?: string };
      };
    };
  };

  const identity = root.browser?.identity ?? root.chrome?.identity;
  if (!identity?.getRedirectURL || !identity.launchWebAuthFlow) {
    throw new Error("Browser identity API is unavailable.");
  }

  return {
    getRedirectURL(path?: string) {
      return identity.getRedirectURL?.(path) ?? "";
    },
    async launchWebAuthFlow(details) {
      const maybePromise = identity.launchWebAuthFlow?.(details);
      if (maybePromise && typeof (maybePromise as Promise<string | undefined>).then === "function") {
        return maybePromise as Promise<string | undefined>;
      }

      return new Promise<string | undefined>((resolve, reject) => {
        try {
          root.chrome?.identity?.launchWebAuthFlow?.(details, (redirectUrl) => {
            const runtimeError = root.chrome?.runtime?.lastError;
            if (runtimeError?.message) {
              reject(new Error(runtimeError.message));
              return;
            }
            resolve(redirectUrl);
          });
        } catch (error) {
          reject(error);
        }
      });
    },
  };
}

function randomState(): string {
  if (typeof globalThis.crypto?.randomUUID === "function") {
    return globalThis.crypto.randomUUID();
  }
  return Math.random().toString(36).slice(2);
}

async function createPkcePair(): Promise<{ challenge: string; verifier: string }> {
  const verifierBytes = globalThis.crypto.getRandomValues(new Uint8Array(32));
  const verifier = toBase64Url(verifierBytes);
  const digest = await globalThis.crypto.subtle.digest(
    "SHA-256",
    new TextEncoder().encode(verifier),
  );
  const challenge = toBase64Url(new Uint8Array(digest));
  return { challenge, verifier };
}

function computeExpiresAt(now: () => number, expiresInSeconds: number | undefined): string | null {
  if (!expiresInSeconds || !Number.isFinite(expiresInSeconds)) return null;
  return new Date(now() + expiresInSeconds * 1000).toISOString();
}

function buildRedirectUri(browserApi: BrowserIdentityApi, userAgent: string): string {
  const generatedRedirectUrl = browserApi.getRedirectURL("google");
  if (!generatedRedirectUrl) {
    throw new Error("Failed to build the extension redirect URL.");
  }
  if (!isFirefox(userAgent)) {
    return generatedRedirectUrl;
  }

  const redirectUrl = new URL(generatedRedirectUrl);
  return `http://127.0.0.1/mozoauth2/${redirectUrl.hostname}`;
}

async function fetchGoogleTokenInfo(
  accessToken: string,
  fetchImpl: typeof fetch,
): Promise<GoogleTokenInfoResponse> {
  const url = new URL("https://oauth2.googleapis.com/tokeninfo");
  url.searchParams.set("access_token", accessToken);

  const response = await fetchImpl(url, {
    headers: {
      Accept: "application/json",
    },
  });

  if (!response.ok) {
    throw new Error(`Google tokeninfo request failed with HTTP ${response.status}.`);
  }

  return (await response.json()) as GoogleTokenInfoResponse;
}

async function exchangeCodeForToken(
  clientId: string,
  code: string,
  codeVerifier: string,
  redirectUri: string,
  fetchImpl: typeof fetch,
): Promise<GoogleTokenResponse> {
  const body = new URLSearchParams({
    client_id: clientId,
    code,
    code_verifier: codeVerifier,
    grant_type: "authorization_code",
    redirect_uri: redirectUri,
  });

  const response = await fetchImpl("https://oauth2.googleapis.com/token", {
    body,
    headers: {
      "Content-Type": "application/x-www-form-urlencoded",
    },
    method: "POST",
  });

  if (!response.ok) {
    const message = await response.text().catch(() => "");
    throw new Error(`Google token exchange failed${message ? `: ${message}` : "."}`);
  }

  return (await response.json()) as GoogleTokenResponse;
}

async function runGoogleOAuthFlow({
  browserApi = resolveRuntimeBrowserApi(),
  config = resolveEnvConfig(),
  fetchImpl = fetch,
  interactive = true,
  now = () => Date.now(),
  stateFactory = randomState,
  userAgent = resolveUserAgent(),
}: GoogleOAuthOptions): Promise<GoogleAuthState> {
  const clientId = resolveGoogleOAuthClientId(config, userAgent);
  if (!clientId) {
    throw new Error(
      "Missing Google OAuth client ID. Set WXT_GOOGLE_OAUTH_CLIENT_ID or a browser-specific override.",
    );
  }

  const state = stateFactory();
  const redirectUri = buildRedirectUri(browserApi, userAgent);
  const pkce = await createPkcePair();
  const authUrl = new URL("https://accounts.google.com/o/oauth2/v2/auth");

  authUrl.searchParams.set("client_id", clientId);
  authUrl.searchParams.set("code_challenge", pkce.challenge);
  authUrl.searchParams.set("code_challenge_method", "S256");
  authUrl.searchParams.set("include_granted_scopes", "true");
  authUrl.searchParams.set("redirect_uri", redirectUri);
  authUrl.searchParams.set("response_type", "code");
  authUrl.searchParams.set("scope", (config.scopes ?? DEFAULT_SCOPES).join(" "));
  authUrl.searchParams.set("state", state);
  authUrl.searchParams.set("prompt", interactive ? "consent" : "none");

  const redirectResult = await browserApi.launchWebAuthFlow({
    interactive,
    url: authUrl.toString(),
  });
  if (!redirectResult) {
    throw new Error("Google OAuth did not return a redirect URL.");
  }

  const responseUrl = new URL(redirectResult);
  const returnedState = responseUrl.searchParams.get("state");
  if (returnedState !== state) {
    throw new Error("Google OAuth state mismatch.");
  }

  const code = responseUrl.searchParams.get("code");
  if (!code) {
    const error = responseUrl.searchParams.get("error");
    if (error) {
      throw new Error(`Google OAuth failed: ${error}`);
    }
    throw new Error("Google OAuth did not return an authorization code.");
  }

  const tokenResponse = await exchangeCodeForToken(
    clientId,
    code,
    pkce.verifier,
    redirectUri,
    fetchImpl,
  );
  const accessToken = tokenResponse.access_token?.trim();
  if (!accessToken) {
    throw new Error("Google token exchange did not return an access token.");
  }

  const tokenInfo = await fetchGoogleTokenInfo(accessToken, fetchImpl);

  return {
    accessToken,
    clientId,
    email: tokenInfo.email?.trim() ?? "",
    hostedDomain: tokenInfo.hd?.trim() ?? "",
    expiresAt: computeExpiresAt(now, tokenResponse.expires_in),
  };
}

export async function signInWithGoogle(options: GoogleOAuthOptions = {}): Promise<GoogleAuthState> {
  return runGoogleOAuthFlow({
    ...options,
    interactive: options.interactive ?? true,
  });
}

export async function refreshGoogleAuthSession(
  options: Omit<GoogleOAuthOptions, "interactive"> = {},
): Promise<GoogleAuthState> {
  try {
    return await runGoogleOAuthFlow({
      ...options,
      interactive: false,
    });
  } catch {
    return runGoogleOAuthFlow({
      ...options,
      interactive: true,
    });
  }
}

export async function revokeGoogleAuthToken(
  accessToken: string,
  fetchImpl: typeof fetch = fetch,
): Promise<void> {
  if (!accessToken.trim()) return;
  const url = `https://oauth2.googleapis.com/revoke?token=${encodeURIComponent(accessToken)}`;
  const response = await fetchImpl(url, {
    method: "POST",
  });
  if (!response.ok) {
    throw new Error(`Google token revoke failed with HTTP ${response.status}.`);
  }
}

export async function signOutFromGoogle(
  accessToken: string,
  fetchImpl: typeof fetch = fetch,
): Promise<void> {
  await revokeGoogleAuthToken(accessToken, fetchImpl);
}

export type { BrowserIdentityApi, GoogleOAuthConfig, GoogleOAuthOptions };
