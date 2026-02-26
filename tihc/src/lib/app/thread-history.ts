import type {
  ExportedMessageRepository,
  ThreadHistoryAdapter,
} from "@assistant-ui/react";

type RepoMessageItem = ExportedMessageRepository["messages"][number];

const THREAD_HISTORY_STORAGE_KEY = "tihc_thread_history_v1";
const MAX_PERSISTED_MESSAGES = 10;

function hasLocalStorage(): boolean {
  return (
    typeof globalThis !== "undefined" &&
    typeof globalThis.localStorage !== "undefined"
  );
}

function emptyRepo(): ExportedMessageRepository {
  return {
    headId: null,
    messages: [],
  };
}

function parseRepo(raw: string): ExportedMessageRepository {
  try {
    const parsed = JSON.parse(raw) as Partial<ExportedMessageRepository>;
    if (!parsed || typeof parsed !== "object") return emptyRepo();
    const messages = Array.isArray(parsed.messages) ? parsed.messages : [];
    const headId =
      typeof parsed.headId === "string" || parsed.headId === null
        ? parsed.headId
        : null;
    return {
      headId,
      messages: messages as ExportedMessageRepository["messages"],
    };
  } catch {
    return emptyRepo();
  }
}

function loadRepoFromStorage(): ExportedMessageRepository {
  if (!hasLocalStorage()) return emptyRepo();
  const raw = globalThis.localStorage.getItem(THREAD_HISTORY_STORAGE_KEY);
  if (!raw) return emptyRepo();
  return parseRepo(raw);
}

function saveRepoToStorage(repo: ExportedMessageRepository): void {
  if (!hasLocalStorage()) return;
  try {
    globalThis.localStorage.setItem(
      THREAD_HISTORY_STORAGE_KEY,
      JSON.stringify(repo),
    );
  } catch {
    // ignore storage quota and serialization errors
  }
}

function trimRepoToLatestMessages(
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

export function createLocalThreadHistoryAdapter(): ThreadHistoryAdapter {
  return {
    async load() {
      return trimRepoToLatestMessages(loadRepoFromStorage());
    },
    async append(item: RepoMessageItem) {
      const repo = loadRepoFromStorage();
      const messageId = item.message.id;
      const index = repo.messages.findIndex((m) => m.message.id === messageId);
      if (index >= 0) {
        repo.messages[index] = item;
      } else {
        repo.messages.push(item);
      }
      repo.headId = messageId;
      saveRepoToStorage(trimRepoToLatestMessages(repo));
    },
  };
}

export function clearLocalThreadHistory(): void {
  if (!hasLocalStorage()) return;
  try {
    globalThis.localStorage.removeItem(THREAD_HISTORY_STORAGE_KEY);
  } catch {
    // ignore
  }
}
