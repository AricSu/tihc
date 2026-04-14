import {
  buildStoredCaseRecord,
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

function randomId() {
  return crypto.randomUUID();
}

function clone<T>(value: T): T {
  return JSON.parse(JSON.stringify(value)) as T;
}

export function createMemoryCaseStore(): CaseStore {
  const principalsByGoogleSub = new Map<string, PrincipalRecord>();
  const settingsByPrincipalId = new Map<string, PrincipalSettingsRecord>();
  const llmCredentialsByPrincipalId = new Map<
    string,
    Map<string, { apiKey: string; updatedAt: string }>
  >();
  const casesByPrincipalId = new Map<string, Map<string, StoredCaseRecord>>();
  const historiesByPrincipalId = new Map<string, Map<string, CaseHistoryRepository>>();
  const importReceiptsByPrincipalId = new Map<string, Set<string>>();

  const ensureCases = (principalId: string) => {
    const existing = casesByPrincipalId.get(principalId);
    if (existing) return existing;
    const next = new Map<string, StoredCaseRecord>();
    casesByPrincipalId.set(principalId, next);
    return next;
  };

  const ensureHistories = (principalId: string) => {
    const existing = historiesByPrincipalId.get(principalId);
    if (existing) return existing;
    const next = new Map<string, CaseHistoryRepository>();
    historiesByPrincipalId.set(principalId, next);
    return next;
  };

  const ensureReceipts = (principalId: string) => {
    const existing = importReceiptsByPrincipalId.get(principalId);
    if (existing) return existing;
    const next = new Set<string>();
    importReceiptsByPrincipalId.set(principalId, next);
    return next;
  };

  const ensureLlmCredentials = (principalId: string) => {
    const existing = llmCredentialsByPrincipalId.get(principalId);
    if (existing) return existing;
    const next = new Map<string, { apiKey: string; updatedAt: string }>();
    llmCredentialsByPrincipalId.set(principalId, next);
    return next;
  };

  return {
    async upsertPrincipal(identity: PrincipalIdentity): Promise<PrincipalRecord> {
      const existing = principalsByGoogleSub.get(identity.googleSub);
      const now = new Date().toISOString();
      if (existing) {
        const next = {
          ...existing,
          email: identity.email,
          hostedDomain: identity.hostedDomain,
          lastSeenAt: now,
        };
        principalsByGoogleSub.set(identity.googleSub, next);
        return clone(next);
      }

      const created: PrincipalRecord = {
        id: randomId(),
        googleSub: identity.googleSub,
        email: identity.email,
        hostedDomain: identity.hostedDomain,
        createdAt: now,
        lastSeenAt: now,
      };
      principalsByGoogleSub.set(identity.googleSub, created);
      return clone(created);
    },

    async getSettings(principalId: string): Promise<PrincipalSettingsRecord | null> {
      const settings = settingsByPrincipalId.get(principalId);
      return settings ? clone(settings) : null;
    },

    async saveSettings(
      principalId: string,
      input: SavePrincipalSettingsInput,
    ): Promise<PrincipalSettingsRecord> {
      const next: PrincipalSettingsRecord = {
        activeCaseId: input.activeCaseId,
        analyticsConsent: input.analyticsConsent,
        llmRuntime: clone(input.llmRuntime),
        installedPlugins: clone(input.installedPlugins),
        updatedAt: new Date().toISOString(),
      };
      settingsByPrincipalId.set(principalId, next);
      return clone(next);
    },

    async getLlmCredential(
      principalId: string,
      providerId: string,
    ): Promise<PrincipalLlmCredentialRecord | null> {
      const record = ensureLlmCredentials(principalId).get(providerId);
      if (!record) return null;
      return {
        providerId,
        hasSecret: true,
        updatedAt: record.updatedAt,
      };
    },

    async getLlmCredentialSecret(principalId: string, providerId: string): Promise<string | null> {
      return ensureLlmCredentials(principalId).get(providerId)?.apiKey ?? null;
    },

    async saveLlmCredential(
      principalId: string,
      input: SavePrincipalLlmCredentialInput,
    ): Promise<PrincipalLlmCredentialRecord> {
      const updatedAt = new Date().toISOString();
      ensureLlmCredentials(principalId).set(input.providerId, {
        apiKey: input.apiKey,
        updatedAt,
      });
      return {
        providerId: input.providerId,
        hasSecret: true,
        updatedAt,
      };
    },

    async listCases(principalId: string): Promise<StoredCaseRecord[]> {
      return [...ensureCases(principalId).values()]
        .sort((a, b) => Date.parse(b.updatedAt) - Date.parse(a.updatedAt))
        .map(clone);
    },

    async createCase(principalId: string, input: CreateStoredCaseInput): Promise<StoredCaseRecord> {
      const cases = ensureCases(principalId);
      const next = buildStoredCaseRecord(input);
      cases.set(input.id, next);
      return clone(next);
    },

    async updateCase(
      principalId: string,
      caseId: string,
      patch: UpdateStoredCaseInput,
    ): Promise<StoredCaseRecord | null> {
      const cases = ensureCases(principalId);
      const current = cases.get(caseId);
      if (!current) return null;

      const next = {
        ...current,
        ...patch,
      };
      cases.set(caseId, next);
      return clone(next);
    },

    async deleteCase(principalId: string, caseId: string): Promise<boolean> {
      const cases = ensureCases(principalId);
      const histories = ensureHistories(principalId);
      const deleted = cases.delete(caseId);
      histories.delete(caseId);
      return deleted;
    },

    async getHistory(principalId: string, caseId: string): Promise<CaseHistoryRepository | null> {
      const history = ensureHistories(principalId).get(caseId);
      return history ? clone(history) : null;
    },

    async saveHistory(
      principalId: string,
      caseId: string,
      repository: CaseHistoryRepository,
    ): Promise<StoredCaseRecord | null> {
      const cases = ensureCases(principalId);
      const current = cases.get(caseId);
      if (!current) return null;

      ensureHistories(principalId).set(caseId, clone(repository));
      const next = buildStoredCaseRecord(
        {
          id: current.id,
          title: current.title,
          pluginId: current.pluginId,
          activityState: current.activityState,
          resolvedAt: current.resolvedAt,
          archivedAt: current.archivedAt,
          createdAt: current.createdAt,
          updatedAt: new Date().toISOString(),
        },
        repository,
      );
      cases.set(caseId, next);
      return clone(next);
    },

    async importCases(principalId: string, input: ImportCasesInput): Promise<ImportCasesResult> {
      const receipts = ensureReceipts(principalId);
      if (receipts.has(input.clientId)) {
        return {
          alreadyImported: true,
          importedCases: 0,
        };
      }

      const cases = ensureCases(principalId);
      const histories = ensureHistories(principalId);
      let importedCases = 0;

      for (const item of input.cases) {
        const repository = input.historiesByCaseId[item.id] ?? null;
        cases.set(item.id, buildStoredCaseRecord(item, repository));
        if (repository) {
          histories.set(item.id, clone(repository));
        }
        importedCases += 1;
      }

      receipts.add(input.clientId);
      return {
        alreadyImported: false,
        importedCases,
      };
    },
  };
}
