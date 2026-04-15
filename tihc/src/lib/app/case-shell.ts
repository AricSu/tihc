import type { AppRuntimeSettings, CaseWorkspace } from "@/lib/chat/agent-types";
import { listOpenCases } from "@/lib/app/case-list";

export function resolveActiveCaseWorkspace(settings: AppRuntimeSettings): {
  activeCase: CaseWorkspace | null;
} {
  const visibleCases = listOpenCases(settings.cases);
  const activeCase =
    visibleCases.find((item) => item.id === settings.activeCaseId) ?? visibleCases[0] ?? null;

  return {
    activeCase,
  };
}
