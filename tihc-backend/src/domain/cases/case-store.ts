export type StoredCaseMessage = {
  role: "operator" | "tihc";
  text: string;
};

export type StoredCaseRecord = {
  id: string;
  title: string;
  pluginId: string;
  activityState: "ready" | "active" | "resolved";
  resolvedAt: string | null;
  archivedAt: string | null;
  createdAt: string;
  updatedAt: string;
  summary: string;
  signals: string[];
  messagesPreview: StoredCaseMessage[];
};

export type CaseHistoryMessage = {
  parentId: string | null;
  message: {
    id: string;
    role: string;
    content: unknown;
    createdAt?: string;
    attachments?: unknown[];
    metadata?: Record<string, unknown>;
  };
};

export type CaseHistoryRepository = {
  headId: string | null;
  messages: CaseHistoryMessage[];
};

export type PrincipalIdentity = {
  googleSub: string;
  email: string;
  hostedDomain: string;
};

export type PrincipalRecord = {
  id: string;
  googleSub: string;
  email: string;
  hostedDomain: string;
  createdAt: string;
  lastSeenAt: string;
};

export type CreateStoredCaseInput = {
  id: string;
  title: string;
  pluginId: string;
  activityState: "ready" | "active" | "resolved";
  resolvedAt: string | null;
  archivedAt: string | null;
  createdAt: string;
  updatedAt: string;
};

export type UpdateStoredCaseInput = Partial<
  Pick<
    StoredCaseRecord,
    "title" | "activityState" | "resolvedAt" | "archivedAt" | "updatedAt"
  >
>;

export type ImportCasesInput = {
  clientId: string;
  cases: CreateStoredCaseInput[];
  historiesByCaseId: Record<string, CaseHistoryRepository>;
};

export type ImportCasesResult = {
  alreadyImported: boolean;
  importedCases: number;
};

export type StoredInstalledPlugin = {
  pluginId: string;
  label: string;
  kind: string;
  capabilities: string[];
  config: Record<string, unknown>;
};

export type StoredLlmRuntimeConfig = {
  baseUrl: string;
  providerId: string;
  model: string;
};

export type PrincipalSettingsRecord = {
  activeCaseId: string | null;
  analyticsConsent: "unknown" | "granted" | "denied";
  llmRuntime: StoredLlmRuntimeConfig;
  installedPlugins: StoredInstalledPlugin[];
  updatedAt: string;
};

export type SavePrincipalSettingsInput = {
  activeCaseId: string | null;
  analyticsConsent: "unknown" | "granted" | "denied";
  llmRuntime: StoredLlmRuntimeConfig;
  installedPlugins: StoredInstalledPlugin[];
};

export type PrincipalLlmCredentialRecord = {
  providerId: string;
  hasSecret: true;
  updatedAt: string;
};

export type SavePrincipalLlmCredentialInput = {
  providerId: string;
  apiKey: string;
};

export interface CaseStore {
  upsertPrincipal(identity: PrincipalIdentity): Promise<PrincipalRecord>;
  getSettings(principalId: string): Promise<PrincipalSettingsRecord | null>;
  saveSettings(
    principalId: string,
    input: SavePrincipalSettingsInput,
  ): Promise<PrincipalSettingsRecord>;
  getLlmCredential(
    principalId: string,
    providerId: string,
  ): Promise<PrincipalLlmCredentialRecord | null>;
  getLlmCredentialSecret(principalId: string, providerId: string): Promise<string | null>;
  saveLlmCredential(
    principalId: string,
    input: SavePrincipalLlmCredentialInput,
  ): Promise<PrincipalLlmCredentialRecord>;
  listCases(principalId: string): Promise<StoredCaseRecord[]>;
  createCase(principalId: string, input: CreateStoredCaseInput): Promise<StoredCaseRecord>;
  updateCase(
    principalId: string,
    caseId: string,
    patch: UpdateStoredCaseInput,
  ): Promise<StoredCaseRecord | null>;
  deleteCase(principalId: string, caseId: string): Promise<boolean>;
  getHistory(principalId: string, caseId: string): Promise<CaseHistoryRepository | null>;
  saveHistory(
    principalId: string,
    caseId: string,
    repository: CaseHistoryRepository,
  ): Promise<StoredCaseRecord | null>;
  importCases(principalId: string, input: ImportCasesInput): Promise<ImportCasesResult>;
}
