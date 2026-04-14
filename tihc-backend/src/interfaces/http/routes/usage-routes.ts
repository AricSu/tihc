import type { CaseStore } from "../../../domain/cases/case-store";
import type { UsageStore } from "../../../domain/usage/usage-store";
import type { AppLogger } from "../../../lib/logger";
import { jsonError } from "../http";
import type { AppInstance, HttpContextHelpers } from "../http-context";

type RegisterUsageRoutesOptions = {
  app: AppInstance;
  caseStore: CaseStore | null;
  usageStore: UsageStore | null;
  helpers: HttpContextHelpers;
  logger: AppLogger;
};

function parseWindowDays(raw: string | undefined): number {
  const parsed = Number.parseInt(raw ?? "", 10);
  if (!Number.isFinite(parsed) || parsed < 1) return 30;
  return Math.min(parsed, 365);
}

export function registerUsageRoutes({
  app,
  caseStore,
  usageStore,
  helpers,
  logger,
}: RegisterUsageRoutesOptions) {
  app.get("/v1/usage/summary", async (context) => {
    const requestId = helpers.requestIdOf(context);
    if (!caseStore || !usageStore) {
      logger.error("usage.config_missing", {
        request_id: requestId,
      });
      return jsonError(500, "Missing DATABASE_URL");
    }

    const principal = await helpers.requireAppPrincipal(context);
    if (principal instanceof Response) return principal;

    const windowDays = parseWindowDays(new URL(context.req.raw.url).searchParams.get("days") ?? undefined);
    const summary = await usageStore.getUsageSummary(principal.id, windowDays);

    logger.info("usage.summary_read", {
      principal_id: principal.id,
      request_id: requestId,
      window_days: windowDays,
    });

    return context.json({ summary }, 200, helpers.noStoreHeaders);
  });

  app.get("/v1/usage/timeseries", async (context) => {
    const requestId = helpers.requestIdOf(context);
    if (!caseStore || !usageStore) {
      logger.error("usage.config_missing", {
        request_id: requestId,
      });
      return jsonError(500, "Missing DATABASE_URL");
    }

    const principal = await helpers.requireAppPrincipal(context);
    if (principal instanceof Response) return principal;

    const windowDays = parseWindowDays(new URL(context.req.raw.url).searchParams.get("days") ?? undefined);
    const points = await usageStore.getUsageTimeseries(principal.id, windowDays);

    logger.info("usage.timeseries_read", {
      point_count: points.length,
      principal_id: principal.id,
      request_id: requestId,
      window_days: windowDays,
    });

    return context.json({ points }, 200, helpers.noStoreHeaders);
  });
}
