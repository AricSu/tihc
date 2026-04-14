import type { ExportedMessageRepository } from "@assistant-ui/react";
import { beforeEach, describe, expect, test, vi } from "vitest";

import {
  clearLocalCaseHistory,
  clearLocalThreadHistory,
  createCaseHistoryAdapter,
  createLocalCaseHistoryAdapter,
  createScopedCaseHistoryAdapter,
  createScopedThreadHistoryAdapter,
} from "./thread-history";

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

function buildRepoItem(id: string, parentId: string | null = null) {
  return {
    parentId,
    message: {
      id,
      role: "user",
      content: [{ type: "text", text: `message ${id}` }],
      createdAt: new Date(0),
      attachments: [],
      metadata: {},
    },
  } as unknown as ExportedMessageRepository["messages"][number];
}

describe("scoped thread history", () => {
  beforeEach(() => {
    installMockStorage();
    localStorage.clear();
    clearLocalThreadHistory();
  });

  test("isolates in-memory threads by agent instance id", async () => {
    const agentA = createScopedThreadHistoryAdapter("agent-a");
    const agentB = createScopedThreadHistoryAdapter("agent-b");

    await agentA.append(buildRepoItem("a-1"));
    await agentB.append(buildRepoItem("b-1"));

    const repoA = await agentA.load();
    const repoB = await agentB.load();

    expect(repoA.messages.map((item) => item.message.id)).toEqual(["a-1"]);
    expect(repoB.messages.map((item) => item.message.id)).toEqual(["b-1"]);
  });

  test("clears only the requested agent scope", async () => {
    const agentA = createScopedThreadHistoryAdapter("agent-a");
    const agentB = createScopedThreadHistoryAdapter("agent-b");

    await agentA.append(buildRepoItem("a-1"));
    await agentB.append(buildRepoItem("b-1"));

    clearLocalThreadHistory("agent-a");

    const repoA = await agentA.load();
    const repoB = await agentB.load();

    expect(repoA.messages).toHaveLength(0);
    expect(repoB.messages.map((item) => item.message.id)).toEqual(["b-1"]);
  });

  test("isolates threads by thread id within the same agent", async () => {
    const threadA = createScopedThreadHistoryAdapter("agent-a", "thread-a");
    const threadB = createScopedThreadHistoryAdapter("agent-a", "thread-b");

    await threadA.append(buildRepoItem("a-1"));
    await threadB.append(buildRepoItem("b-1"));

    const repoA = await threadA.load();
    const repoB = await threadB.load();

    expect(repoA.messages.map((item) => item.message.id)).toEqual(["a-1"]);
    expect(repoB.messages.map((item) => item.message.id)).toEqual(["b-1"]);
  });

  test("keeps history bound to case id within the current session", async () => {
    const caseHistory = createCaseHistoryAdapter("case-417");
    const sameCaseAfterTargetSwitch = createCaseHistoryAdapter("case-417");

    await caseHistory.append(buildRepoItem("m-1"));

    const repo = await sameCaseAfterTargetSwitch.load();

    expect(repo.messages.map((item) => item.message.id)).toEqual(["m-1"]);
  });

  test("supports nested case workspace scopes", async () => {
    const mainCase = createScopedCaseHistoryAdapter("case-1");
    const branchCase = createScopedCaseHistoryAdapter("case-1", "workspace-b");

    await mainCase.append(buildRepoItem("m-1"));
    await branchCase.append(buildRepoItem("b-1"));

    const mainRepo = await mainCase.load();
    const branchRepo = await branchCase.load();

    expect(mainRepo.messages.map((item) => item.message.id)).toEqual(["m-1"]);
    expect(branchRepo.messages.map((item) => item.message.id)).toEqual(["b-1"]);
  });

  test("supports a local case history adapter alias", async () => {
    const localCaseHistory = createLocalCaseHistoryAdapter();
    const defaultThreadHistory = createScopedThreadHistoryAdapter("default");

    await localCaseHistory.append(buildRepoItem("local-1"));

    const repo = await defaultThreadHistory.load();

    expect(repo.messages.map((item) => item.message.id)).toEqual(["local-1"]);
  });

  test("clears only the requested case scope", async () => {
    const caseA = createCaseHistoryAdapter("case-a");
    const caseB = createCaseHistoryAdapter("case-b");

    await caseA.append(buildRepoItem("a-1"));
    await caseB.append(buildRepoItem("b-1"));

    clearLocalCaseHistory("case-a");

    const repoA = await caseA.load();
    const repoB = await caseB.load();

    expect(repoA.messages).toHaveLength(0);
    expect(repoB.messages.map((item) => item.message.id)).toEqual(["b-1"]);
  });

  test("restores case history from browser storage after module reload", async () => {
    const caseHistory = createCaseHistoryAdapter("case-417");
    await caseHistory.append(buildRepoItem("m-1"));

    vi.resetModules();
    const reloaded = await import("./thread-history");
    const restoredHistory = reloaded.createCaseHistoryAdapter("case-417");
    const repo = await restoredHistory.load();

    expect(repo.messages.map((item) => item.message.id)).toEqual(["m-1"]);
  });
});
