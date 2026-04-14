import type { LanguageModelUsage } from "ai";
import type { StoredLlmUsageEvent } from "../../domain/usage/usage-store";

export type NormalizedUsageSource = "provider_reported" | "estimated" | "unknown";

export type NormalizedUsageMetrics = Pick<
  StoredLlmUsageEvent,
  | "inputTokens"
  | "outputTokens"
  | "totalTokens"
  | "cachedInputTokens"
  | "reasoningTokens"
  | "rawUsage"
>;

export type NormalizedUsageRecord = StoredLlmUsageEvent;

export type CreateNormalizedUsageRecordInput = {
  id?: string;
  requestId: string;
  principalId?: string | null;
  caseId?: string | null;
  sessionId?: string | null;
  provider: string;
  model: string;
  route?: "chat.completions";
  stream: boolean;
  success: boolean;
  startedAt: string;
  finishedAt: string;
  usage?: LanguageModelUsage | null;
  source?: NormalizedUsageSource;
  costUsd?: string | null;
  errorCode?: string | null;
};

function normalizeTokenCount(value: number | undefined | null): number | null {
  if (value == null || !Number.isFinite(value) || value < 0) return null;
  return Math.trunc(value);
}

function normalizeRawUsage(
  value: LanguageModelUsage["raw"] | undefined,
): Record<string, unknown> | null {
  if (!value || typeof value !== "object" || Array.isArray(value)) return null;
  return value as Record<string, unknown>;
}

function deriveTotalTokens(inputTokens: number | null, outputTokens: number | null): number | null {
  if (inputTokens == null && outputTokens == null) return null;
  return (inputTokens ?? 0) + (outputTokens ?? 0);
}

function deriveLatencyMs(startedAt: string, finishedAt: string): number {
  const started = Date.parse(startedAt);
  const finished = Date.parse(finishedAt);
  if (!Number.isFinite(started) || !Number.isFinite(finished)) return 0;
  return Math.max(0, finished - started);
}

export function normalizeLanguageModelUsage(
  usage: LanguageModelUsage | null | undefined,
): NormalizedUsageMetrics {
  if (!usage) {
    return {
      cachedInputTokens: null,
      inputTokens: null,
      outputTokens: null,
      rawUsage: null,
      reasoningTokens: null,
      totalTokens: null,
    };
  }

  const inputTokens = normalizeTokenCount(usage.inputTokens);
  const outputTokens = normalizeTokenCount(usage.outputTokens);
  const totalTokens =
    normalizeTokenCount(usage.totalTokens) ?? deriveTotalTokens(inputTokens, outputTokens);
  const cachedInputTokens = normalizeTokenCount(
    usage.inputTokenDetails?.cacheReadTokens ?? usage.cachedInputTokens,
  );
  const reasoningTokens = normalizeTokenCount(
    usage.outputTokenDetails?.reasoningTokens ?? usage.reasoningTokens,
  );

  return {
    cachedInputTokens,
    inputTokens,
    outputTokens,
    rawUsage: normalizeRawUsage(usage.raw),
    reasoningTokens,
    totalTokens,
  };
}

export function createNormalizedUsageRecord(
  input: CreateNormalizedUsageRecordInput,
): NormalizedUsageRecord {
  const metrics = normalizeLanguageModelUsage(input.usage);

  return {
    id: input.id ?? crypto.randomUUID(),
    requestId: input.requestId,
    principalId: input.principalId ?? null,
    caseId: input.caseId ?? null,
    sessionId: input.sessionId ?? null,
    provider: input.provider,
    model: input.model,
    route: input.route ?? "chat.completions",
    stream: input.stream,
    success: input.success,
    startedAt: input.startedAt,
    finishedAt: input.finishedAt,
    latencyMs: deriveLatencyMs(input.startedAt, input.finishedAt),
    source: input.source ?? (input.usage ? "provider_reported" : "unknown"),
    costUsd: input.costUsd ?? null,
    errorCode: input.errorCode ?? null,
    ...metrics,
  };
}
