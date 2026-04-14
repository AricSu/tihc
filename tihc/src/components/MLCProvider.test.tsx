import { beforeEach, describe, expect, test, vi } from "vitest";
import type { AppRuntimeSettings } from "@/lib/chat/agent-types";

const {
  autoTitleCaseFromPromptMock,
  getAppSettingsSnapshotMock,
  hasCompleteGlobalLlmRuntimeMock,
  runWebSearchMock,
  streamGlobalRuntimeMock,
  trackTelemetryEventMock,
} = vi.hoisted(() => ({
  autoTitleCaseFromPromptMock: vi.fn(),
  getAppSettingsSnapshotMock: vi.fn(),
  hasCompleteGlobalLlmRuntimeMock: vi.fn(),
  runWebSearchMock: vi.fn(),
  streamGlobalRuntimeMock: vi.fn(),
  trackTelemetryEventMock: vi.fn(),
}));

function buildSettings(overrides: Partial<AppRuntimeSettings> = {}): AppRuntimeSettings {
  return {
    activeCaseId: "case-123",
    analyticsConsent: "unknown",
    cases: [],
    cloudSync: {
      importedClientId: null,
      lastHydratedAt: null,
      mode: "local",
    },
    llmRuntime: {
      baseUrl: overrides.llmRuntime?.baseUrl ?? "https://runtime.example.com",
      providerId: overrides.llmRuntime?.providerId ?? "openai",
      model: overrides.llmRuntime?.model ?? "gpt-4.1-mini",
    },
    googleAuth: null,
    installedPlugins: [
      {
        pluginId: "tidb.ai",
        label: "tidb.ai",
        kind: "mcp",
        capabilities: ["mcp"],
        config: {
          baseUrl: "https://tidb.ai",
        },
      },
      {
        pluginId: "websearch",
        label: "Web Search",
        kind: "mcp",
        capabilities: ["mcp"],
        config: {
          enabled: true,
          mode: "aggressive",
          primaryEngine: "duckduckgo",
        },
      },
    ],
    ...overrides,
  };
}

vi.mock("@/lib/app/runtime", () => ({
  autoTitleCaseFromPrompt: autoTitleCaseFromPromptMock,
  getAppSettingsSnapshot: getAppSettingsSnapshotMock,
}));

vi.mock("@/lib/llm/runtime", () => ({
  hasCompleteGlobalLlmRuntime: hasCompleteGlobalLlmRuntimeMock,
  streamGlobalRuntime: streamGlobalRuntimeMock,
}));

vi.mock("@/lib/websearch/client", () => ({
  runWebSearch: runWebSearchMock,
}));

vi.mock("@/lib/telemetry", () => ({
  trackTelemetryEvent: trackTelemetryEventMock,
}));

function isAsyncIterable<T>(value: unknown): value is AsyncIterable<T> {
  return !!value && typeof (value as AsyncIterable<T>)[Symbol.asyncIterator] === "function";
}

describe("buildModelAdapter", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    const settings = buildSettings();
    getAppSettingsSnapshotMock.mockReturnValue(settings);
    hasCompleteGlobalLlmRuntimeMock.mockImplementation((runtime) =>
      Boolean(runtime.providerId?.trim() && runtime.model?.trim()),
    );
  });

  test("emits chat_failed telemetry with failure_kind instead of raw failure text", async () => {
    runWebSearchMock.mockResolvedValue(null);
    streamGlobalRuntimeMock.mockImplementation(async function* () {
      yield {
        type: "error",
        message: "Request timed out after 30 seconds",
      };
    });

    const { buildModelAdapter } = await import("./MLCProvider");
    const modelAdapter = buildModelAdapter("case-123");
    const runResult = modelAdapter.run({
      abortSignal: new AbortController().signal,
      messages: [
        {
          role: "user",
          content: [{ type: "text", text: "hello" }],
        },
      ] as never,
    } as never);

    if (!isAsyncIterable(runResult)) {
      throw new Error("expected model adapter to stream outputs");
    }

    const outputs: unknown[] = [];
    for await (const output of runResult) {
      outputs.push(output);
    }

    expect(autoTitleCaseFromPromptMock).toHaveBeenCalledWith("case-123", "hello");
    expect(trackTelemetryEventMock).toHaveBeenNthCalledWith(1, "tihc_ext_chat_submitted", {
      context: {
        case_id: "case-123",
        surface: "sidepanel",
      },
    });
    expect(trackTelemetryEventMock).toHaveBeenNthCalledWith(2, "tihc_ext_chat_failed", {
      context: {
        case_id: "case-123",
        failure_kind: "timeout",
        surface: "sidepanel",
      },
    });
    expect(trackTelemetryEventMock.mock.calls[1]?.[1]?.context?.failure_reason).toBeUndefined();
    expect(outputs).toEqual([
      {
        content: [{ type: "text", text: "Request timed out after 30 seconds" }],
      },
    ]);
  });

  test("injects web search context into the outbound user message and surfaces websearch statuses", async () => {
    runWebSearchMock.mockImplementation(async ({ onStatus }) => {
      onStatus?.("Searching web with duckduckgo");
      onStatus?.("Injected 1 sources into prompt");
      return {
        engine: "duckduckgo",
        query: "latest TiDB docs",
        results: [
          {
            title: "TiDB Docs",
            url: "https://docs.pingcap.com/tidb/stable",
            snippet: "Official TiDB documentation.",
            pageExcerpt: "Read the latest TiDB documentation here.",
          },
        ],
        searchedAt: "2026-04-14T12:00:00.000Z",
      };
    });
    streamGlobalRuntimeMock.mockImplementation(async function* () {
      yield { type: "text-delta", text: "Grounded answer." };
      yield { type: "done" };
    });

    const { buildModelAdapter } = await import("./MLCProvider");
    const modelAdapter = buildModelAdapter("case-987");
    const runResult = modelAdapter.run({
      abortSignal: new AbortController().signal,
      messages: [
        {
          role: "user",
          content: [{ type: "text", text: "latest TiDB docs" }],
        },
      ] as never,
    } as never);

    if (!isAsyncIterable(runResult)) {
      throw new Error("expected model adapter to stream outputs");
    }

    const outputs: unknown[] = [];
    for await (const output of runResult) {
      outputs.push(output);
    }

    expect(runWebSearchMock).toHaveBeenCalledWith(
      expect.objectContaining({
        query: "latest TiDB docs",
        primaryEngine: "duckduckgo",
      }),
    );
    expect(streamGlobalRuntimeMock).toHaveBeenCalledWith(
      expect.objectContaining({
        messages: [
          {
            role: "user",
            content: expect.stringContaining("[Web Search Context]"),
          },
        ],
      }),
      expect.objectContaining({
        llmRuntime: expect.objectContaining({
          providerId: "openai",
          model: "gpt-4.1-mini",
        }),
      }),
    );
    expect(outputs).toEqual([
      {
        content: [
          {
            type: "text",
            text: "Retrieval Process:\n- Searching web with duckduckgo\n\nAnswer:",
          },
        ],
      },
      {
        content: [
          {
            type: "text",
            text: "Retrieval Process:\n- Searching web with duckduckgo\n- Injected 1 sources into prompt\n\nAnswer:",
          },
        ],
      },
      {
        content: [
          {
            type: "text",
            text: "Retrieval Process:\n- Searching web with duckduckgo\n- Injected 1 sources into prompt\n\nAnswer:\nGrounded answer.",
          },
        ],
      },
    ]);
  });

  test("skips websearch when the user explicitly asks for an offline answer", async () => {
    runWebSearchMock.mockResolvedValue(null);
    streamGlobalRuntimeMock.mockImplementation(async function* () {
      yield { type: "done" };
    });

    const { buildModelAdapter } = await import("./MLCProvider");
    const modelAdapter = buildModelAdapter("case-offline");
    const runResult = modelAdapter.run({
      abortSignal: new AbortController().signal,
      messages: [
        {
          role: "user",
          content: [{ type: "text", text: "Explain TiDB architecture, offline only." }],
        },
      ] as never,
    } as never);

    if (!isAsyncIterable(runResult)) {
      throw new Error("expected model adapter to stream outputs");
    }

    for await (const _ of runResult) {
      // drain
    }

    expect(runWebSearchMock).not.toHaveBeenCalled();
    expect(streamGlobalRuntimeMock).toHaveBeenCalledWith(
      expect.objectContaining({
        messages: [
          {
            role: "user",
            content: "Explain TiDB architecture, offline only.",
          },
        ],
      }),
      expect.any(Object),
    );
  });

  test("skips websearch when the separate websearch plugin is not installed or enabled", async () => {
    getAppSettingsSnapshotMock.mockReturnValueOnce(
      buildSettings({
        installedPlugins: [
          {
            pluginId: "tidb.ai",
            label: "tidb.ai",
            kind: "mcp",
            capabilities: ["mcp"],
            config: {
              baseUrl: "https://tidb.ai",
            },
          },
        ],
      }),
    );
    streamGlobalRuntimeMock.mockImplementation(async function* () {
      yield { type: "done" };
    });

    const { buildModelAdapter } = await import("./MLCProvider");
    const modelAdapter = buildModelAdapter("case-no-websearch");
    const runResult = modelAdapter.run({
      abortSignal: new AbortController().signal,
      messages: [
        {
          role: "user",
          content: [{ type: "text", text: "latest TiDB docs" }],
        },
      ] as never,
    } as never);

    if (!isAsyncIterable(runResult)) {
      throw new Error("expected model adapter to stream outputs");
    }

    for await (const _ of runResult) {
      // drain
    }

    expect(runWebSearchMock).not.toHaveBeenCalled();
  });

  test("shows a not-ready message when the global runtime is not configured", async () => {
    getAppSettingsSnapshotMock.mockReturnValueOnce(
      buildSettings({
        llmRuntime: {
          baseUrl: "https://runtime.example.com",
          providerId: "",
          model: "",
        },
      }),
    );

    const { buildModelAdapter } = await import("./MLCProvider");
    const modelAdapter = buildModelAdapter("case-agent-missing-runtime");
    const runResult = modelAdapter.run({
      abortSignal: new AbortController().signal,
      messages: [
        {
          role: "user",
          content: [{ type: "text", text: "Investigate TiDB hotspot writes." }],
        },
      ] as never,
    } as never);

    if (!isAsyncIterable(runResult)) {
      throw new Error("expected model adapter to stream outputs");
    }

    const outputs: unknown[] = [];
    for await (const output of runResult) {
      outputs.push(output);
    }

    expect(streamGlobalRuntimeMock).not.toHaveBeenCalled();
    expect(outputs).toEqual([
      {
        content: [
          {
            type: "text",
            text: "User LLM Settings are not configured. Open Settings and choose a provider and model before chatting.",
          },
        ],
      },
    ]);
  });
});
