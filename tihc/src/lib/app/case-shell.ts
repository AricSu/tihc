import type { AppRuntimeSettings, CaseWorkspace } from "@/lib/chat/agent-types";

export function resolveActiveCaseWorkspace(settings: AppRuntimeSettings): {
  activeCase: CaseWorkspace | null;
} {
  const visibleCases = settings.cases.filter((item) => item.archivedAt === null);
  const activeCase =
    visibleCases.find((item) => item.id === settings.activeCaseId) ?? visibleCases[0] ?? null;

  return {
    activeCase,
  };
}
