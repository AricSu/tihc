import { describe, expect, test } from "vitest";
import type { AppRuntimeSettings } from "@/lib/chat/agent-types";
import { resolveBackendEndpoint } from "./backend-endpoint";

describe("resolveBackendEndpoint", () => {
  test("uses the llm runtime base url instead of the tidb.ai plugin config", () => {
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
        baseUrl: "https://runtime.example.com",
        providerId: "openai",
        model: "gpt-4.1-mini",
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

    expect(resolveBackendEndpoint(settings, "/v1/llm/providers")).toBe(
      "https://runtime.example.com/v1/llm/providers",
    );
  });
});
