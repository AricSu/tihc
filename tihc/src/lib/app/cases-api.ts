import { createCaseHistoryAdapter } from "@/lib/app/thread-history";
import {
  isCloudSyncEnabled,
  listStoredCases,
} from "@/lib/app/cloud-cases";
import { resolveRuntimeBackendBaseUrl } from "@/lib/app/backend-endpoint";
import { getAppSettingsSnapshot } from "@/lib/app/runtime";
import type {
  AppRuntimeSettings,
  CaseWorkspace,
  StoredCaseRecord,
} from "@/lib/chat/agent-types";

export type DashboardCaseRecord = {
  id: string;
  title: string;
  status: string;
  priority: string;
  channel: string;
  updatedAt: string;
  executionTarget: string;
  owner: string;
  summary: string;
  signals: string[];
  messages: Array<{
    role: "operator" | "tihc";
    text: string;
  }>;
};

type RepoMessageItem = {
  message?: {
    role?: unknown;
    content?: unknown;
  };
};

const EMPTY_SUMMARY = "No thread activity yet.";

function compareByUpdatedAtDesc(a: CaseWorkspace, b: CaseWorkspace): number {
  return Date.parse(b.updatedAt) - Date.parse(a.updatedAt);
}

function extractTextFromContent(content: unknown): string {
  if (typeof content === "string") return content.trim();
  if (!Array.isArray(content)) return "";

  return content
    .map((part) => {
      if (!part || typeof part !== "object") return "";
      const text = (part as { text?: unknown }).text;
      return typeof text === "string" ? text.trim() : "";
    })
    .filter(Boolean)
    .join("\n")
    .trim();
}

function toDashboardMessage(item: RepoMessageItem): { role: "operator" | "tihc"; text: string } | null {
  const text = extractTextFromContent(item.message?.content);
  if (!text) return null;

  return {
    role: item.message?.role === "assistant" ? "tihc" : "operator",
    text,
  };
}

function mapActivityStateToStatus(activityState: CaseWorkspace["activityState"]): string {
  if (activityState === "active") return "Investigating";
  if (activityState === "resolved") return "Resolved";
  return "Watching";
}

function mapActivityStateToPriority(activityState: CaseWorkspace["activityState"]): string {
  if (activityState === "active") return "Hot";
  if (activityState === "resolved") return "Closed";
  return "Ready";
}

function buildFallbackMessages(): Array<{ role: "operator" | "tihc"; text: string }> {
  return [
    {
      role: "tihc",
      text: EMPTY_SUMMARY,
    },
  ];
}

function resolvePluginLabel(settings: AppRuntimeSettings, pluginId: string): string {
  return (
    settings.installedPlugins.find((item) => item.pluginId === pluginId)?.label ??
    pluginId
  );
}

async function buildDashboardCaseRecord(
  settings: AppRuntimeSettings,
  caseWorkspace: CaseWorkspace,
): Promise<DashboardCaseRecord> {
  // The dashboard needs only a lightweight read model, so we derive it from
  // the stored thread repository instead of leaking the repository shape into UI code.
  const repo = await createCaseHistoryAdapter(caseWorkspace.id).load();
  const textMessages = (repo.messages as RepoMessageItem[])
    .map(toDashboardMessage)
    .filter((item): item is NonNullable<typeof item> => !!item);
  const previewMessages = textMessages.slice(-2);
  const latestFirstSignals = [...textMessages]
    .reverse()
    .map((item) => item.text)
    .filter((value, index, values) => values.indexOf(value) === index)
    .slice(0, 3);
  const summary =
    [...textMessages]
      .reverse()
      .find((item) => item.role === "tihc")?.text ??
    textMessages[textMessages.length - 1]?.text ??
    EMPTY_SUMMARY;

  return {
    id: caseWorkspace.id,
    title: caseWorkspace.title,
    status: mapActivityStateToStatus(caseWorkspace.activityState),
    priority: mapActivityStateToPriority(caseWorkspace.activityState),
    channel: resolvePluginLabel(settings, caseWorkspace.pluginId),
    updatedAt: caseWorkspace.updatedAt,
    executionTarget: resolvePluginLabel(settings, caseWorkspace.pluginId),
    owner: "TIHC",
    summary,
    signals: latestFirstSignals.length ? latestFirstSignals : [EMPTY_SUMMARY],
    messages: previewMessages.length ? previewMessages : buildFallbackMessages(),
  };
}

async function buildLocalDashboardCasesSnapshot(
  settings: AppRuntimeSettings,
): Promise<DashboardCaseRecord[]> {
  const visibleCases = settings.cases
    .filter((item) => item.archivedAt === null)
    .sort(compareByUpdatedAtDesc);

  return Promise.all(visibleCases.map((item) => buildDashboardCaseRecord(settings, item)));
}

export function buildDashboardCaseRecordFromStoredCase(
  settings: AppRuntimeSettings,
  storedCase: StoredCaseRecord,
): DashboardCaseRecord {
  return {
    id: storedCase.id,
    title: storedCase.title,
    status: mapActivityStateToStatus(storedCase.activityState),
    priority: mapActivityStateToPriority(storedCase.activityState),
    channel: resolvePluginLabel(settings, storedCase.pluginId),
    updatedAt: storedCase.updatedAt,
    executionTarget: resolvePluginLabel(settings, storedCase.pluginId),
    owner: "TIHC",
    summary: storedCase.summary,
    signals: storedCase.signals.length ? storedCase.signals : [EMPTY_SUMMARY],
    messages: storedCase.messagesPreview.length ? storedCase.messagesPreview : buildFallbackMessages(),
  };
}

export async function listDashboardCases(): Promise<DashboardCaseRecord[]> {
  const settings = getAppSettingsSnapshot();
  if (!isCloudSyncEnabled(settings)) {
    return buildLocalDashboardCasesSnapshot(settings);
  }

  if (!resolveRuntimeBackendBaseUrl(settings).trim()) {
    return [];
  }

  const storedCases = await listStoredCases(settings);
  return storedCases
    ? storedCases.map((item) => buildDashboardCaseRecordFromStoredCase(settings, item))
    : [];
}
