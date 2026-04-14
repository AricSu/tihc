import {
  boolean,
  decimal,
  datetime,
  foreignKey,
  int,
  index,
  json,
  mysqlTable,
  primaryKey,
  unique,
  varchar,
} from "drizzle-orm/mysql-core";
import type {
  CaseHistoryRepository,
  StoredCaseMessage,
  StoredInstalledPlugin,
  StoredLlmRuntimeConfig,
} from "../../../domain/cases/case-store";

export const principals = mysqlTable(
  "principals",
  {
    id: varchar("id", { length: 36 }).notNull().primaryKey(),
    googleSub: varchar("google_sub", { length: 255 }).notNull(),
    email: varchar("email", { length: 320 }).notNull(),
    hostedDomain: varchar("hosted_domain", { length: 255 }).notNull(),
    createdAt: datetime("created_at", { mode: "string" }).notNull(),
    lastSeenAt: datetime("last_seen_at", { mode: "string" }).notNull(),
  },
  (table) => [unique("principals_google_sub_unique").on(table.googleSub)],
);

export const cases = mysqlTable(
  "cases",
  {
    principalId: varchar("principal_id", { length: 36 }).notNull(),
    id: varchar("id", { length: 128 }).notNull(),
    pluginId: varchar("plugin_id", { length: 128 }).notNull(),
    title: varchar("title", { length: 256 }).notNull(),
    activityState: varchar("activity_state", { length: 32 }).notNull(),
    resolvedAt: datetime("resolved_at", { mode: "string" }),
    archivedAt: datetime("archived_at", { mode: "string" }),
    createdAt: datetime("created_at", { mode: "string" }).notNull(),
    updatedAt: datetime("updated_at", { mode: "string" }).notNull(),
    summary: varchar("summary", { length: 4000 }).notNull(),
    signalsJson: json("signals_json").$type<string[]>().notNull(),
    messagesPreviewJson: json("messages_preview_json").$type<StoredCaseMessage[]>().notNull(),
  },
  (table) => [
    primaryKey({ columns: [table.principalId, table.id], name: "cases_pk" }),
    index("cases_principal_updated_idx").on(table.principalId, table.updatedAt),
    foreignKey({
      columns: [table.principalId],
      foreignColumns: [principals.id],
      name: "cases_principal_fk",
    }),
  ],
);

export const caseHistories = mysqlTable(
  "case_histories",
  {
    principalId: varchar("principal_id", { length: 36 }).notNull(),
    caseId: varchar("case_id", { length: 128 }).notNull(),
    repositoryJson: json("repository_json").$type<CaseHistoryRepository>().notNull(),
    updatedAt: datetime("updated_at", { mode: "string" }).notNull(),
  },
  (table) => [
    primaryKey({
      columns: [table.principalId, table.caseId],
      name: "case_histories_pk",
    }),
    foreignKey({
      columns: [table.principalId, table.caseId],
      foreignColumns: [cases.principalId, cases.id],
      name: "case_histories_case_fk",
    }),
  ],
);

export const clientImportReceipts = mysqlTable(
  "client_import_receipts",
  {
    principalId: varchar("principal_id", { length: 36 }).notNull(),
    clientId: varchar("client_id", { length: 128 }).notNull(),
    importedAt: datetime("imported_at", { mode: "string" }).notNull(),
  },
  (table) => [
    primaryKey({
      columns: [table.principalId, table.clientId],
      name: "client_import_receipts_pk",
    }),
    foreignKey({
      columns: [table.principalId],
      foreignColumns: [principals.id],
      name: "client_import_receipts_principal_fk",
    }),
  ],
);

export const principalSettings = mysqlTable(
  "principal_settings",
  {
    principalId: varchar("principal_id", { length: 36 }).notNull().primaryKey(),
    activeCaseId: varchar("active_case_id", { length: 128 }),
    analyticsConsent: varchar("analytics_consent", { length: 32 }).notNull(),
    llmRuntimeJson: json("llm_runtime_json").$type<StoredLlmRuntimeConfig>().notNull(),
    installedPluginsJson: json("installed_plugins_json").$type<StoredInstalledPlugin[]>().notNull(),
    updatedAt: datetime("updated_at", { mode: "string" }).notNull(),
  },
  (table) => [
    foreignKey({
      columns: [table.principalId],
      foreignColumns: [principals.id],
      name: "principal_settings_principal_fk",
    }),
  ],
);

export const principalLlmCredentials = mysqlTable(
  "principal_llm_credentials",
  {
    principalId: varchar("principal_id", { length: 36 }).notNull(),
    providerId: varchar("provider_id", { length: 128 }).notNull(),
    apiKey: varchar("api_key", { length: 4096 }).notNull(),
    updatedAt: datetime("updated_at", { mode: "string" }).notNull(),
  },
  (table) => [
    primaryKey({
      columns: [table.principalId, table.providerId],
      name: "principal_llm_credentials_pk",
    }),
    index("principal_llm_credentials_principal_idx").on(table.principalId, table.providerId),
    foreignKey({
      columns: [table.principalId],
      foreignColumns: [principals.id],
      name: "principal_llm_credentials_principal_fk",
    }),
  ],
);

export const llmUsageEvents = mysqlTable(
  "llm_usage_events",
  {
    id: varchar("id", { length: 36 }).notNull().primaryKey(),
    requestId: varchar("request_id", { length: 128 }).notNull(),
    principalId: varchar("principal_id", { length: 36 }),
    caseId: varchar("case_id", { length: 128 }),
    sessionId: varchar("session_id", { length: 128 }),
    provider: varchar("provider", { length: 128 }).notNull(),
    model: varchar("model", { length: 255 }).notNull(),
    route: varchar("route", { length: 64 }).notNull(),
    stream: boolean("stream").notNull(),
    success: boolean("success").notNull(),
    startedAt: datetime("started_at", { mode: "string" }).notNull(),
    finishedAt: datetime("finished_at", { mode: "string" }).notNull(),
    latencyMs: int("latency_ms").notNull(),
    inputTokens: int("input_tokens"),
    outputTokens: int("output_tokens"),
    totalTokens: int("total_tokens"),
    cachedInputTokens: int("cached_input_tokens"),
    reasoningTokens: int("reasoning_tokens"),
    source: varchar("source", { length: 32 }).notNull(),
    costUsd: decimal("cost_usd", { precision: 12, scale: 6 }),
    rawUsage: json("raw_usage").$type<Record<string, unknown> | null>(),
    errorCode: varchar("error_code", { length: 128 }),
  },
  (table) => [
    index("llm_usage_events_request_idx").on(table.requestId),
    index("llm_usage_events_principal_finished_idx").on(table.principalId, table.finishedAt),
  ],
);

export const schema = {
  caseHistories,
  cases,
  clientImportReceipts,
  llmUsageEvents,
  principalLlmCredentials,
  principalSettings,
  principals,
};
