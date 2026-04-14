import type {
  StoredLlmUsageEvent,
  UsagePeriodTotals,
  UsageStore,
  UsageSummaryRecord,
  UsageTimeseriesPoint,
} from "../../../domain/usage/usage-store";

function clone<T>(value: T): T {
  return JSON.parse(JSON.stringify(value)) as T;
}

function createEmptyTotals(): UsagePeriodTotals {
  return {
    requestCount: 0,
    inputTokens: 0,
    outputTokens: 0,
    totalTokens: 0,
    cachedInputTokens: 0,
    reasoningTokens: 0,
    costUsd: 0,
  };
}

function startOfUtcDay(date: Date): Date {
  return new Date(Date.UTC(date.getUTCFullYear(), date.getUTCMonth(), date.getUTCDate()));
}

function addUtcDays(date: Date, days: number): Date {
  const next = new Date(date);
  next.setUTCDate(next.getUTCDate() + days);
  return next;
}

function dayKey(isoString: string): string {
  return isoString.slice(0, 10);
}

function addEventToTotals(totals: UsagePeriodTotals, event: StoredLlmUsageEvent): void {
  totals.requestCount += 1;
  totals.inputTokens += event.inputTokens ?? 0;
  totals.outputTokens += event.outputTokens ?? 0;
  totals.totalTokens += event.totalTokens ?? 0;
  totals.cachedInputTokens += event.cachedInputTokens ?? 0;
  totals.reasoningTokens += event.reasoningTokens ?? 0;
  totals.costUsd += Number(event.costUsd ?? 0);
}

function buildWindowBounds(windowDays: number, now: Date): {
  currentStart: Date;
  currentEnd: Date;
  previousStart: Date;
} {
  const currentEnd = addUtcDays(startOfUtcDay(now), 1);
  const currentStart = addUtcDays(currentEnd, -windowDays);
  const previousStart = addUtcDays(currentStart, -windowDays);
  return {
    currentStart,
    currentEnd,
    previousStart,
  };
}

export function createMemoryUsageStore(): UsageStore {
  const events: StoredLlmUsageEvent[] = [];

  return {
    async saveUsageEvent(event: StoredLlmUsageEvent): Promise<void> {
      events.push(clone(event));
    },

    async getUsageSummary(
      principalId: string,
      windowDays: number,
      now = new Date(),
    ): Promise<UsageSummaryRecord> {
      const { currentStart, currentEnd, previousStart } = buildWindowBounds(windowDays, now);
      const current = createEmptyTotals();
      const previous = createEmptyTotals();

      for (const event of events) {
        if (event.principalId !== principalId) continue;
        const finishedAt = Date.parse(event.finishedAt);
        if (!Number.isFinite(finishedAt)) continue;

        if (finishedAt >= currentStart.getTime() && finishedAt < currentEnd.getTime()) {
          addEventToTotals(current, event);
          continue;
        }

        if (finishedAt >= previousStart.getTime() && finishedAt < currentStart.getTime()) {
          addEventToTotals(previous, event);
        }
      }

      return {
        windowDays,
        current,
        previous,
      };
    },

    async getUsageTimeseries(
      principalId: string,
      windowDays: number,
      now = new Date(),
    ): Promise<UsageTimeseriesPoint[]> {
      const { currentStart, currentEnd } = buildWindowBounds(windowDays, now);
      const points = new Map<string, UsageTimeseriesPoint>();

      for (let cursor = new Date(currentStart); cursor < currentEnd; cursor = addUtcDays(cursor, 1)) {
        const key = cursor.toISOString().slice(0, 10);
        points.set(key, {
          date: key,
          ...createEmptyTotals(),
        });
      }

      for (const event of events) {
        if (event.principalId !== principalId) continue;
        const finishedAt = Date.parse(event.finishedAt);
        if (!Number.isFinite(finishedAt)) continue;
        if (finishedAt < currentStart.getTime() || finishedAt >= currentEnd.getTime()) continue;

        const key = dayKey(event.finishedAt);
        const point = points.get(key);
        if (!point) continue;
        addEventToTotals(point, event);
      }

      return [...points.values()];
    },
  };
}
