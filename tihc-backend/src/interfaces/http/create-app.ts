import { Hono } from "hono";
import { cors } from "hono/cors";
import type { CaseStore } from "../../domain/cases/case-store";
import type { UsageStore } from "../../domain/usage/usage-store";
import { createTiDbUsageStore } from "../../infrastructure/persistence/tidb/tidb-usage-store";
import { createTiDbCaseStore } from "../../infrastructure/persistence/tidb/tidb-case-store";
import { createDb } from "../../infrastructure/persistence/tidb/db-client";
import { createCodexBridge, type CodexBridge } from "../../lib/codex-bridge";
import { createLogger, type AppLogger } from "../../lib/logger";
import { JSON_RESPONSE_HEADERS, jsonError } from "./http";
import { createHttpContextHelpers, type AppContext } from "./http-context";
import { registerCaseRoutes } from "./routes/case-routes";
import { registerLlmRoutes } from "./routes/llm-routes";
import { registerSettingsRoutes } from "./routes/settings-routes";
import { registerTelemetryRoutes } from "./routes/telemetry-routes";
import { registerUsageRoutes } from "./routes/usage-routes";
import {
  defaultLogFormat,
  defaultLogLevel,
  errorMessage,
  resolveEnvValue,
  type AppEnv,
} from "../../shared/support";

type CreateAppOptions = {
  caseStore?: CaseStore;
  usageStore?: UsageStore;
  codexBridge?: CodexBridge;
  env?: AppEnv;
  fetchImpl?: typeof fetch;
  logger?: AppLogger;
};

function responseLogLevel(status: number): "info" | "warn" | "error" {
  if (status >= 500) return "error";
  if (status >= 400) return "warn";
  return "info";
}

function installRequestLifecycle(app: Hono<AppContext>, logger: AppLogger) {
  app.use("*", async (context, next) => {
    const requestId = crypto.randomUUID();
    const startedAt = Date.now();

    context.set("requestId", requestId);
    context.set("requestStartedAt", startedAt);

    logger.info("request.started", {
      method: context.req.method,
      path: context.req.path,
      request_id: requestId,
    });

    await next();

    context.res.headers.set("X-Request-Id", requestId);
    logger[responseLogLevel(context.res.status)]("request.completed", {
      duration_ms: Date.now() - startedAt,
      method: context.req.method,
      path: context.req.path,
      request_id: requestId,
      status: context.res.status,
    });
  });
}

function installCors(app: Hono<AppContext>) {
  app.use(
    "*",
    cors({
      allowHeaders: ["Authorization", "Content-Type", "X-Tihc-Client-Id"],
      allowMethods: ["GET", "POST", "OPTIONS"],
      origin: "*",
    }),
  );
}

export function createApp({
  caseStore: configuredCaseStore,
  usageStore: configuredUsageStore,
  codexBridge = createCodexBridge(),
  env = process.env,
  fetchImpl = fetch,
  logger = createLogger({
    format: defaultLogFormat(env),
    level: defaultLogLevel(env),
  }),
}: CreateAppOptions = {}) {
  const app = new Hono<AppContext>();
  const databaseUrl = resolveEnvValue(env, "DATABASE_URL");
  const db = databaseUrl ? createDb(databaseUrl) : null;
  const caseStore = configuredCaseStore ?? (db ? createTiDbCaseStore(db) : null);
  const usageStore = configuredUsageStore ?? (db ? createTiDbUsageStore(db) : null);
  const helpers = createHttpContextHelpers({
    caseStore,
    env,
    fetchImpl,
    logger,
  });

  installRequestLifecycle(app, logger);
  installCors(app);

  app.onError((error, context) => {
    const requestId = context.get("requestId");
    logger.error("request.unhandled_error", {
      error_message: errorMessage(error),
      method: context.req.method,
      path: context.req.path,
      request_id: requestId,
    });

    const response = jsonError(500, "Internal server error");
    response.headers.set("X-Request-Id", requestId);
    return response;
  });

  app.get("/health", (context) => {
    logger.debug("health.checked", {
      request_id: context.get("requestId"),
    });
    return context.json({ ok: true }, 200, helpers.noStoreHeaders);
  });

  registerLlmRoutes({
    app,
    caseStore,
    codexBridge,
    env,
    fetchImpl,
    helpers,
    logger,
    usageStore,
  });
  registerSettingsRoutes({
    app,
    caseStore,
    helpers,
    logger,
  });
  registerCaseRoutes({
    app,
    caseStore,
    helpers,
    logger,
  });
  registerTelemetryRoutes({
    app,
    env,
    fetchImpl,
    helpers,
    logger,
  });
  registerUsageRoutes({
    app,
    caseStore,
    usageStore,
    helpers,
    logger,
  });

  app.notFound((context) => {
    logger.warn("request.not_found", {
      method: context.req.method,
      path: context.req.path,
      request_id: context.get("requestId"),
    });
    return context.json({ error: { message: "Not found" } }, 404, JSON_RESPONSE_HEADERS);
  });

  return app;
}

const app = createApp();

export default app;
