import { useState } from "react";
import { Button } from "@/components/ui/button";
import { CaseRenameDialog } from "@/components/case-rename-dialog";
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from "@/components/ui/context-menu";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import type { AppRuntimeSettings } from "@/lib/chat/agent-types";
import { AnonymousLocalCaseLimitDialog } from "@/components/agent/anonymous-local-case-limit-dialog";
import { resolveActiveCaseWorkspace } from "@/lib/app/case-shell";
import { MLCProvider } from "@/components/MLCProvider";
import { Thread } from "@/components/assistant-ui/thread";
import { deleteCase, renameCase, setActiveCaseId } from "@/lib/app/runtime";
import { isPlaceholderCase, listVisibleCases } from "@/lib/app/case-list";
import { openCaseCreationPage, openGeneralSettingsPage } from "@/lib/app/settings-page";
import { CheckIcon, ChevronsUpDownIcon, SettingsIcon } from "lucide-react";

type CaseShellProps = {
  settings: AppRuntimeSettings;
};

export function CaseShell({ settings }: CaseShellProps) {
  const visibleCases = listVisibleCases(settings.cases);
  const { activeCase } = resolveActiveCaseWorkspace(settings);
  const limitDialog = <AnonymousLocalCaseLimitDialog settings={settings} />;

  if (!activeCase) {
    return (
      <>
        {limitDialog}
        <div className="h-full" />
      </>
    );
  }

  return (
    <>
      {limitDialog}
      <div className="flex h-full min-h-0 flex-col bg-background text-foreground">
        <div className="min-h-0 flex-1 overflow-hidden">
          <MLCProvider key={activeCase.id} caseWorkspace={activeCase}>
            <Thread
              composerStart={
                <CaseSelectorControls
                  activeCaseId={activeCase.id}
                  caseTitle={isPlaceholderCase(activeCase) ? "New case" : activeCase.title}
                  visibleCaseItems={visibleCases}
                />
              }
            />
          </MLCProvider>
        </div>
      </div>
    </>
  );
}

type CaseSelectorControlsProps = {
  activeCaseId: string;
  caseTitle: string;
  visibleCaseItems: AppRuntimeSettings["cases"];
};

function CaseSelectorControls({
  activeCaseId,
  caseTitle,
  visibleCaseItems,
}: CaseSelectorControlsProps) {
  const [renameTarget, setRenameTarget] = useState<{ id: string; title: string } | null>(null);

  return (
    <>
      <div className="flex items-center gap-2">
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button
              type="button"
              variant="ghost"
              size="sm"
              className="h-8 max-w-44 justify-between rounded-full px-3 text-left font-normal"
              aria-label="Select case"
            >
              <span className="truncate">{caseTitle}</span>
              <ChevronsUpDownIcon className="size-4 text-muted-foreground" />
            </Button>
          </DropdownMenuTrigger>

          <DropdownMenuContent side="top" align="start" className="w-56 max-w-[calc(100vw-2rem)]">
            <DropdownMenuLabel>Cases</DropdownMenuLabel>
            <DropdownMenuSeparator />
            <DropdownMenuItem
              onSelect={() => {
                void openCaseCreationPage();
              }}
            >
              <span>New case</span>
            </DropdownMenuItem>
            <DropdownMenuSeparator />
            {visibleCaseItems.map((caseWorkspace) => (
              <ContextMenu key={caseWorkspace.id}>
                <ContextMenuTrigger asChild>
                  <DropdownMenuItem onSelect={() => setActiveCaseId(caseWorkspace.id)}>
                    <span className="truncate">{caseWorkspace.title}</span>
                    {caseWorkspace.id === activeCaseId ? <CheckIcon className="ml-auto size-4" /> : null}
                  </DropdownMenuItem>
                </ContextMenuTrigger>
                <ContextMenuContent>
                  <ContextMenuItem
                    onSelect={() => setRenameTarget({ id: caseWorkspace.id, title: caseWorkspace.title })}
                  >
                    Rename case
                  </ContextMenuItem>
                  <ContextMenuItem variant="destructive" onSelect={() => deleteCase(caseWorkspace.id)}>
                    Delete case
                  </ContextMenuItem>
                </ContextMenuContent>
              </ContextMenu>
            ))}
          </DropdownMenuContent>
        </DropdownMenu>

        <Button
          type="button"
          variant="ghost"
          size="icon-sm"
          className="rounded-full"
          aria-label="Open settings"
          onClick={() => {
            void openGeneralSettingsPage();
          }}
        >
          <SettingsIcon className="size-4" />
        </Button>
      </div>
      <CaseRenameDialog
        caseTitle={renameTarget?.title ?? ""}
        open={!!renameTarget}
        onOpenChange={(open) => {
          if (open) return;
          setRenameTarget(null);
        }}
        onRename={(nextTitle) => {
          if (!renameTarget) return;
          renameCase(renameTarget.id, nextTitle);
        }}
      />
    </>
  );
}

export const AgentShell = CaseShell;
