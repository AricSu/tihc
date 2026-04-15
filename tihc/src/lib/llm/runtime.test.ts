import { describe, expect, test } from "vitest";
import type { AppRuntimeSettings } from "@/lib/chat/agent-types";
import { buildGlobalRuntimeAgent, hasCompleteGlobalLlmRuntime } from "./runtime";

describe("global llm runtime helpers", () => {
  test("treats an empty provider selection as the anonymous tidb.ai default route", () => {
    expect(
      hasCompleteGlobalLlmRuntime({
        baseUrl: "",
        providerId: "",
        model: "",
      }),
    ).toBe(true);
  });

  test("uses the tidb.ai plugin endpoint when anonymous runtime settings are empty", () => {
    const settings: AppRuntimeSettings = {
      activeCaseId: null,
      analyticsConsent: "unknown",
      cases: [],
      cloudSync: {
        importedClientId: null,
        lastHydratedAt: null,
        mode: "local",
      },
      llmRuntime: {
        baseUrl: "",
        providerId: "",
        model: "",
      },
      installedPlugins: [
        {
          pluginId: "tidb.ai",
          label: "tidb.ai",
          kind: "mcp",
          capabilities: ["mcp"],
          config: {
            baseUrl: "https://plugin.example.com",
          },
        },
      ],
      googleAuth: null,
    };

    expect(buildGlobalRuntimeAgent(settings)).toMatchObject({
      endpoint: "https://plugin.example.com/v1/chat/completions",
      model: "",
    });
  });

  test("includes case_id in the backend request body when a case-scoped chat is running", () => {
    const settings: AppRuntimeSettings = {
      activeCaseId: "case-123",
      analyticsConsent: "unknown",
      cases: [],
      cloudSync: {
        importedClientId: null,
        lastHydratedAt: null,
        mode: "local",
      },
      llmRuntime: {
        baseUrl: "https://runtime.example.com",
        providerId: "openai",
        model: "gpt-4.1-mini",
      },
      installedPlugins: [],
      googleAuth: null,
    };

    expect(JSON.parse(buildGlobalRuntimeAgent(settings, "case-123").extraBodyJson)).toMatchObject({
      case_id: "case-123",
      provider: "openai",
    });
  });
});
