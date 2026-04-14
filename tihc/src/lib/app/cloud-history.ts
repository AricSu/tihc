import type {
  ExportedMessageRepository,
  ThreadHistoryAdapter,
} from "@assistant-ui/react";
import { getAppSettingsSnapshot } from "@/lib/app/runtime";
import {
  getStoredCaseHistory,
  saveStoredCaseHistory,
} from "@/lib/app/cloud-cases";
import { emptyRepo, trimRepoToLatestMessages } from "@/lib/app/thread-history";

type RepoMessageItem = ExportedMessageRepository["messages"][number];

function appendRepoItem(
  repo: ExportedMessageRepository,
  item: RepoMessageItem,
): ExportedMessageRepository {
  const nextRepo: ExportedMessageRepository = {
    headId: repo.headId,
    messages: [...repo.messages],
  };
  const index = nextRepo.messages.findIndex((message) => message.message.id === item.message.id);
  if (index >= 0) {
    nextRepo.messages[index] = item;
  } else {
    nextRepo.messages.push(item);
  }
  nextRepo.headId = item.message.id;
  return trimRepoToLatestMessages(nextRepo);
}

export function createCloudCaseHistoryAdapter(caseId: string): ThreadHistoryAdapter {
  return {
    async load() {
      const repository = await getStoredCaseHistory(getAppSettingsSnapshot(), caseId);
      return repository ? trimRepoToLatestMessages(repository) : emptyRepo();
    },
    async append(item: RepoMessageItem) {
      const current = await this.load();
      const nextRepository = appendRepoItem(current, item);
      await saveStoredCaseHistory(getAppSettingsSnapshot(), caseId, nextRepository);
    },
  };
}
