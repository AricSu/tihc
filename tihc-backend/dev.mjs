import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { serve } from "@hono/node-server";
import { parse } from "dotenv";

function loadEnvFile(path) {
  if (!existsSync(path)) return;

  const values = parse(readFileSync(path));
  for (const [key, value] of Object.entries(values)) {
    if (process.env[key] === undefined) {
      process.env[key] = value;
    }
  }
}

function loadLocalEnv() {
  const cwd = process.cwd();
  loadEnvFile(resolve(cwd, ".env.local"));
  loadEnvFile(resolve(cwd, ".env"));
}

loadLocalEnv();
process.env.LOG_LEVEL ??= "debug";
process.env.LOG_FORMAT ??= "pretty";

const parsedPort = Number.parseInt(process.env.PORT ?? "3010", 10);
const port = Number.isFinite(parsedPort) ? parsedPort : 3000;
const { createLogger } = await import("./src/lib/logger.ts");
const { default: app } = await import("./src/index.ts");

function redactUrl(rawUrl) {
  if (!rawUrl) return undefined;
  try {
    const parsed = new URL(rawUrl);
    return `${parsed.origin}${parsed.pathname}`;
  } catch {
    return rawUrl;
  }
}

const logger = createLogger({
  format: process.env.LOG_FORMAT,
  level: process.env.LOG_LEVEL,
});

logger.info("server.config", {
  database_url_present: Boolean(process.env.DATABASE_URL),
  ga4_debug: process.env.GA4_DEBUG === "true",
  ga4_enabled: process.env.GA4_ENABLED === "true",
  log_format: process.env.LOG_FORMAT,
  log_level: process.env.LOG_LEVEL,
  port,
  require_auth: process.env.REQUIRE_AUTH === "true",
  tidb_api_token_present: Boolean(process.env.TIDB_API_TOKEN),
  tidb_api_url: redactUrl(process.env.TIDB_API_URL),
});

serve(
  {
    fetch: app.fetch,
    port,
  },
  (info) => {
    logger.info("server.listening", {
      url: `http://localhost:${info.port}`,
    });
  },
);
