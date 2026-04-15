import { Trash2Icon } from "lucide-react";
import { deleteCase } from "@/lib/app/runtime";
import { confirmDeleteCase } from "@/lib/app/case-delete";
import {
  ANONYMOUS_LOCAL_STORAGE_LIMIT_BYTES,
  estimateAnonymousLocalStorageUsageBytes,
  estimateCaseLocalStorageBytes,
  formatStorageBytes,
  isAnonymousLocalStorageLimitReached,
  sortCasesByLocalStorageBytesDesc,
} from "@/lib/app/anonymous-local-case-limit";
import type { AppRuntimeSettings, CaseWorkspace } from "@/lib/chat/agent-types";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { ScrollArea } from "@/components/ui/scroll-area";

type AnonymousLocalCaseLimitDialogProps = {
  settings: Pick<
    AppRuntimeSettings,
    "activeCaseId" | "analyticsConsent" | "cases" | "googleAuth" | "installedPlugins"
  >;
};

function formatUpdatedAt(value: string): string {
  const parsed = Date.parse(value);
  if (Number.isNaN(parsed)) return "Unknown update";

  return new Intl.DateTimeFormat("en-US", {
    month: "short",
    day: "numeric",
    hour: "numeric",
    minute: "2-digit",
  }).format(new Date(parsed));
}

function formatCaseState(caseWorkspace: CaseWorkspace): string {
  if (caseWorkspace.archivedAt) return "Archived";
  if (caseWorkspace.activityState === "resolved") return "Resolved";
  if (caseWorkspace.activityState === "active") return "Investigating";
  return "Ready";
}

export function AnonymousLocalCaseLimitDialog({
  settings,
}: AnonymousLocalCaseLimitDialogProps) {
  const isBlocked = isAnonymousLocalStorageLimitReached(settings);
  if (!isBlocked) return null;

  const usageBytes = estimateAnonymousLocalStorageUsageBytes(settings);
  const storedCases = sortCasesByLocalStorageBytesDesc(settings.cases);
  const handleDelete = (caseWorkspace: CaseWorkspace) => {
    if (!confirmDeleteCase(caseWorkspace.title)) return;
    deleteCase(caseWorkspace.id);
  };

  return (
    <Dialog
      open={isBlocked}
      onOpenChange={(open) => {
        if (open) return;
      }}
    >
      <DialogContent
        showCloseButton={false}
        onEscapeKeyDown={(event) => event.preventDefault()}
        onPointerDownOutside={(event) => event.preventDefault()}
        className="max-w-[560px] rounded-xl border bg-background p-6 shadow-lg"
      >
        <DialogHeader className="space-y-2 text-left">
          <DialogTitle className="text-2xl font-semibold tracking-tight">
            Delete local cases to continue
          </DialogTitle>
          <DialogDescription className="space-y-2 text-sm leading-6 text-muted-foreground">
            <span className="block">
              Anonymous mode is limited by browser storage usage, not case count.
            </span>
            <span className="block">
              Delete one or more larger cases below until local usage drops under{" "}
              {formatStorageBytes(ANONYMOUS_LOCAL_STORAGE_LIMIT_BYTES)}.
            </span>
          </DialogDescription>
        </DialogHeader>

        <div className="rounded-xl border bg-muted/30">
          <div className="flex items-center justify-between border-b px-4 py-3 text-xs font-medium uppercase text-muted-foreground">
            <span>Local usage</span>
            <span>
              {formatStorageBytes(usageBytes)} / {formatStorageBytes(ANONYMOUS_LOCAL_STORAGE_LIMIT_BYTES)}
            </span>
          </div>
          <ScrollArea className="h-[320px]">
            <div className="space-y-3 p-4">
              {storedCases.map((caseWorkspace) => (
                <div
                  key={caseWorkspace.id}
                  className="flex items-start justify-between gap-3 rounded-lg border bg-background px-4 py-3"
                >
                  <div className="min-w-0 space-y-1">
                    <div className="truncate text-sm font-semibold text-foreground">
                      {caseWorkspace.title}
                    </div>
                    <div className="text-xs text-muted-foreground">
                      {formatCaseState(caseWorkspace)} · {formatStorageBytes(estimateCaseLocalStorageBytes(caseWorkspace))} · Updated{" "}
                      {formatUpdatedAt(caseWorkspace.updatedAt)}
                    </div>
                  </div>
                  <Button
                    type="button"
                    variant="destructive"
                    size="sm"
                    aria-label={`Delete case ${caseWorkspace.title}`}
                    className="rounded-full"
                    onClick={() => handleDelete(caseWorkspace)}
                  >
                    <Trash2Icon />
                    Delete
                  </Button>
                </div>
              ))}
            </div>
          </ScrollArea>
        </div>
      </DialogContent>
    </Dialog>
  );
}
