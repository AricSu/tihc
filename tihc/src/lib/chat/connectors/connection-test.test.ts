import { afterEach, describe, expect, test, vi } from "vitest";

import type { AgentInstance } from "@/lib/chat/agent-types";
import { runAgentConnectionTest } from "./connection-test";

function buildAgent(
  templateId: AgentInstance["templateId"],
  overrides: Partial<AgentInstance> = {},
): AgentInstance {
  return {
    id: "agent-1",
    name: "Agent",
    templateId,
    transport: templateId === "generic-websocket" ? "websocket" : "http",
    endpoint: "https://agent.example.com/v1/chat/completions",
    model: "test-model",
    apiKey: "secret",
    headersJson: "{}",
    extraBodyJson: "{}",
    responseMode: "delta",
    deltaPath: "delta",
    snapshotPath: "text",
    donePath: "done",
    doneSentinel: "[DONE]",
    ...overrides,
  };
}

afterEach(() => {
  vi.restoreAllMocks();
});

describe("agent connection tests", () => {
  test("returns success for reachable HTTP agents", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn(async () => new Response(JSON.stringify({ ok: true }), { status: 200 })),
    );

    const result = await runAgentConnectionTest(buildAgent("openai-compatible"));

    expect(result.ok).toBe(true);
    expect(result.message).toContain("200");
  });

  test("returns success when a websocket agent opens a connection", async () => {
    class MockWebSocket {
      static instances: MockWebSocket[] = [];
      onopen: ((event: Event) => void) | null = null;
      onerror: ((event: Event) => void) | null = null;
      onclose: ((event: Event) => void) | null = null;

      constructor(_url: string) {
        MockWebSocket.instances.push(this);
        queueMicrotask(() => {
          this.onopen?.(new Event("open"));
        });
      }

      close() {
        this.onclose?.(new Event("close"));
      }

      addEventListener(
        type: "open" | "error" | "close",
        listener: (event: Event) => void,
      ) {
        if (type === "open") this.onopen = listener;
        if (type === "error") this.onerror = listener;
        if (type === "close") this.onclose = listener;
      }
    }

    vi.stubGlobal("WebSocket", MockWebSocket);

    const result = await runAgentConnectionTest(
      buildAgent("generic-websocket", {
        endpoint: "wss://agent.example.com/socket",
      }),
    );

    expect(result.ok).toBe(true);
    expect(result.message).toContain("connected");
  });
});
