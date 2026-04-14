import { describe, expect, test } from "vitest";

import { readMigrationFiles } from "drizzle-orm/migrator";
import { buildMysqlDbCredentials } from "./drizzle-config";

describe("drizzle mysql migration config", () => {
  test("uses explicit mysql2 credentials so ssl is preserved", () => {
    const dbCredentials = buildMysqlDbCredentials(
      "mysql://user:pa%24%24@gateway01.ap-southeast-1.prod.aws.tidbcloud.com:4000/tihc_server",
    );

    expect(dbCredentials).not.toHaveProperty("url");
    expect(dbCredentials).toMatchObject({
      database: "tihc_server",
      host: "gateway01.ap-southeast-1.prod.aws.tidbcloud.com",
      password: "pa$$",
      port: 4000,
      user: "user",
    });
    expect(dbCredentials).toHaveProperty("ssl");
  });

  test("splits mysql migrations into discrete statements", () => {
    const [migration] = readMigrationFiles({ migrationsFolder: "./drizzle" });

    expect(migration?.sql.length).toBeGreaterThan(1);
  });
});
