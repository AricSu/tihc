import { connect } from "@tidbcloud/serverless";
import { drizzle } from "drizzle-orm/tidb-serverless";
import { schema } from "./schema";

export function createDb(databaseUrl: string) {
  const client = connect({ url: databaseUrl });
  return drizzle({ client, schema });
}

export type AppDb = ReturnType<typeof createDb>;
