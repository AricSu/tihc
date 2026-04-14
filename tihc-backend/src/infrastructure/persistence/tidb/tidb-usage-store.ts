import { and, eq, gte, lt } from "drizzle-orm";
import type {
  StoredLlmUsageEvent,
  UsagePeriodTotals,
  UsageStore,
  UsageSummaryRecord,
  UsageTimeseriesPoint,
} from "../../../domain/usage/usage-store";
import type { AppDb } from "./db-client";
import { llmUsageEvents } from "./schema";

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

function addEventToTotals(
  totals: UsagePeriodTotals,
  row: typeof llmUsageEvents.$inferSelect,
): void {
  totals.requestCount += 1;
  totals.inputTokens += row.inputTokens ?? 0;
  totals.outputTokens += row.outputTokens ?? 0;
  totals.totalTokens += row.totalTokens ?? 0;
  totals.cachedInputTokens += row.cachedInputTokens ?? 0;
  totals.reasoningTokens += row.reasoningTokens ?? 0;
  totals.costUsd += Number(row.costUsd ?? 0);
}

function toDayKey(isoString: string): string {
  return isoString.slice(0, 10);
}

export function createTiDbUsageStore(db: AppDb): UsageStore {
  return {
    async saveUsageEvent(event: StoredLlmUsageEvent): Promise<void> {
      await db.insert(llmUsageEvents).values({
        id: event.id,
        requestId: event.requestId,
        principalId: event.principalId,
        caseId: event.caseId,
        sessionId: event.sessionId,
        provider: event.provider,
        model: event.model,
        route: event.route,
        stream: event.stream,
        success: event.success,
        startedAt: event.startedAt,
        finishedAt: event.finishedAt,
        latencyMs: event.latencyMs,
        inputTokens: event.inputTokens,
        outputTokens: event.outputTokens,
        totalTokens: event.totalTokens,
        cachedInputTokens: event.cachedInputTokens,
        reasoningTokens: event.reasoningTokens,
        source: event.source,
        costUsd: event.costUsd,
        rawUsage: event.rawUsage,
        errorCode: event.errorCode,
      });
    },

    async getUsageSummary(
      principalId: string,
      windowDays: number,
      now = new Date(),
    ): Promise<UsageSummaryRecord> {
      const { currentStart, currentEnd, previousStart } = buildWindowBounds(windowDays, now);
      const rows = await db
        .select()
        .from(llmUsageEvents)
        .where(
          and(
            eq(llmUsageEvents.principalId, principalId),
            gte(llmUsageEvents.finishedAt, previousStart.toISOString()),
            lt(llmUsageEvents.finishedAt, currentEnd.toISOString()),
          ),
        );

      const current = createEmptyTotals();
      const previous = createEmptyTotals();

      for (const row of rows) {
        const finishedAt = Date.parse(row.finishedAt);
        if (!Number.isFinite(finishedAt)) continue;

        if (finishedAt >= currentStart.getTime()) {
          addEventToTotals(current, row);
          continue;
        }

        addEventToTotals(previous, row);
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
      const rows = await db
        .select()
        .from(llmUsageEvents)
        .where(
          and(
            eq(llmUsageEvents.principalId, principalId),
            gte(llmUsageEvents.finishedAt, currentStart.toISOString()),
            lt(llmUsageEvents.finishedAt, currentEnd.toISOString()),
          ),
        );

      const points = new Map<string, UsageTimeseriesPoint>();
      for (let cursor = new Date(currentStart); cursor < currentEnd; cursor = addUtcDays(cursor, 1)) {
        const key = cursor.toISOString().slice(0, 10);
        points.set(key, {
          date: key,
          ...createEmptyTotals(),
        });
      }

      for (const row of rows) {
        const point = points.get(toDayKey(row.finishedAt));
        if (!point) continue;
        addEventToTotals(point, row);
      }

      return [...points.values()];
    },
  };
}
