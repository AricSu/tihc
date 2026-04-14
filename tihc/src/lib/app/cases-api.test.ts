import type { ExportedMessageRepository } from "@assistant-ui/react";
import { beforeEach, describe, expect, test, vi } from "vitest";

function installMockStorage() {
  const storage = new Map<string, string>();
  Object.defineProperty(globalThis, "localStorage", {
    configurable: true,
    value: {
      clear() {
        storage.clear();
      },
      getItem(key: string) {
        return storage.get(key) ?? null;
      },
      key(index: number) {
        return [...storage.keys()][index] ?? null;
      },
      removeItem(key: string) {
        storage.delete(key);
      },
      setItem(key: string, value: string) {
        storage.set(key, value);
      },
      get length() {
        return storage.size;
      },
    } satisfies Storage,
  });
}

function buildRepoItem(
  id: string,
  role: "user" | "assistant",
  text: string,
  parentId: string | null = null,
) {
  return {
    parentId,
    message: {
      id,
      role,
      content: [{ type: "text", text }],
      createdAt: new Date("2026-04-14T12:00:00.000Z"),
      attachments: [],
      metadata: {},
    },
  } as unknown as ExportedMessageRepository["messages"][number];
}

async function loadModules() {
  vi.resetModules();
  const runtime = await import("./runtime");
  const threadHistory = await import("./thread-history");
  const casesApi = await import("./cases-api");
  return { casesApi, runtime, threadHistory };
}

describe("cases api", () => {
  beforeEach(() => {
    installMockStorage();
    localStorage.clear();
    vi.stubGlobal("fetch", vi.fn());
  });

  test("builds dashboard cases from anonymous local runtime state when cloud sync is inactive", async () => {
    const { casesApi, runtime, threadHistory } = await loadModules();
    const fetchMock = vi.mocked(fetch);

    runtime.setAppSettings({
      activeCaseId: "case-2",
      analyticsConsent: "unknown",
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
      ],
      cases: [
        {
          id: "case-1",
          title: "Archived case",
          pluginId: "tidb.ai",
          activityState: "resolved",
          resolvedAt: "2026-04-14T10:00:00.000Z",
          archivedAt: "2026-04-14T11:00:00.000Z",
          createdAt: "2026-04-14T09:00:00.000Z",
          updatedAt: "2026-04-14T11:00:00.000Z",
        },
        {
          id: "case-2",
          title: "Checkout timeout",
          pluginId: "tidb.ai",
          activityState: "active",
          resolvedAt: null,
          archivedAt: null,
          createdAt: "2026-04-14T09:30:00.000Z",
          updatedAt: "2026-04-14T12:00:00.000Z",
        },
      ],
    });

    const history = threadHistory.createCaseHistoryAdapter("case-2");
    await history.append(buildRepoItem("m-1", "user", "Customer checkout fails after card entry."));
    await history.append(
      buildRepoItem(
        "m-2",
        "assistant",
        "The thread points to auth refresh latency after token exchange.",
        "m-1",
      ),
    );

    const cases = await casesApi.listDashboardCases();

    expect(fetchMock).not.toHaveBeenCalled();
    expect(cases).toHaveLength(1);
    expect(cases[0]).toMatchObject({
      id: "case-2",
      title: "Checkout timeout",
      status: "Investigating",
      channel: "tidb.ai",
      executionTarget: "tidb.ai",
      summary: "The thread points to auth refresh latency after token exchange.",
      messages: [
        {
          role: "operator",
          text: "Customer checkout fails after card entry.",
        },
        {
          role: "tihc",
          text: "The thread points to auth refresh latency after token exchange.",
        },
      ],
    });
    expect(cases[0]?.signals).toContain(
      "The thread points to auth refresh latency after token exchange.",
    );
  });

  test("fetches cloud-backed stored cases and maps them into dashboard records when cloud sync is active", async () => {
    const casesApi = await import("./cases-api");
    const cases = [
      casesApi.buildDashboardCaseRecordFromStoredCase(
        {
          activeCaseId: "case-cloud-1",
          analyticsConsent: "unknown",
          cases: [],
          cloudSync: {
            importedClientId: "client-123",
            lastHydratedAt: "2026-04-14T12:05:00.000Z",
            mode: "cloud",
          },
          googleAuth: {
            accessToken: "google-token",
            clientId: "google-client-id",
            email: "alice@example.com",
            hostedDomain: "example.com",
            expiresAt: "2026-04-14T18:00:00.000Z",
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
                baseUrl: "https://tidb.ai",
              },
            },
          ],
        },
        {
          id: "case-cloud-1",
          title: "Cloud checkout timeout",
          pluginId: "tidb.ai",
          activityState: "active",
          resolvedAt: null,
          archivedAt: null,
          createdAt: "2026-04-14T09:30:00.000Z",
          updatedAt: "2026-04-14T12:00:00.000Z",
          summary: "Cloud summary from TiDB.",
          signals: ["Cloud summary from TiDB."],
          messagesPreview: [
            {
              role: "operator",
              text: "The cloud case started from another device.",
            },
            {
              role: "tihc",
              text: "Cloud summary from TiDB.",
            },
          ],
        },
      ),
    ];

    expect(cases).toMatchObject([
      {
        id: "case-cloud-1",
        title: "Cloud checkout timeout",
        status: "Investigating",
        priority: "Hot",
        channel: "tidb.ai",
        executionTarget: "tidb.ai",
        summary: "Cloud summary from TiDB.",
        messages: [
          {
            role: "operator",
            text: "The cloud case started from another device.",
          },
          {
            role: "tihc",
            text: "Cloud summary from TiDB.",
          },
        ],
      },
    ]);
  });
});
