import type { LanguageModelUsage } from "ai";
import type { StoredLlmUsageEvent, UsageStore } from "../../domain/usage/usage-store";
import type { AppLogger } from "../../lib/logger";
import {
  createNormalizedUsageRecord,
  type NormalizedUsageSource,
} from "./usage-normalizer";

export type UsageRecordCallback = (input: {
  finishedAt?: string;
  success: boolean;
  usage?: LanguageModelUsage | null;
  source?: NormalizedUsageSource;
  errorCode?: string | null;
}) => Promise<void>;

export function createUsageRecordCallback({
  usageStore,
  logger,
  baseRecord,
}: {
  usageStore: UsageStore | null;
  logger: AppLogger;
  baseRecord: Omit<
    StoredLlmUsageEvent,
    | "id"
    | "finishedAt"
    | "latencyMs"
    | "success"
    | "inputTokens"
    | "outputTokens"
    | "totalTokens"
    | "cachedInputTokens"
    | "reasoningTokens"
    | "source"
    | "rawUsage"
    | "costUsd"
    | "errorCode"
  >;
}): UsageRecordCallback {
  return async ({ finishedAt, success, usage, source, errorCode }) => {
    if (!usageStore) return;

    try {
      const record = createNormalizedUsageRecord({
        ...baseRecord,
        finishedAt: finishedAt ?? new Date().toISOString(),
        success,
        usage,
        ...(source ? { source } : {}),
        ...(errorCode ? { errorCode } : {}),
      });
      await usageStore.saveUsageEvent(record);
      logger.info("usage.recorded", {
        model: record.model,
        principal_id: record.principalId ?? undefined,
        provider: record.provider,
        request_id: record.requestId,
        source: record.source,
        success: record.success,
        total_tokens: record.totalTokens ?? 0,
      });
    } catch (error) {
      logger.warn("usage.record_failed", {
        error_message: error instanceof Error ? error.message : String(error),
        request_id: baseRecord.requestId,
      });
    }
  };
}
