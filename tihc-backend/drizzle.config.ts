import "dotenv/config";
import { defineConfig } from "drizzle-kit";
import { buildMysqlDbCredentials } from "./src/infrastructure/persistence/tidb/drizzle-config";

const databaseUrl = process.env.DATABASE_URL ?? "";
const sslCaPath = process.env.DATABASE_SSL_CA_PATH?.trim();

export default defineConfig({
  out: "./drizzle",
  schema: "./src/infrastructure/persistence/tidb/schema.ts",
  dialect: "mysql",
  dbCredentials: buildMysqlDbCredentials(databaseUrl, sslCaPath),
});
