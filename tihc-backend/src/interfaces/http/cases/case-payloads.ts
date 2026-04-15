import type {
  CaseHistoryMessage,
  CaseHistoryRepository,
  CreateStoredCaseInput,
  ImportCasesInput,
  SavePrincipalLlmCredentialInput,
  SavePrincipalSettingsInput,
  StoredInstalledPlugin,
  UpdateStoredCaseInput,
} from "../../../domain/cases/case-store";
import { supportsUserLlmCredentialProvider } from "../../../application/chat/provider-registry";

function asString(value: unknown): string | null {
  return typeof value === "string" ? value : null;
}

function truncateValue(value: string | null, maxLength: number): string | undefined {
  if (!value) return undefined;
  const trimmed = value.trim();
  if (!trimmed) return undefined;
  return trimmed.length <= maxLength ? trimmed : trimmed.slice(0, maxLength);
}

function sanitizeStringValue(value: unknown, maxLength: number): string | undefined {
  return truncateValue(asString(value), maxLength);
}

function sanitizeNullableString(value: unknown, maxLength: number): string | null | undefined {
  if (value === null) return null;
  return sanitizeStringValue(value, maxLength);
}

function asBoolean(value: unknown): boolean | null {
  return typeof value === "boolean" ? value : null;
}

function asRecord(value: unknown): Record<string, unknown> | null {
  if (!value || typeof value !== "object" || Array.isArray(value)) return null;
  return value as Record<string, unknown>;
}

function sanitizeCaseActivityState(value: unknown): CreateStoredCaseInput["activityState"] | undefined {
  if (value === "ready" || value === "active" || value === "resolved") {
    return value;
  }
  return undefined;
}

function sanitizeAnalyticsConsent(
  value: unknown,
): SavePrincipalSettingsInput["analyticsConsent"] | undefined {
  if (value === "granted" || value === "denied" || value === "unknown") {
    return value;
  }
  return undefined;
}

function sanitizeInstalledPlugin(value: unknown): StoredInstalledPlugin | null {
  const record = asRecord(value);
  if (!record) return null;

  const pluginId = sanitizeStringValue(record.pluginId, 128);
  if (pluginId === "tidb.ai") {
    const config = asRecord(record.config);
    return {
      pluginId,
      label: "tidb.ai",
      kind: "mcp",
      capabilities: ["mcp"],
      config: {
        baseUrl: sanitizeStringValue(config?.baseUrl, 2048) ?? "",
      },
    };
  }

  if (pluginId === "websearch") {
    const config = asRecord(record.config);
    const primaryEngine =
      config?.primaryEngine === "bing" || config?.primaryEngine === "baidu"
        ? config.primaryEngine
        : "duckduckgo";
    return {
      pluginId,
      label: "Web Search",
      kind: "mcp",
      capabilities: ["mcp"],
      config: {
        enabled: asBoolean(config?.enabled) ?? true,
        mode: config?.mode === "off" ? "off" : "aggressive",
        primaryEngine,
      },
    };
  }

  return null;
}

function normalizeCreateCaseInput(
  value: unknown,
  { requireId }: { requireId: boolean },
): CreateStoredCaseInput | null {
  const record = asRecord(value);
  if (!record) return null;

  const title = sanitizeStringValue(record.title, 256);
  if (!title) return null;

  const id = sanitizeStringValue(record.id, 128) ?? (requireId ? undefined : crypto.randomUUID());
  if (!id) return null;

  return {
    id,
    title,
    pluginId: sanitizeStringValue(record.pluginId, 128) ?? "tidb.ai",
    activityState: sanitizeCaseActivityState(record.activityState) ?? "ready",
    resolvedAt: sanitizeNullableString(record.resolvedAt, 64) ?? null,
    archivedAt: sanitizeNullableString(record.archivedAt, 64) ?? null,
    createdAt: sanitizeStringValue(record.createdAt, 64) ?? new Date().toISOString(),
    updatedAt: sanitizeStringValue(record.updatedAt, 64) ?? new Date().toISOString(),
  };
}

export function sanitizeCreateCaseInput(value: unknown): CreateStoredCaseInput | null {
  return normalizeCreateCaseInput(value, { requireId: false });
}

export function sanitizeImportedCaseInput(value: unknown): CreateStoredCaseInput | null {
  return normalizeCreateCaseInput(value, { requireId: true });
}

export function sanitizeUpdateCaseInput(value: unknown): UpdateStoredCaseInput | null {
  const record = asRecord(value);
  if (!record) return null;

  const patch: UpdateStoredCaseInput = {};
  const title = sanitizeStringValue(record.title, 256);
  if (title) patch.title = title;

  const activityState = sanitizeCaseActivityState(record.activityState);
  if (activityState) patch.activityState = activityState;

  const resolvedAt = sanitizeNullableString(record.resolvedAt, 64);
  if (resolvedAt !== undefined) patch.resolvedAt = resolvedAt;

  const archivedAt = sanitizeNullableString(record.archivedAt, 64);
  if (archivedAt !== undefined) patch.archivedAt = archivedAt;

  patch.updatedAt = sanitizeStringValue(record.updatedAt, 64) ?? new Date().toISOString();
  const hasDomainPatch =
    patch.title !== undefined ||
    patch.activityState !== undefined ||
    patch.resolvedAt !== undefined ||
    patch.archivedAt !== undefined;
  return hasDomainPatch ? patch : null;
}

export function sanitizeCaseHistoryRepository(value: unknown): CaseHistoryRepository | null {
  const record = asRecord(value);
  if (!record) return null;

  const headId = sanitizeNullableString(record.headId, 128) ?? null;
  const rawMessages = Array.isArray(record.messages) ? record.messages : [];
  const messages: CaseHistoryMessage[] = [];
  for (const item of rawMessages) {
    const parsed = asRecord(item);
    if (!parsed) continue;
    const message = asRecord(parsed.message);
    const messageId = sanitizeStringValue(message?.id, 128);
    if (!message || !messageId) continue;

    messages.push({
      parentId: sanitizeNullableString(parsed.parentId, 128) ?? null,
      message: {
        id: messageId,
        role: sanitizeStringValue(message.role, 32) ?? "user",
        content: message.content ?? "",
        createdAt: sanitizeStringValue(message.createdAt, 64),
        attachments: Array.isArray(message.attachments) ? message.attachments : [],
        metadata:
          message.metadata && typeof message.metadata === "object" && !Array.isArray(message.metadata)
            ? (message.metadata as Record<string, unknown>)
            : {},
      },
    });
  }

  return {
    headId,
    messages,
    metadata:
      record.metadata && typeof record.metadata === "object" && !Array.isArray(record.metadata)
        ? (record.metadata as Record<string, unknown>)
        : undefined,
  };
}

export function sanitizeImportCasesInput(value: unknown): ImportCasesInput | null {
  const record = asRecord(value);
  if (!record) return null;

  const clientId = sanitizeStringValue(record.clientId, 128);
  if (!clientId) return null;

  const cases = Array.isArray(record.cases)
    ? record.cases
        .map(sanitizeImportedCaseInput)
        .filter((item): item is CreateStoredCaseInput => Boolean(item))
    : [];

  const historiesRecord = asRecord(record.historiesByCaseId) ?? {};
  const historiesByCaseId = Object.fromEntries(
    Object.entries(historiesRecord)
      .map(([caseId, repository]) => {
        const sanitizedCaseId = sanitizeStringValue(caseId, 128);
        const sanitizedRepository = sanitizeCaseHistoryRepository(repository);
        if (!sanitizedCaseId || !sanitizedRepository) return null;
        return [sanitizedCaseId, sanitizedRepository] as const;
      })
      .filter((entry): entry is readonly [string, CaseHistoryRepository] => Boolean(entry)),
  );

  return {
    clientId,
    cases,
    historiesByCaseId,
  };
}

export function sanitizePrincipalSettingsInput(value: unknown): SavePrincipalSettingsInput | null {
  const record = asRecord(value);
  if (!record) return null;

  const analyticsConsent = sanitizeAnalyticsConsent(record.analyticsConsent);
  if (!analyticsConsent) return null;

  const installedPlugins = Array.isArray(record.installedPlugins)
    ? record.installedPlugins
        .map(sanitizeInstalledPlugin)
        .filter((item): item is StoredInstalledPlugin => Boolean(item))
    : [];
  if (!installedPlugins.length) return null;

  const llmRuntime = asRecord(record.llmRuntime);

  return {
    activeCaseId: sanitizeNullableString(record.activeCaseId, 128) ?? null,
    analyticsConsent,
    llmRuntime: {
      baseUrl: sanitizeStringValue(llmRuntime?.baseUrl, 2048) ?? "",
      providerId: sanitizeStringValue(llmRuntime?.providerId, 128) ?? "",
      model: sanitizeStringValue(llmRuntime?.model, 128) ?? "",
    },
    installedPlugins,
  };
}

export function sanitizePrincipalLlmCredentialInput(
  value: unknown,
): SavePrincipalLlmCredentialInput | null {
  const record = asRecord(value);
  if (!record) return null;

  const providerId = sanitizeStringValue(record.providerId, 128);
  const apiKey = sanitizeStringValue(record.apiKey, 4096);
  if (!providerId || !supportsUserLlmCredentialProvider(providerId) || !apiKey) return null;

  return {
    providerId,
    apiKey,
  };
}
