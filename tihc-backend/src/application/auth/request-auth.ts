import type { CaseStore, PrincipalRecord } from "../../domain/cases/case-store";
import {
  extractBearerToken,
  verifyGoogleToken,
  type GoogleTokenInfo,
} from "../../lib/google-auth";
import { deriveDisplayNameFromEmail } from "../../domain/users/current-user";
import type { AppLogger } from "../../lib/logger";
import { jsonError, unauthorizedResponse } from "../../interfaces/http/http";
import { authRequired, resolveEnvValue, type AppEnv } from "../../shared/support";

export async function verifyRequestAuth(
  request: Request,
  env: AppEnv,
  fetchImpl: typeof fetch,
  logger: AppLogger,
  requestId: string,
): Promise<Response | null> {
  if (!authRequired(env)) {
    logger.debug("auth.skipped", {
      request_id: requestId,
    });
    return null;
  }

  const token = extractBearerToken(request.headers.get("authorization"));
  if (!token) {
    logger.warn("auth.missing_bearer", {
      request_id: requestId,
    });
    return unauthorizedResponse("Missing bearer token");
  }

  try {
    await verifyGoogleToken({
      expectedAudience: resolveEnvValue(env, "GOOGLE_CLIENT_ID"),
      expectedWorkspaceDomain: resolveEnvValue(env, "GOOGLE_WORKSPACE_DOMAIN"),
      fetchImpl,
      token,
    });
    logger.info("auth.accepted", {
      request_id: requestId,
    });
    return null;
  } catch {
    logger.warn("auth.rejected", {
      request_id: requestId,
    });
    return unauthorizedResponse("Unauthorized");
  }
}

export async function verifyGoogleRequest(
  request: Request,
  env: AppEnv,
  fetchImpl: typeof fetch,
  logger: AppLogger,
  requestId: string,
): Promise<GoogleTokenInfo | Response> {
  const token = extractBearerToken(request.headers.get("authorization"));
  if (!token) {
    logger.warn("auth.missing_bearer", {
      request_id: requestId,
    });
    return unauthorizedResponse("Missing bearer token");
  }

  try {
    const tokenInfo = await verifyGoogleToken({
      expectedAudience: resolveEnvValue(env, "GOOGLE_CLIENT_ID"),
      expectedWorkspaceDomain: resolveEnvValue(env, "GOOGLE_WORKSPACE_DOMAIN"),
      fetchImpl,
      token,
    });
    logger.info("auth.accepted", {
      request_id: requestId,
    });
    return tokenInfo;
  } catch {
    logger.warn("auth.rejected", {
      request_id: requestId,
    });
    return unauthorizedResponse("Unauthorized");
  }
}

export async function requirePrincipal(
  request: Request,
  env: AppEnv,
  fetchImpl: typeof fetch,
  logger: AppLogger,
  requestId: string,
  caseStore: CaseStore | null,
): Promise<PrincipalRecord | Response> {
  if (!caseStore) {
    logger.error("cases.config_missing", {
      database_url_present: Boolean(resolveEnvValue(env, "DATABASE_URL")),
      request_id: requestId,
    });
    return jsonError(500, "Missing DATABASE_URL");
  }

  const tokenInfo = await verifyGoogleRequest(request, env, fetchImpl, logger, requestId);
  if (tokenInfo instanceof Response) return tokenInfo;

  const googleSub = tokenInfo.sub?.trim();
  const email = tokenInfo.email?.trim() ?? "";
  if (!googleSub) {
    logger.warn("auth.missing_google_sub", {
      request_id: requestId,
    });
    return unauthorizedResponse("Unauthorized");
  }

  return caseStore.upsertPrincipal({
    googleSub,
    displayName: deriveDisplayNameFromEmail(email),
    email,
    hostedDomain: tokenInfo.hd?.trim() ?? "",
  });
}

export async function resolvePrincipalIfPresent(
  request: Request,
  env: AppEnv,
  fetchImpl: typeof fetch,
  logger: AppLogger,
  requestId: string,
  caseStore: CaseStore | null,
): Promise<PrincipalRecord | Response | null> {
  const token = extractBearerToken(request.headers.get("authorization"));
  if (!token || !caseStore) {
    return null;
  }

  try {
    const tokenInfo = await verifyGoogleToken({
      expectedAudience: resolveEnvValue(env, "GOOGLE_CLIENT_ID"),
      expectedWorkspaceDomain: resolveEnvValue(env, "GOOGLE_WORKSPACE_DOMAIN"),
      fetchImpl,
      token,
    });
    const googleSub = tokenInfo.sub?.trim();
    const email = tokenInfo.email?.trim() ?? "";
    if (!googleSub) {
      logger.warn("auth.missing_google_sub", {
        request_id: requestId,
      });
      return null;
    }
    logger.info("auth.accepted", {
      request_id: requestId,
    });
    return caseStore.upsertPrincipal({
      googleSub,
      displayName: deriveDisplayNameFromEmail(email),
      email,
      hostedDomain: tokenInfo.hd?.trim() ?? "",
    });
  } catch {
    if (authRequired(env)) {
      logger.warn("auth.rejected", {
        request_id: requestId,
      });
      return unauthorizedResponse("Unauthorized");
    }
    return null;
  }
}
