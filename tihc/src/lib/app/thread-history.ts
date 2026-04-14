import type {
  ExportedMessageRepository,
  ThreadHistoryAdapter,
} from "@assistant-ui/react";
import {
  listLocalHistoryScopes,
  readLocalHistory,
  removeLocalHistory,
  writeLocalHistory,
} from "@/lib/app/local-browser-persistence";

type RepoMessageItem = ExportedMessageRepository["messages"][number];

const DEFAULT_SCOPE = "default";
const MAX_PERSISTED_MESSAGES = 10;
const memoryRepositories = new Map<string, ExportedMessageRepository>();

export function emptyRepo(): ExportedMessageRepository {
  return {
    headId: null,
    messages: [],
  };
}

function cloneRepo(repo: ExportedMessageRepository): ExportedMessageRepository {
  return JSON.parse(JSON.stringify(repo)) as ExportedMessageRepository;
}

function getHistoryScope(
  caseId: string = DEFAULT_SCOPE,
  workspaceId?: string,
): string {
  return workspaceId ? `${caseId}:${workspaceId}` : caseId;
}

function loadRepoFromMemory(
  caseId?: string,
  workspaceId?: string,
): ExportedMessageRepository {
  const scope = getHistoryScope(caseId, workspaceId);
  const repo = memoryRepositories.get(scope) ?? readLocalHistory(scope);
  if (!repo) return emptyRepo();

  memoryRepositories.set(scope, cloneRepo(repo));
  return cloneRepo(repo);
}

function saveRepoToMemory(
  repo: ExportedMessageRepository,
  caseId?: string,
  workspaceId?: string,
): void {
  const scope = getHistoryScope(caseId, workspaceId);
  const clonedRepo = cloneRepo(repo);
  memoryRepositories.set(scope, clonedRepo);
  writeLocalHistory(scope, clonedRepo);
}

export function trimRepoToLatestMessages(
  repo: ExportedMessageRepository,
): ExportedMessageRepository {
  if (repo.messages.length <= MAX_PERSISTED_MESSAGES) return repo;

  const byId = new Map<string, RepoMessageItem>();
  for (const item of repo.messages) {
    byId.set(item.message.id, item);
  }

  let headId = repo.headId;
  if (!headId || !byId.has(headId)) {
    headId = repo.messages.at(-1)?.message.id ?? null;
  }
  if (!headId) return emptyRepo();

  const reversedChain: RepoMessageItem[] = [];
  const visited = new Set<string>();
  let currentId: string | null = headId;
  while (currentId && !visited.has(currentId)) {
    visited.add(currentId);
    const item = byId.get(currentId);
    if (!item) break;
    reversedChain.push(item);
    currentId = item.parentId;
  }

  const chain = reversedChain.reverse();
  const kept = chain.slice(-MAX_PERSISTED_MESSAGES);
  if (!kept.length) return emptyRepo();

  const normalized = kept.map((item, index) => ({
    ...item,
    parentId: index === 0 ? null : kept[index - 1]?.message.id ?? null,
  }));

  return {
    headId: normalized[normalized.length - 1]?.message.id ?? null,
    messages: normalized,
  };
}

export function createScopedThreadHistoryAdapter(
  agentInstanceId: string,
  threadId?: string,
): ThreadHistoryAdapter {
  return {
    async load() {
      return trimRepoToLatestMessages(loadRepoFromMemory(agentInstanceId, threadId));
    },
    async append(item: RepoMessageItem) {
      const repo = loadRepoFromMemory(agentInstanceId, threadId);
      const messageId = item.message.id;
      const index = repo.messages.findIndex((message) => message.message.id === messageId);
      if (index >= 0) {
        repo.messages[index] = item;
      } else {
        repo.messages.push(item);
      }
      repo.headId = messageId;
      saveRepoToMemory(trimRepoToLatestMessages(repo), agentInstanceId, threadId);
    },
  };
}

export function createScopedCaseHistoryAdapter(
  caseId: string,
  workspaceId?: string,
): ThreadHistoryAdapter {
  return createScopedThreadHistoryAdapter(caseId, workspaceId);
}

export function createCaseHistoryAdapter(caseId: string): ThreadHistoryAdapter {
  return createScopedCaseHistoryAdapter(caseId);
}

export function createLocalCaseHistoryAdapter(): ThreadHistoryAdapter {
  return createScopedCaseHistoryAdapter(DEFAULT_SCOPE);
}

export function createLocalThreadHistoryAdapter(): ThreadHistoryAdapter {
  return createLocalCaseHistoryAdapter();
}

export function hasStoredCaseHistory(caseId: string): boolean {
  return loadRepoFromMemory(caseId).messages.length > 0;
}

export function clearLocalThreadHistory(agentInstanceId?: string, threadId?: string): void {
  const scopedKey = agentInstanceId
    ? getHistoryScope(agentInstanceId, threadId)
    : null;

  const storedKeys = new Set<string>([...memoryRepositories.keys(), ...listLocalHistoryScopes()]);

  for (const key of storedKeys) {
    if (!scopedKey) {
      memoryRepositories.delete(key);
      removeLocalHistory(key);
      continue;
    }

    if (threadId) {
      if (key === scopedKey) {
        memoryRepositories.delete(key);
        removeLocalHistory(key);
      }
      continue;
    }

    if (key === scopedKey || key.startsWith(`${scopedKey}:`)) {
      memoryRepositories.delete(key);
      removeLocalHistory(key);
    }
  }
}

export function clearLocalCaseHistory(caseId?: string): void {
  clearLocalThreadHistory(caseId);
}
