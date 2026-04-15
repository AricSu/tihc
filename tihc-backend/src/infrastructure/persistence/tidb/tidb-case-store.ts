import { and, desc, eq } from "drizzle-orm";
import {
  buildStoredCaseRecord,
  EMPTY_CASE_SUMMARY,
} from "../../../domain/cases/case-presentation";
import type {
  CaseHistoryRepository,
  CaseStore,
  CreateStoredCaseInput,
  ImportCasesInput,
  ImportCasesResult,
  PrincipalLlmCredentialRecord,
  PrincipalIdentity,
  PrincipalRecord,
  PrincipalSettingsRecord,
  SavePrincipalLlmCredentialInput,
  SavePrincipalSettingsInput,
  StoredCaseRecord,
  UpdateStoredCaseInput,
} from "../../../domain/cases/case-store";
import type { AppDb } from "./db-client";
import {
  caseHistories,
  cases,
  clientImportReceipts,
  principalLlmCredentials,
  principalSettings,
  principals,
} from "./schema";

function toStoredCaseRecord(
  row: typeof cases.$inferSelect,
): StoredCaseRecord {
  return {
    id: row.id,
    title: row.title,
    pluginId: row.pluginId,
    activityState: row.activityState as StoredCaseRecord["activityState"],
    resolvedAt: row.resolvedAt,
    archivedAt: row.archivedAt,
    createdAt: row.createdAt,
    updatedAt: row.updatedAt,
    summary: row.summary,
    signals: Array.isArray(row.signalsJson) ? row.signalsJson : [EMPTY_CASE_SUMMARY],
    messagesPreview: Array.isArray(row.messagesPreviewJson)
      ? row.messagesPreviewJson
      : [{ role: "tihc", text: EMPTY_CASE_SUMMARY }],
  };
}

function toPrincipalSettingsRecord(
  row: typeof principalSettings.$inferSelect,
): PrincipalSettingsRecord {
  return {
    activeCaseId: row.activeCaseId,
    analyticsConsent:
      row.analyticsConsent === "granted" || row.analyticsConsent === "denied"
        ? row.analyticsConsent
        : "unknown",
    llmRuntime: row.llmRuntimeJson ?? {
      baseUrl: "",
      providerId: "",
      model: "",
    },
    installedPlugins: Array.isArray(row.installedPluginsJson) ? row.installedPluginsJson : [],
    updatedAt: row.updatedAt,
  };
}

export function createTiDbCaseStore(db: AppDb): CaseStore {
  return {
    async upsertPrincipal(identity: PrincipalIdentity): Promise<PrincipalRecord> {
      const now = new Date().toISOString();
      await db
        .insert(principals)
        .values({
          id: crypto.randomUUID(),
          googleSub: identity.googleSub,
          displayName: identity.displayName,
          email: identity.email,
          hostedDomain: identity.hostedDomain,
          createdAt: now,
          lastSeenAt: now,
        })
        .onDuplicateKeyUpdate({
          set: {
            displayName: identity.displayName,
            email: identity.email,
            hostedDomain: identity.hostedDomain,
            lastSeenAt: now,
          },
        });

      const [principal] = await db
        .select()
        .from(principals)
        .where(eq(principals.googleSub, identity.googleSub))
        .limit(1);
      if (!principal) {
        throw new Error("Principal upsert failed.");
      }

      return {
        id: principal.id,
        googleSub: principal.googleSub,
        displayName: principal.displayName,
        email: principal.email,
        hostedDomain: principal.hostedDomain,
        createdAt: principal.createdAt,
        lastSeenAt: principal.lastSeenAt,
      };
    },

    async getSettings(principalId: string): Promise<PrincipalSettingsRecord | null> {
      const [row] = await db
        .select()
        .from(principalSettings)
        .where(eq(principalSettings.principalId, principalId))
        .limit(1);
      return row ? toPrincipalSettingsRecord(row) : null;
    },

    async saveSettings(
      principalId: string,
      input: SavePrincipalSettingsInput,
    ): Promise<PrincipalSettingsRecord> {
      const updatedAt = new Date().toISOString();
      await db
        .insert(principalSettings)
        .values({
          principalId,
          activeCaseId: input.activeCaseId,
          analyticsConsent: input.analyticsConsent,
          llmRuntimeJson: input.llmRuntime,
          installedPluginsJson: input.installedPlugins,
          updatedAt,
        })
        .onDuplicateKeyUpdate({
          set: {
            activeCaseId: input.activeCaseId,
            analyticsConsent: input.analyticsConsent,
            llmRuntimeJson: input.llmRuntime,
            installedPluginsJson: input.installedPlugins,
            updatedAt,
          },
        });

      return {
        activeCaseId: input.activeCaseId,
        analyticsConsent: input.analyticsConsent,
        llmRuntime: input.llmRuntime,
        installedPlugins: input.installedPlugins,
        updatedAt,
      };
    },

    async getLlmCredential(
      principalId: string,
      providerId: string,
    ): Promise<PrincipalLlmCredentialRecord | null> {
      const [row] = await db
        .select()
        .from(principalLlmCredentials)
        .where(
          and(
            eq(principalLlmCredentials.principalId, principalId),
            eq(principalLlmCredentials.providerId, providerId),
          ),
        )
        .limit(1);

      if (!row) return null;
      return {
        providerId: row.providerId,
        hasSecret: true,
        updatedAt: row.updatedAt,
      };
    },

    async getLlmCredentialSecret(principalId: string, providerId: string): Promise<string | null> {
      const [row] = await db
        .select()
        .from(principalLlmCredentials)
        .where(
          and(
            eq(principalLlmCredentials.principalId, principalId),
            eq(principalLlmCredentials.providerId, providerId),
          ),
        )
        .limit(1);

      return row?.apiKey ?? null;
    },

    async saveLlmCredential(
      principalId: string,
      input: SavePrincipalLlmCredentialInput,
    ): Promise<PrincipalLlmCredentialRecord> {
      const updatedAt = new Date().toISOString();
      await db
        .insert(principalLlmCredentials)
        .values({
          principalId,
          providerId: input.providerId,
          apiKey: input.apiKey,
          updatedAt,
        })
        .onDuplicateKeyUpdate({
          set: {
            apiKey: input.apiKey,
            updatedAt,
          },
        });

      return {
        providerId: input.providerId,
        hasSecret: true,
        updatedAt,
      };
    },

    async listCases(principalId: string): Promise<StoredCaseRecord[]> {
      const rows = await db
        .select()
        .from(cases)
        .where(eq(cases.principalId, principalId))
        .orderBy(desc(cases.updatedAt));
      return rows.map(toStoredCaseRecord);
    },

    async createCase(principalId: string, input: CreateStoredCaseInput): Promise<StoredCaseRecord> {
      const stored = buildStoredCaseRecord(input);
      await db.insert(cases).values({
        principalId,
        id: stored.id,
        pluginId: stored.pluginId,
        title: stored.title,
        activityState: stored.activityState,
        resolvedAt: stored.resolvedAt,
        archivedAt: stored.archivedAt,
        createdAt: stored.createdAt,
        updatedAt: stored.updatedAt,
        summary: stored.summary,
        signalsJson: stored.signals,
        messagesPreviewJson: stored.messagesPreview,
      });
      return stored;
    },

    async updateCase(
      principalId: string,
      caseId: string,
      patch: UpdateStoredCaseInput,
    ): Promise<StoredCaseRecord | null> {
      const result = await db
        .update(cases)
        .set({
          ...(patch.title !== undefined ? { title: patch.title } : {}),
          ...(patch.activityState !== undefined ? { activityState: patch.activityState } : {}),
          ...(patch.resolvedAt !== undefined ? { resolvedAt: patch.resolvedAt } : {}),
          ...(patch.archivedAt !== undefined ? { archivedAt: patch.archivedAt } : {}),
          ...(patch.updatedAt !== undefined ? { updatedAt: patch.updatedAt } : {}),
        })
        .where(and(eq(cases.principalId, principalId), eq(cases.id, caseId)));

      if (!result.rowsAffected) return null;

      const [row] = await db
        .select()
        .from(cases)
        .where(and(eq(cases.principalId, principalId), eq(cases.id, caseId)))
        .limit(1);
      return row ? toStoredCaseRecord(row) : null;
    },

    async deleteCase(principalId: string, caseId: string): Promise<boolean> {
      await db
        .delete(caseHistories)
        .where(and(eq(caseHistories.principalId, principalId), eq(caseHistories.caseId, caseId)));
      const result = await db
        .delete(cases)
        .where(and(eq(cases.principalId, principalId), eq(cases.id, caseId)));
      return Boolean(result.rowsAffected);
    },

    async getHistory(principalId: string, caseId: string): Promise<CaseHistoryRepository | null> {
      const [row] = await db
        .select()
        .from(caseHistories)
        .where(and(eq(caseHistories.principalId, principalId), eq(caseHistories.caseId, caseId)))
        .limit(1);
      return row?.repositoryJson ?? null;
    },

    async saveHistory(
      principalId: string,
      caseId: string,
      repository: CaseHistoryRepository,
    ): Promise<StoredCaseRecord | null> {
      const [existingCase] = await db
        .select()
        .from(cases)
        .where(and(eq(cases.principalId, principalId), eq(cases.id, caseId)))
        .limit(1);
      if (!existingCase) return null;

      const nextUpdatedAt = new Date().toISOString();
      const nextCase = buildStoredCaseRecord(
        {
          id: existingCase.id,
          title: existingCase.title,
          pluginId: existingCase.pluginId,
          activityState: existingCase.activityState as StoredCaseRecord["activityState"],
          resolvedAt: existingCase.resolvedAt,
          archivedAt: existingCase.archivedAt,
          createdAt: existingCase.createdAt,
          updatedAt: nextUpdatedAt,
        },
        repository,
      );

      await db
        .insert(caseHistories)
        .values({
          principalId,
          caseId,
          repositoryJson: repository,
          updatedAt: nextUpdatedAt,
        })
        .onDuplicateKeyUpdate({
          set: {
            repositoryJson: repository,
            updatedAt: nextUpdatedAt,
          },
        });

      await db
        .update(cases)
        .set({
          summary: nextCase.summary,
          signalsJson: nextCase.signals,
          messagesPreviewJson: nextCase.messagesPreview,
          updatedAt: nextUpdatedAt,
        })
        .where(and(eq(cases.principalId, principalId), eq(cases.id, caseId)));

      return nextCase;
    },

    async importCases(principalId: string, input: ImportCasesInput): Promise<ImportCasesResult> {
      const [existingReceipt] = await db
        .select()
        .from(clientImportReceipts)
        .where(
          and(
            eq(clientImportReceipts.principalId, principalId),
            eq(clientImportReceipts.clientId, input.clientId),
          ),
        )
        .limit(1);
      if (existingReceipt) {
        return {
          alreadyImported: true,
          importedCases: 0,
        };
      }

      let importedCases = 0;
      for (const item of input.cases) {
        const repository = input.historiesByCaseId[item.id] ?? null;
        const stored = buildStoredCaseRecord(item, repository);

        await db
          .insert(cases)
          .values({
            principalId,
            id: stored.id,
            pluginId: stored.pluginId,
            title: stored.title,
            activityState: stored.activityState,
            resolvedAt: stored.resolvedAt,
            archivedAt: stored.archivedAt,
            createdAt: stored.createdAt,
            updatedAt: stored.updatedAt,
            summary: stored.summary,
            signalsJson: stored.signals,
            messagesPreviewJson: stored.messagesPreview,
          })
          .onDuplicateKeyUpdate({
            set: {
              pluginId: stored.pluginId,
              title: stored.title,
              activityState: stored.activityState,
              resolvedAt: stored.resolvedAt,
              archivedAt: stored.archivedAt,
              createdAt: stored.createdAt,
              updatedAt: stored.updatedAt,
              summary: stored.summary,
              signalsJson: stored.signals,
              messagesPreviewJson: stored.messagesPreview,
            },
          });

        if (repository) {
          await db
            .insert(caseHistories)
            .values({
              principalId,
              caseId: stored.id,
              repositoryJson: repository,
              updatedAt: stored.updatedAt,
            })
            .onDuplicateKeyUpdate({
              set: {
                repositoryJson: repository,
                updatedAt: stored.updatedAt,
              },
            });
        }
        importedCases += 1;
      }

      await db.insert(clientImportReceipts).values({
        principalId,
        clientId: input.clientId,
        importedAt: new Date().toISOString(),
      });

      return {
        alreadyImported: false,
        importedCases,
      };
    },
  };
}
