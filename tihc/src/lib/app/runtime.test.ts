import { beforeEach, describe, expect, test, vi } from "vitest";
import { ANONYMOUS_LOCAL_STORAGE_LIMIT_BYTES } from "@/lib/app/anonymous-local-case-limit";

const LOCAL_RUNTIME_STATE_KEY = "tihc_local_runtime_state_v2";

const {
  isGoogleOAuthConfiguredMock,
  refreshGoogleAuthSessionMock,
} = vi.hoisted(() => ({
  isGoogleOAuthConfiguredMock: vi.fn(() => false),
  refreshGoogleAuthSessionMock: vi.fn(),
}));

vi.mock("@/lib/auth/google-oauth", () => ({
  isGoogleOAuthConfigured: isGoogleOAuthConfiguredMock,
  refreshGoogleAuthSession: refreshGoogleAuthSessionMock,
}));

function installMockStorage(initialEntries: Array<[string, string]> = []) {
  const storage = new Map<string, string>(initialEntries);
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

function listStoredValues(): string[] {
  const values: string[] = [];
  for (let index = 0; index < localStorage.length; index += 1) {
    const key = localStorage.key(index);
    if (!key) continue;
    const value = localStorage.getItem(key);
    if (value) values.push(value);
  }
  return values;
}

async function loadRuntimeModule() {
  vi.resetModules();
  return import("./runtime");
}

describe("app runtime settings", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.unstubAllEnvs();
    installMockStorage();
    localStorage.clear();
    vi.stubGlobal("fetch", vi.fn());
  });

  test("falls back to the default local backend when no runtime base url is configured", async () => {
    vi.stubEnv("MODE", "development");
    vi.stubEnv("VITE_BACKEND_BASE_URL", "");

    const runtime = await loadRuntimeModule();

    expect(runtime.getAppSettings()).toMatchObject({
      llmRuntime: {
        baseUrl: "http://localhost:3010",
        providerId: "",
        model: "",
      },
      installedPlugins: [
        {
          pluginId: "tidb.ai",
          config: {
            baseUrl: "http://localhost:3010",
          },
        },
      ],
    });
  });

  test("starts from anonymous local defaults and clears legacy local storage keys", async () => {
    installMockStorage([
      ["tihc_app_settings_v1", "{\"activeCaseId\":\"legacy\"}"],
      ["tihc_local_mode_state_v1", "{\"activeCaseId\":\"legacy-local\"}"],
      ["tihc_app_client_id_v1", "legacy-client-id"],
      ["tihc_telemetry_client_id_v1", "legacy-telemetry-client-id"],
      ["tihc_telemetry_session_id_v1", "legacy-session-id"],
      ["tihc_telemetry_session_seen_at_v1", "1713110400"],
      ["tihc_case_history_v3:case-1", "{\"headId\":\"m-1\",\"messages\":[]}"],
      ["tihc_thread_history_v2:case-1", "{\"headId\":\"m-1\",\"messages\":[]}"],
    ]);

    const runtime = await loadRuntimeModule();
    const settings = runtime.getAppSettings();

    expect(settings.installedPlugins.map((plugin) => plugin.pluginId)).toEqual([
      "tidb.ai",
    ]);
    expect(settings.llmRuntime).toEqual({
      baseUrl: "",
      providerId: "",
      model: "",
    });
    expect(settings.analyticsConsent).toBe("unknown");
    expect(settings.cases).toEqual([]);
    expect(settings.googleAuth).toBeNull();
    expect(localStorage.getItem("tihc_app_settings_v1")).toBeNull();
    expect(localStorage.getItem("tihc_app_client_id_v1")).toBeNull();
    expect(localStorage.getItem("tihc_case_history_v3:case-1")).toBeNull();
    expect(localStorage.getItem("tihc_thread_history_v2:case-1")).toBeNull();
  });

  test("creates cases on the only installed tidb.ai plugin", async () => {
    const runtime = await loadRuntimeModule();

    const created = runtime.createCase?.("Ticket 417");

    expect(created).toMatchObject({
      title: "Ticket 417",
      pluginId: "tidb.ai",
      activityState: "ready",
      resolvedAt: null,
      archivedAt: null,
    });

    const settings = runtime.getAppSettings();
    expect(settings.activeCaseId).toBe(created?.id);
  });

  test("blocks creating anonymous local cases when browser storage usage is already above the limit", async () => {
    const oversizedText = "x".repeat(Math.ceil(ANONYMOUS_LOCAL_STORAGE_LIMIT_BYTES / 2));
    installMockStorage([
      [
        LOCAL_RUNTIME_STATE_KEY,
        JSON.stringify({
          activeCaseId: "case-1",
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
          googleAuth: null,
          cases: [
            {
              id: "case-1",
              title: "Oversized history case",
              pluginId: "tidb.ai",
              activityState: "active",
              resolvedAt: null,
              archivedAt: null,
              createdAt: "2026-04-14T12:00:00.000Z",
              updatedAt: "2026-04-14T12:00:00.000Z",
            },
          ],
        }),
      ],
      [
        "tihc_local_history_v2:case-1",
        JSON.stringify({
          headId: "m-1",
          messages: [
            {
              parentId: null,
              message: {
                id: "m-1",
                role: "user",
                content: [{ type: "text", text: oversizedText }],
                createdAt: "2026-04-14T12:00:00.000Z",
                attachments: [],
                metadata: {},
              },
            },
          ],
        }),
      ],
    ]);

    const runtime = await loadRuntimeModule();

    expect(runtime.createCase?.("Ticket overflow")).toBeNull();
    expect(runtime.getAppSettings().cases).toHaveLength(1);
  });

  test("persists anonymous local cases in browser storage across runtime reloads", async () => {
    let runtime = await loadRuntimeModule();

    runtime.createCase?.("Ticket 417");
    runtime.updateInstalledPluginConfig?.("tidb.ai", {
      baseUrl: "https://persisted.tidb.ai",
    });

    const persistedState = localStorage.getItem(LOCAL_RUNTIME_STATE_KEY);
    expect(persistedState).toBeTruthy();

    runtime = await loadRuntimeModule();

    expect(runtime.getAppSettings()).toMatchObject({
      activeCaseId: expect.any(String),
      cases: [
        expect.objectContaining({
          title: "Ticket 417",
          pluginId: "tidb.ai",
        }),
      ],
      installedPlugins: [
        {
          pluginId: "tidb.ai",
          config: {
            baseUrl: "https://persisted.tidb.ai",
          },
        },
      ],
      googleAuth: null,
    });
    expect(runtime.getAppSettings().installedPlugins).toHaveLength(1);
  });

  test("renames, resolves, reopens, archives, unarchives, and deletes cases", async () => {
    const runtime = await loadRuntimeModule();
    const created = runtime.createCase?.("Ticket 932");
    if (!created) {
      throw new Error("expected createCase to return a case");
    }

    runtime.renameCase?.(created.id, "Ticket 932 (renamed)");
    runtime.resolveCase?.(created.id);
    runtime.archiveCase?.(created.id);

    let settings = runtime.getAppSettings();
    expect(settings.cases.find((item) => item.id === created.id)).toMatchObject({
      title: "Ticket 932 (renamed)",
      activityState: "resolved",
    });
    expect(settings.cases.find((item) => item.id === created.id)?.resolvedAt).toBeTruthy();
    expect(settings.cases.find((item) => item.id === created.id)?.archivedAt).toBeTruthy();

    runtime.unarchiveCase?.(created.id);
    runtime.reopenCase?.(created.id);

    settings = runtime.getAppSettings();
    expect(settings.cases.find((item) => item.id === created.id)).toMatchObject({
      activityState: "ready",
      resolvedAt: null,
      archivedAt: null,
    });

    runtime.deleteCase?.(created.id);
    settings = runtime.getAppSettings();
    expect(settings.cases.some((item) => item.id === created.id)).toBe(false);
  });

  test("moves a case from ready to active on the first prompt while auto-titling it", async () => {
    const runtime = await loadRuntimeModule();
    const created = runtime.createCase?.("New Case");
    if (!created) {
      throw new Error("expected createCase to return a case");
    }

    runtime.autoTitleCaseFromPrompt?.(
      created.id,
      "Investigate repeated region heartbeat timeouts after the last restart",
    );

    const settings = runtime.getAppSettings();
    expect(settings.cases.find((item) => item.id === created.id)).toMatchObject({
      id: created.id,
      title: "Investigate repeated region heartbeat timeouts after the last restart",
      activityState: "active",
    });
  });

  test("converts the default placeholder case into a timestamped case on the first prompt", async () => {
    vi.useFakeTimers();
    vi.setSystemTime(new Date("2026-04-15T18:16:00.000Z"));

    try {
      const runtime = await loadRuntimeModule();
      const created = runtime.createCase?.("Default", undefined, { transient: true });
      if (!created) {
        throw new Error("expected createCase to return a case");
      }

      const placeholderState = JSON.parse(localStorage.getItem(LOCAL_RUNTIME_STATE_KEY) ?? "{}");
      expect(placeholderState.cases ?? []).toEqual([]);

      runtime.autoTitleCaseFromPrompt?.(created.id, "hello");

      const settings = runtime.getAppSettings();
      expect(settings.cases.find((item) => item.id === created.id)).toMatchObject({
        id: created.id,
        title: "2026-04-15 18:16 case",
        isPlaceholder: false,
        activityState: "active",
      });

      const persistedState = JSON.parse(localStorage.getItem(LOCAL_RUNTIME_STATE_KEY) ?? "{}");
      expect(persistedState.cases).toEqual([
        expect.objectContaining({
          id: created.id,
          title: "2026-04-15 18:16 case",
        }),
      ]);
    } finally {
      vi.useRealTimers();
    }
  });

  test("does not import a transient default placeholder into cloud sync", async () => {
    const runtime = await loadRuntimeModule();
    const fetchMock = vi.mocked(fetch);

    runtime.setAppSettings?.({
      llmRuntime: {
        baseUrl: "https://runtime.example.com",
        providerId: "",
        model: "",
      },
    });
    runtime.createCase?.("Default", undefined, { transient: true });

    fetchMock
      .mockResolvedValueOnce(
        Response.json({
          cases: [],
        }),
      )
      .mockResolvedValueOnce(
        Response.json({
          settings: null,
        }),
      )
      .mockResolvedValueOnce(
        Response.json({
          settings: {
            activeCaseId: null,
            analyticsConsent: "unknown",
            llmRuntime: {
              baseUrl: "https://runtime.example.com",
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
                  baseUrl: "https://runtime.example.com",
                },
              },
            ],
            updatedAt: "2026-04-14T12:05:00.000Z",
          },
        }),
      );

    runtime.setGoogleAuth?.({
      accessToken: "token-1",
      clientId: "google-client-id",
      email: "dev@example.com",
      hostedDomain: "example.com",
      expiresAt: "2026-04-14T16:00:00.000Z",
    });
    await runtime.syncCloudCasesIfNeeded?.();

    expect(fetchMock).not.toHaveBeenCalledWith(
      "https://runtime.example.com/v1/cases/import",
      expect.anything(),
    );
    expect(fetchMock).toHaveBeenNthCalledWith(
      1,
      "https://runtime.example.com/v1/cases",
      expect.objectContaining({
        method: "GET",
      }),
    );
    expect(fetchMock).toHaveBeenNthCalledWith(
      2,
      "https://runtime.example.com/v1/settings",
      expect.objectContaining({
        method: "GET",
      }),
    );
  });

  test("deleting the active case falls back to the most recently updated non-archived case", async () => {
    const runtime = await loadRuntimeModule();
    const firstCase = runtime.createCase?.("First");
    const secondCase = runtime.createCase?.("Second");
    const archivedCase = runtime.createCase?.("Archived");

    if (!firstCase || !secondCase || !archivedCase) {
      throw new Error("expected createCase to return cases");
    }

    runtime.archiveCase?.(archivedCase.id);
    runtime.setActiveCaseId?.(firstCase.id);
    runtime.autoTitleCaseFromPrompt?.(secondCase.id, "Second case is now hottest");
    runtime.deleteCase?.(secondCase.id);

    let settings = runtime.getAppSettings();
    expect(settings.activeCaseId).toBe(firstCase.id);

    runtime.deleteCase?.(firstCase.id);
    settings = runtime.getAppSettings();
    expect(settings.activeCaseId).toBeNull();
    expect(settings.cases).toHaveLength(1);
    expect(settings.cases[0]?.id).toBe(archivedCase.id);
  });

  test("updates the anonymous tidb.ai config and persists it locally", async () => {
    const runtime = await loadRuntimeModule();

    runtime.updateInstalledPluginConfig?.("tidb.ai", {
      baseUrl: "https://new.tidb.ai",
    });

    const settings = runtime.getAppSettings();
    expect(settings.installedPlugins.find((plugin) => plugin.pluginId === "tidb.ai")).toMatchObject({
      pluginId: "tidb.ai",
      config: {
        baseUrl: "https://new.tidb.ai",
      },
    });
    expect(settings.installedPlugins.find((plugin) => plugin.pluginId === "websearch")).toBeUndefined();
    expect(settings.llmRuntime).toEqual({
      baseUrl: "",
      providerId: "",
      model: "",
    });
    expect(localStorage.getItem(LOCAL_RUNTIME_STATE_KEY)).toBeTruthy();
  });

  test("does not expose reusable skills in app settings", async () => {
    const runtime = await loadRuntimeModule();
    const settings = runtime.getAppSettings() as Record<string, unknown>;

    expect(settings).not.toHaveProperty("skills");
  });

  test("persists anonymous analytics consent in browser storage", async () => {
    const runtime = await loadRuntimeModule();

    runtime.setAnalyticsConsent?.("granted");

    expect(runtime.getAppSettings().analyticsConsent).toBe("granted");
    expect(localStorage.getItem(LOCAL_RUNTIME_STATE_KEY)).toBeTruthy();
  });

  test("defaults assistant reply font size and persists local changes even after sign-in", async () => {
    const runtime = await loadRuntimeModule();

    expect((runtime.getAppSettings() as Record<string, unknown>).assistantReplyFontSize).toBe(
      "default",
    );

    (runtime as Record<string, any>).updateAssistantReplyFontSize?.("large");

    expect((runtime.getAppSettings() as Record<string, unknown>).assistantReplyFontSize).toBe(
      "large",
    );
    expect(
      listStoredValues().some((value) => value.includes('"assistantReplyFontSize":"large"')),
    ).toBe(true);

    runtime.setGoogleAuth?.({
      accessToken: "token-1",
      clientId: "google-client-id",
      email: "dev@example.com",
      hostedDomain: "example.com",
      expiresAt: "2026-04-14T16:00:00.000Z",
    });
    runtime.setAppSettings({
      cloudSync: {
        importedClientId: "client-1",
        lastHydratedAt: "2026-04-14T12:05:00.000Z",
        mode: "cloud",
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
            baseUrl: "https://tidb.ai",
          },
        },
      ],
    } as any);

    (runtime as Record<string, any>).updateAssistantReplyFontSize?.("small");

    expect((runtime.getAppSettings() as Record<string, unknown>).assistantReplyFontSize).toBe(
      "small",
    );
    expect(
      listStoredValues().some((value) => value.includes('"assistantReplyFontSize":"small"')),
    ).toBe(true);
  });

  test("sets, refreshes, and clears the shared Google auth state", async () => {
    const runtime = await loadRuntimeModule();

    runtime.setGoogleAuth?.({
      accessToken: "token-1",
      clientId: "client-id",
      email: "dev@example.com",
      hostedDomain: "example.com",
      expiresAt: "2026-04-14T16:00:00.000Z",
    });

    expect(runtime.getAppSettings().googleAuth).toMatchObject({
      accessToken: "token-1",
      email: "dev@example.com",
    });

    runtime.refreshGoogleAuth?.({
      accessToken: "token-2",
      expiresAt: "2026-04-14T17:00:00.000Z",
    });

    expect(runtime.getAppSettings().googleAuth).toMatchObject({
      accessToken: "token-2",
      clientId: "client-id",
      email: "dev@example.com",
      hostedDomain: "example.com",
      expiresAt: "2026-04-14T17:00:00.000Z",
    });

    runtime.clearGoogleAuth?.();
    expect(runtime.getAppSettings().googleAuth).toBeNull();
    expect(runtime.getAppSettings().cases).toEqual([]);
  });

  test("restores a Google session silently without local persistence", async () => {
    isGoogleOAuthConfiguredMock.mockReturnValue(true);
    refreshGoogleAuthSessionMock.mockResolvedValue({
      accessToken: "token-silent",
      clientId: "google-client-id",
      email: "dev@example.com",
      hostedDomain: "example.com",
      expiresAt: "2026-04-14T16:00:00.000Z",
    });

    const runtime = await loadRuntimeModule();
    const auth = await runtime.ensureGoogleAuthSession?.();

    expect(refreshGoogleAuthSessionMock).toHaveBeenCalledTimes(1);
    expect(auth).toMatchObject({
      accessToken: "token-silent",
    });
    expect(runtime.getAppSettings().googleAuth).toMatchObject({
      accessToken: "token-silent",
    });
    expect(localStorage.length).toBe(0);
  });

  test("imports in-memory cases into cloud mode, hydrates cloud settings, and restores local state on sign out", async () => {
    vi.stubGlobal("fetch", vi.fn());
    const fetchMock = vi.mocked(fetch);
    const runtime = await loadRuntimeModule();
    const threadHistory = await import("./thread-history");

    runtime.setAppSettings({
      activeCaseId: "case-local-1",
      analyticsConsent: "unknown",
      cloudSync: {
        importedClientId: null,
        lastHydratedAt: null,
        mode: "local",
      },
      llmRuntime: {
        baseUrl: "https://runtime.example.com",
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
      cases: [
        {
          id: "case-local-1",
          title: "Local case before login",
          pluginId: "tidb.ai",
          activityState: "active",
          resolvedAt: null,
          archivedAt: null,
          createdAt: "2026-04-14T09:30:00.000Z",
          updatedAt: "2026-04-14T12:00:00.000Z",
        },
      ],
    });

    const localHistory = threadHistory.createCaseHistoryAdapter("case-local-1");
    await localHistory.append({
      parentId: null,
      message: {
        id: "m-1",
        role: "user",
        content: [{ type: "text", text: "Local cloud import payload." }],
        createdAt: new Date("2026-04-14T12:00:00.000Z"),
        attachments: [],
        metadata: {},
      },
    } as never);

    fetchMock
      .mockResolvedValueOnce(
        Response.json({
          alreadyImported: false,
          importedCases: 1,
        }),
      )
      .mockResolvedValueOnce(
        Response.json({
          cases: [
            {
              id: "case-cloud-1",
              title: "Cloud case after login",
              pluginId: "tidb.ai",
              activityState: "active",
              resolvedAt: null,
              archivedAt: null,
              createdAt: "2026-04-14T09:30:00.000Z",
              updatedAt: "2026-04-14T12:05:00.000Z",
              summary: "Hydrated from cloud.",
              signals: ["Hydrated from cloud."],
              messagesPreview: [
                {
                  role: "tihc",
                  text: "Hydrated from cloud.",
                },
              ],
            },
          ],
        }),
      )
      .mockResolvedValueOnce(
        Response.json({
          settings: {
            activeCaseId: "case-cloud-1",
            analyticsConsent: "granted",
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
                  baseUrl: "https://tidb.ai",
                },
              },
              {
                pluginId: "websearch",
                label: "Web Search",
                kind: "mcp",
                capabilities: ["mcp"],
                config: {
                  enabled: false,
                  mode: "off",
                  primaryEngine: "bing",
                },
              },
            ],
            updatedAt: "2026-04-14T12:05:00.000Z",
          },
        }),
      );

    runtime.setGoogleAuth?.({
      accessToken: "token-1",
      clientId: "google-client-id",
      email: "dev@example.com",
      hostedDomain: "example.com",
      expiresAt: "2026-04-14T16:00:00.000Z",
    });
    await runtime.syncCloudCasesIfNeeded?.();

    expect(fetchMock).toHaveBeenNthCalledWith(
      1,
      "https://runtime.example.com/v1/cases/import",
      expect.objectContaining({
        method: "POST",
      }),
    );
    expect(fetchMock).toHaveBeenNthCalledWith(
      2,
      "https://runtime.example.com/v1/cases",
      expect.objectContaining({
        method: "GET",
      }),
    );
    expect(fetchMock).toHaveBeenNthCalledWith(
      3,
      "https://runtime.example.com/v1/settings",
      expect.objectContaining({
        method: "GET",
      }),
    );
    expect(runtime.getAppSettings()).toMatchObject({
      activeCaseId: "case-cloud-1",
      analyticsConsent: "granted",
      cloudSync: {
        importedClientId: expect.any(String),
        mode: "cloud",
      },
      llmRuntime: {
        baseUrl: "https://runtime.example.com",
        providerId: "openai",
        model: "gpt-4.1-mini",
      },
      cases: [
        {
          id: "case-cloud-1",
          title: "Cloud case after login",
        },
      ],
      installedPlugins: [
        {
          pluginId: "tidb.ai",
          config: {
            baseUrl: "https://tidb.ai",
          },
        },
        {
          pluginId: "websearch",
          config: {
            enabled: false,
            mode: "off",
            primaryEngine: "bing",
          },
        },
      ],
    });

    runtime.clearGoogleAuth?.();

    expect(runtime.getAppSettings()).toMatchObject({
      activeCaseId: "case-local-1",
      analyticsConsent: "unknown",
      cloudSync: {
        importedClientId: null,
        mode: "local",
      },
      llmRuntime: {
        baseUrl: "",
        providerId: "",
        model: "",
      },
      cases: [
        {
          id: "case-local-1",
          title: "Local case before login",
        },
      ],
      googleAuth: null,
    });
  });

  test("persists cloud settings payload without reusable skills when cloud sync is active", async () => {
    const fetchMock = vi.mocked(fetch);
    fetchMock.mockResolvedValue(
      Response.json({
        settings: {
          activeCaseId: null,
          analyticsConsent: "unknown",
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
          updatedAt: "2026-04-14T12:05:00.000Z",
        },
      }),
    );

    const runtime = await loadRuntimeModule();
    runtime.setAppSettings({
      cloudSync: {
        importedClientId: "client-1",
        lastHydratedAt: "2026-04-14T12:05:00.000Z",
        mode: "cloud",
      },
      googleAuth: {
        accessToken: "token-1",
        clientId: "google-client-id",
        email: "dev@example.com",
        hostedDomain: "example.com",
        expiresAt: "2026-04-14T16:00:00.000Z",
      },
      llmRuntime: {
        baseUrl: "https://runtime.example.com",
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
    });

    runtime.setAnalyticsConsent?.("granted");

    expect(fetchMock).toHaveBeenCalledWith(
      "https://runtime.example.com/v1/settings",
      expect.objectContaining({
        method: "PUT",
      }),
    );
    const [, request] = fetchMock.mock.calls.at(-1)!;
    const payload = JSON.parse(String((request as RequestInit).body));
    expect(payload).not.toHaveProperty("skills");
    expect(payload.analyticsConsent).toBe("granted");
  });

  test("returns a stable snapshot reference until settings actually change", async () => {
    const runtime = await loadRuntimeModule();

    const first = runtime.getAppSettingsSnapshot();
    const second = runtime.getAppSettingsSnapshot();

    expect(second).toBe(first);

    runtime.createCase?.("Ticket 1303");

    const third = runtime.getAppSettingsSnapshot();
    expect(third).not.toBe(first);
  });
});
