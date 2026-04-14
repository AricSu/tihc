import { existsSync, readFileSync } from "node:fs";

export const buildMysqlSslConfig = (sslCaPath?: string) =>
  ({
    ...(sslCaPath && existsSync(sslCaPath.trim())
      ? { ca: readFileSync(sslCaPath.trim(), "utf8") }
      : {}),
    minVersion: "TLSv1.2",
    rejectUnauthorized: true,
  }) as const;

export const buildMysqlDbCredentials = (
  databaseUrl: string,
  sslCaPath?: string,
) => {
  const parsedDatabaseUrl = databaseUrl ? new URL(databaseUrl) : null;
  const databaseName = parsedDatabaseUrl?.pathname.replace(/^\/+/, "") ?? "";

  return {
    database: databaseName,
    host: parsedDatabaseUrl?.hostname ?? "",
    password: parsedDatabaseUrl
      ? decodeURIComponent(parsedDatabaseUrl.password)
      : "",
    ...(parsedDatabaseUrl?.port
      ? { port: Number(parsedDatabaseUrl.port) }
      : {}),
    ssl: buildMysqlSslConfig(sslCaPath),
    user: parsedDatabaseUrl ? decodeURIComponent(parsedDatabaseUrl.username) : "",
  };
};
