import type { CaseHistoryRepository, CaseStore } from "../../domain/cases/case-store";
import type { AppLogger } from "../../lib/logger";
import {
  errorMessage,
  redactUrlForLogs,
  resolveEnvValue,
  truncateValue,
  type AppEnv,
} from "../../shared/support";

const TIDB_BINDING_KEY = "tidbAi";
const CHAT_ID_KEY = "chatId";

function asRecord(value: unknown): Record<string, unknown> | null {
  if (!value || typeof value !== "object" || Array.isArray(value)) return null;
  return value as Record<string, unknown>;
}

function sanitizeChatId(value: unknown): string | null {
  if (typeof value !== "string") return null;
  const normalized = value.trim();
  if (!normalized || normalized.length > 128) return null;
  return normalized;
}

export function extractTidbChatIdFromHistory(
  repository: CaseHistoryRepository | null | undefined,
): string | null {
  const metadata = asRecord(repository?.metadata);
  const tidbBinding = asRecord(metadata?.[TIDB_BINDING_KEY]);
  return sanitizeChatId(tidbBinding?.[CHAT_ID_KEY]);
}

export function bindTidbChatIdToHistory(
  repository: CaseHistoryRepository | null | undefined,
  chatId: string,
): CaseHistoryRepository {
  const baseRepository: CaseHistoryRepository = repository
    ? {
        ...repository,
        messages: [...repository.messages],
      }
    : {
        headId: null,
        messages: [],
      };
  const metadata = asRecord(baseRepository.metadata) ?? {};
  const tidbBinding = asRecord(metadata[TIDB_BINDING_KEY]) ?? {};

  return {
    ...baseRepository,
    metadata: {
      ...metadata,
      [TIDB_BINDING_KEY]: {
        ...tidbBinding,
        [CHAT_ID_KEY]: chatId,
      },
    },
  };
}

export async function persistTidbChatBinding(
  caseStore: CaseStore,
  principalId: string,
  caseId: string,
  chatId: string,
): Promise<void> {
  const currentRepository = await caseStore.getHistory(principalId, caseId);
  await caseStore.saveHistory(
    principalId,
    caseId,
    bindTidbChatIdToHistory(currentRepository, chatId),
  );
}

function resolveTidbChatDeleteUrl(apiUrl: string, chatId: string): string | null {
  try {
    const resolved = new URL(apiUrl);
    resolved.pathname = `${resolved.pathname.replace(/\/+$/, "")}/${encodeURIComponent(chatId)}`;
    resolved.search = "";
    return resolved.toString();
  } catch {
    return null;
  }
}

export async function deleteTidbChatUpstream({
  chatId,
  env,
  fetchImpl,
  logger,
  requestId,
}: {
  chatId: string;
  env: AppEnv;
  fetchImpl: typeof fetch;
  logger: AppLogger;
  requestId: string;
}): Promise<void> {
  const apiUrl = resolveEnvValue(env, "TIDB_API_URL");
  const apiToken = resolveEnvValue(env, "TIDB_API_TOKEN");
  if (!apiUrl || !apiToken) {
    throw new Error("Missing TIDB_API_URL or TIDB_API_TOKEN");
  }

  const deleteUrl = resolveTidbChatDeleteUrl(apiUrl, chatId);
  if (!deleteUrl) {
    throw new Error("Invalid TIDB_API_URL");
  }

  logger.info("tidb.chat_delete.request", {
    chat_id: chatId,
    request_id: requestId,
    upstream_url: redactUrlForLogs(deleteUrl),
  });

  let response: Response;
  try {
    response = await fetchImpl(deleteUrl, {
      method: "DELETE",
      headers: {
        Accept: "application/json",
        Authorization: `Bearer ${apiToken}`,
      },
    });
  } catch (error) {
    logger.error("tidb.chat_delete.fetch_failed", {
      chat_id: chatId,
      error_message: errorMessage(error),
      request_id: requestId,
      upstream_url: redactUrlForLogs(deleteUrl),
    });
    throw error;
  }

  if (!response.ok) {
    const body = await response.text().catch(() => "");
    logger.warn("tidb.chat_delete.rejected", {
      body_excerpt: truncateValue(body, 512),
      chat_id: chatId,
      request_id: requestId,
      status: response.status,
      upstream_url: redactUrlForLogs(deleteUrl),
    });
    throw new Error(
      `Upstream returned ${response.status}${body.trim() ? `: ${body.trim()}` : ""}`,
    );
  }

  logger.info("tidb.chat_delete.response", {
    chat_id: chatId,
    request_id: requestId,
    status: response.status,
    upstream_url: redactUrlForLogs(deleteUrl),
  });
}
