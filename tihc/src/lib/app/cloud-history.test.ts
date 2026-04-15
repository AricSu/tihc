import type { ExportedMessageRepository } from "@assistant-ui/react";
import { beforeEach, describe, expect, test, vi } from "vitest";

const {
  getAppSettingsSnapshotMock,
  getStoredCaseHistoryMock,
  saveStoredCaseHistoryMock,
} = vi.hoisted(() => ({
  getAppSettingsSnapshotMock: vi.fn(),
  getStoredCaseHistoryMock: vi.fn(),
  saveStoredCaseHistoryMock: vi.fn(),
}));

vi.mock("@/lib/app/runtime", () => ({
  getAppSettingsSnapshot: getAppSettingsSnapshotMock,
}));

vi.mock("@/lib/app/cloud-cases", () => ({
  getStoredCaseHistory: getStoredCaseHistoryMock,
  saveStoredCaseHistory: saveStoredCaseHistoryMock,
}));

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

describe("cloud history adapter", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    getAppSettingsSnapshotMock.mockReturnValue({
      activeCaseId: "case-1",
      analyticsConsent: "unknown",
      cases: [],
      cloudSync: {
        importedClientId: "client-123",
        lastHydratedAt: "2026-04-14T12:00:00.000Z",
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
        baseUrl: "https://runtime.example.com",
        providerId: "openai",
        model: "gpt-4.1-mini",
      },
      installedPlugins: [],
    });
    saveStoredCaseHistoryMock.mockResolvedValue(null);
  });

  test("preserves repository metadata when appending cloud-backed case history", async () => {
    getStoredCaseHistoryMock.mockResolvedValue({
      headId: "m-1",
      messages: [
        buildRepoItem("m-1", "user", "First turn"),
      ],
      metadata: {
        tidbAi: {
          chatId: "chat-upstream-123",
        },
      },
    });

    const { createCloudCaseHistoryAdapter } = await import("./cloud-history");
    const adapter = createCloudCaseHistoryAdapter("case-1");
    await adapter.append(buildRepoItem("m-2", "assistant", "Second turn", "m-1"));

    expect(saveStoredCaseHistoryMock).toHaveBeenCalledWith(
      expect.any(Object),
      "case-1",
      expect.objectContaining({
        headId: "m-2",
        metadata: {
          tidbAi: {
            chatId: "chat-upstream-123",
          },
        },
      }),
    );
  });
});
