import type { LanguageModelUsage } from "ai";
import { describe, expect, test, vi } from "vitest";
import { createNoopLogger, type AppLogger } from "../../lib/logger";
import { resolveProviderUsageSafely } from "./provider-proxy";

function createUsage(overrides: Partial<LanguageModelUsage> = {}): LanguageModelUsage {
  return {
    inputTokens: 12,
    inputTokenDetails: {
      noCacheTokens: 12,
      cacheReadTokens: 0,
      cacheWriteTokens: 0,
    },
    outputTokens: 4,
    outputTokenDetails: {
      textTokens: 4,
      reasoningTokens: 0,
    },
    totalTokens: 16,
    raw: {
      input_tokens: 12,
      output_tokens: 4,
      total_tokens: 16,
    },
    ...overrides,
  };
}

describe("resolveProviderUsageSafely", () => {
  test("returns provider usage when the usage promise resolves", async () => {
    const usage = createUsage();

    await expect(
      resolveProviderUsageSafely({
        usage: Promise.resolve(usage),
        logger: createNoopLogger(),
        provider: "anthropic",
        requestId: "req-1",
        upstreamUrl: "https://api.anthropic.com/v1/messages",
      }),
    ).resolves.toEqual(usage);
  });

  test("returns null and logs a warning when the usage promise rejects", async () => {
    const warn = vi.fn();
    const logger: AppLogger = {
      debug: vi.fn(),
      info: vi.fn(),
      warn,
      error: vi.fn(),
    };

    await expect(
      resolveProviderUsageSafely({
        usage: Promise.reject(new Error("usage unavailable")),
        logger,
        provider: "anthropic",
        requestId: "req-2",
        upstreamUrl: "https://api.anthropic.com/v1/messages",
      }),
    ).resolves.toBeNull();

    expect(warn).toHaveBeenCalledWith(
      "provider.usage_unavailable",
      expect.objectContaining({
        error_message: "usage unavailable",
        provider: "anthropic",
        request_id: "req-2",
      }),
    );
  });
});
