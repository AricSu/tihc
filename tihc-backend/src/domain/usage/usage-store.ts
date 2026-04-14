export type StoredLlmUsageEvent = {
  id: string;
  requestId: string;
  principalId: string | null;
  caseId: string | null;
  sessionId: string | null;
  provider: string;
  model: string;
  route: "chat.completions";
  stream: boolean;
  success: boolean;
  startedAt: string;
  finishedAt: string;
  latencyMs: number;
  inputTokens: number | null;
  outputTokens: number | null;
  totalTokens: number | null;
  cachedInputTokens: number | null;
  reasoningTokens: number | null;
  source: "provider_reported" | "estimated" | "unknown";
  costUsd: string | null;
  rawUsage: Record<string, unknown> | null;
  errorCode: string | null;
};

export type UsagePeriodTotals = {
  requestCount: number;
  inputTokens: number;
  outputTokens: number;
  totalTokens: number;
  cachedInputTokens: number;
  reasoningTokens: number;
  costUsd: number;
};

export type UsageSummaryRecord = {
  windowDays: number;
  current: UsagePeriodTotals;
  previous: UsagePeriodTotals;
};

export type UsageTimeseriesPoint = UsagePeriodTotals & {
  date: string;
};

export interface UsageStore {
  saveUsageEvent(event: StoredLlmUsageEvent): Promise<void>;
  getUsageSummary(
    principalId: string,
    windowDays: number,
    now?: Date,
  ): Promise<UsageSummaryRecord>;
  getUsageTimeseries(
    principalId: string,
    windowDays: number,
    now?: Date,
  ): Promise<UsageTimeseriesPoint[]>;
}
