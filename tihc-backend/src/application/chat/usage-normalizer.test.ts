import type { LanguageModelUsage } from "ai";
import { describe, expect, test } from "vitest";
import {
  createNormalizedUsageRecord,
  normalizeLanguageModelUsage,
} from "./usage-normalizer";

function createUsage(overrides: Partial<LanguageModelUsage> = {}): LanguageModelUsage {
  return {
    inputTokens: 120,
    inputTokenDetails: {
      noCacheTokens: 100,
      cacheReadTokens: 20,
      cacheWriteTokens: 0,
    },
    outputTokens: 40,
    outputTokenDetails: {
      textTokens: 35,
      reasoningTokens: 5,
    },
    totalTokens: 170,
    raw: {
      input_tokens: 120,
      output_tokens: 40,
      total_tokens: 170,
    },
    ...overrides,
  };
}

describe("usage normalizer", () => {
  test("maps ai sdk usage into normalized request-level fields", () => {
    const usage = createUsage();

    expect(normalizeLanguageModelUsage(usage)).toEqual({
      cachedInputTokens: 20,
      inputTokens: 120,
      outputTokens: 40,
      rawUsage: {
        input_tokens: 120,
        output_tokens: 40,
        total_tokens: 170,
      },
      reasoningTokens: 5,
      totalTokens: 170,
    });

    const record = createNormalizedUsageRecord({
      caseId: "case-1",
      finishedAt: "2026-04-15T10:00:02.500Z",
      model: "claude-3-5-sonnet-latest",
      principalId: "principal-1",
      provider: "anthropic",
      requestId: "req-1",
      sessionId: "sess-1",
      startedAt: "2026-04-15T10:00:00.000Z",
      stream: true,
      success: true,
      usage,
    });

    expect(record).toMatchObject({
      cachedInputTokens: 20,
      caseId: "case-1",
      inputTokens: 120,
      latencyMs: 2500,
      model: "claude-3-5-sonnet-latest",
      outputTokens: 40,
      principalId: "principal-1",
      provider: "anthropic",
      rawUsage: {
        input_tokens: 120,
        output_tokens: 40,
        total_tokens: 170,
      },
      reasoningTokens: 5,
      requestId: "req-1",
      route: "chat.completions",
      sessionId: "sess-1",
      source: "provider_reported",
      stream: true,
      success: true,
      totalTokens: 170,
    });
    expect(record.id).toEqual(expect.any(String));
  });

  test("derives total tokens when the provider omits them", () => {
    const usage = createUsage({
      totalTokens: undefined,
    });

    expect(normalizeLanguageModelUsage(usage)).toMatchObject({
      inputTokens: 120,
      outputTokens: 40,
      totalTokens: 160,
    });
  });

  test("returns null token fields and unknown source when usage is missing", () => {
    const record = createNormalizedUsageRecord({
      finishedAt: "2026-04-15T10:00:01.000Z",
      model: "tidb",
      provider: "tidb",
      requestId: "req-2",
      startedAt: "2026-04-15T10:00:00.000Z",
      stream: false,
      success: false,
      usage: null,
    });

    expect(record).toMatchObject({
      cachedInputTokens: null,
      inputTokens: null,
      outputTokens: null,
      rawUsage: null,
      reasoningTokens: null,
      source: "unknown",
      totalTokens: null,
    });
  });
});
