type GoogleTokenInfo = {
  aud?: string;
  email?: string;
  hd?: string;
  sub?: string;
};

type VerifyGoogleTokenOptions = {
  expectedAudience?: string;
  expectedWorkspaceDomain?: string;
  fetchImpl?: typeof fetch;
  token: string;
};

function normalizeExpectedValue(value: string | undefined): string {
  return value?.trim() ?? "";
}

export function extractBearerToken(
  headerValue: Headers | string | null | undefined,
): string | null {
  if (!headerValue) return null;
  const rawValue =
    headerValue instanceof Headers
      ? headerValue.get("authorization")
      : headerValue;
  if (!rawValue) return null;
  const trimmed = rawValue.trim();
  if (!trimmed.toLowerCase().startsWith("bearer ")) return null;
  const token = trimmed.slice(7).trim();
  return token || null;
}

async function fetchGoogleTokenInfo(
  token: string,
  fetchImpl: typeof fetch,
): Promise<GoogleTokenInfo> {
  const isJwt = token.split(".").length === 3;
  const url = new URL("https://oauth2.googleapis.com/tokeninfo");
  url.searchParams.set(isJwt ? "id_token" : "access_token", token);

  const response = await fetchImpl(url, {
    headers: {
      Accept: "application/json",
    },
  });

  if (!response.ok) {
    throw new Error(`tokeninfo rejected: http ${response.status}`);
  }

  const payload = (await response.json()) as GoogleTokenInfo;
  return payload;
}

function enforceAudience(
  tokenInfo: GoogleTokenInfo,
  expectedAudience: string,
): void {
  if (!expectedAudience) return;
  if ((tokenInfo.aud ?? "").trim() !== expectedAudience) {
    throw new Error("google token audience mismatch");
  }
}

function enforceWorkspaceDomain(
  tokenInfo: GoogleTokenInfo,
  expectedWorkspaceDomain: string,
): void {
  if (!expectedWorkspaceDomain) return;

  const normalizedExpectedDomain = expectedWorkspaceDomain.toLowerCase();
  if ((tokenInfo.hd ?? "").trim().toLowerCase() === normalizedExpectedDomain) {
    return;
  }

  const email = (tokenInfo.email ?? "").trim().toLowerCase();
  if (email.endsWith(`@${normalizedExpectedDomain}`)) {
    return;
  }

  throw new Error("google workspace domain not allowed");
}

export async function verifyGoogleToken({
  expectedAudience,
  expectedWorkspaceDomain,
  fetchImpl = fetch,
  token,
}: VerifyGoogleTokenOptions): Promise<GoogleTokenInfo> {
  const tokenInfo = await fetchGoogleTokenInfo(token, fetchImpl);

  enforceAudience(tokenInfo, normalizeExpectedValue(expectedAudience));
  enforceWorkspaceDomain(tokenInfo, normalizeExpectedValue(expectedWorkspaceDomain));

  return tokenInfo;
}

export type { GoogleTokenInfo, VerifyGoogleTokenOptions };
