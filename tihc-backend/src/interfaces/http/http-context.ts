import type { Hono } from "hono";
import { requirePrincipal, resolvePrincipalIfPresent } from "../../application/auth/request-auth";
import type { CaseStore, PrincipalRecord } from "../../domain/cases/case-store";
import type { AppLogger } from "../../lib/logger";
import { readBoundedJson } from "./http";
import type { AppEnv } from "../../shared/support";

export type AppContext = {
  Variables: {
    requestId: string;
    requestStartedAt: number;
  };
};

export type RequestContext = {
  get(name: "requestId"): string;
  req: { raw: Request };
};

export type AppInstance = Hono<AppContext>;

export type HttpContextHelpers = {
  noStoreHeaders: {
    "Cache-Control": string;
  };
  requestIdOf(context: RequestContext): string;
  readJsonBody<T>(
    context: RequestContext,
  ): Promise<{ ok: true; value: T } | { ok: false; response: Response }>;
  requireAppPrincipal(context: RequestContext): Promise<PrincipalRecord | Response>;
  resolveOptionalAppPrincipal(
    context: RequestContext,
  ): Promise<PrincipalRecord | Response | null>;
};

export function createHttpContextHelpers({
  caseStore,
  env,
  fetchImpl,
  logger,
}: {
  caseStore: CaseStore | null;
  env: AppEnv;
  fetchImpl: typeof fetch;
  logger: AppLogger;
}): HttpContextHelpers {
  return {
    noStoreHeaders: {
      "Cache-Control": "no-store",
    },
    requestIdOf(context) {
      return context.get("requestId");
    },
    readJsonBody<T>(context: RequestContext) {
      return readBoundedJson<T>(context.req.raw);
    },
    requireAppPrincipal(context: RequestContext): Promise<PrincipalRecord | Response> {
      return requirePrincipal(
        context.req.raw,
        env,
        fetchImpl,
        logger,
        context.get("requestId"),
        caseStore,
      );
    },
    resolveOptionalAppPrincipal(context: RequestContext): Promise<PrincipalRecord | Response | null> {
      return resolvePrincipalIfPresent(
        context.req.raw,
        env,
        fetchImpl,
        logger,
        context.get("requestId"),
        caseStore,
      );
    },
  };
}
