import {
  ArchiveIcon,
  ChevronDownIcon,
  MoreHorizontal,
  PencilIcon,
  PlusIcon,
  RotateCcwIcon,
  Settings2Icon,
  Trash2Icon,
} from "lucide-react";
import type { AppRuntimeSettings } from "@/lib/chat/agent-types";
import {
  archiveCase,
  deleteCase,
  renameCase,
  reopenCase,
  resolveCase,
  setActiveCaseId,
} from "@/lib/app/runtime";
import { AnonymousLocalCaseLimitDialog } from "@/components/agent/anonymous-local-case-limit-dialog";
import { openCaseCreationPage, openPluginSettingsPage } from "@/lib/app/settings-page";
import { resolveActiveCaseWorkspace } from "@/lib/app/case-shell";
import { MLCProvider } from "@/components/MLCProvider";
import { Thread } from "@/components/assistant-ui/thread";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { trackTelemetryEvent } from "@/lib/telemetry";

type CaseShellProps = {
  settings: AppRuntimeSettings;
};

export function CaseShell({ settings }: CaseShellProps) {
  const visibleCases = settings.cases.filter((item) => item.archivedAt === null);
  const { activeCase } = resolveActiveCaseWorkspace(settings);
  const limitDialog = <AnonymousLocalCaseLimitDialog settings={settings} />;

  if (!activeCase || !visibleCases.length) {
    return (
      <>
        {limitDialog}
        <div className="flex h-full flex-col items-center justify-center gap-4 px-6 text-center">
          <div className="space-y-2">
            <h2 className="text-lg font-semibold text-slate-950">No cases yet</h2>
            <p className="max-w-sm text-sm leading-6 text-slate-500">
              Create a case to start a new TIHC investigation thread.
            </p>
          </div>
          <Button type="button" className="rounded-full" onClick={() => void openCaseCreationPage()}>
            <PlusIcon className="mr-2 size-4" />
            Create case
          </Button>
        </div>
      </>
    );
  }

  const handleRename = () => {
    const nextTitle = globalThis.prompt?.("Rename case", activeCase.title)?.trim();
    if (!nextTitle) return;
    renameCase(activeCase.id, nextTitle);
  };

  const handleDelete = () => {
    const confirmed = globalThis.confirm?.(
      `Delete "${activeCase.title}"? This clears the local case history.`,
    );
    if (!confirmed) return;
    deleteCase(activeCase.id);
    void trackTelemetryEvent("tihc_ext_case_status_changed", {
      context: {
        case_id: activeCase.id,
        status: "deleted",
        surface: "sidepanel",
      },
    });
  };

  const composerToolbar = (
    <>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button
            type="button"
            variant="outline"
            size="sm"
            className="max-w-[12rem] rounded-full"
            aria-label="Case switcher"
          >
            <span className="truncate">{activeCase.title}</span>
            <ChevronDownIcon data-icon="inline-end" />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="start" className="w-64 rounded-2xl">
          <DropdownMenuLabel>Cases</DropdownMenuLabel>
          <DropdownMenuSeparator />
          <DropdownMenuGroup>
            <DropdownMenuRadioGroup
              value={activeCase.id}
              onValueChange={(caseId) => {
                setActiveCaseId(caseId);
                void trackTelemetryEvent("tihc_ext_case_switched", {
                  context: {
                    case_id: caseId,
                    surface: "sidepanel",
                  },
                });
              }}
            >
              {visibleCases.map((item) => (
                <DropdownMenuRadioItem key={item.id} value={item.id}>
                  <span className="truncate">{item.title}</span>
                </DropdownMenuRadioItem>
              ))}
            </DropdownMenuRadioGroup>
          </DropdownMenuGroup>
          <DropdownMenuSeparator />
          <DropdownMenuItem
            onSelect={(event) => {
              event.preventDefault();
              void openCaseCreationPage();
            }}
          >
            <PlusIcon />
            Create case
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>

      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button
            type="button"
            variant="outline"
            size="icon-sm"
            className="rounded-full"
            aria-label="More actions"
          >
            <MoreHorizontal />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="start" className="w-56 rounded-2xl">
          <DropdownMenuLabel>Actions</DropdownMenuLabel>
          <DropdownMenuSeparator />
          <DropdownMenuGroup>
            <DropdownMenuItem
              onSelect={(event) => {
                event.preventDefault();
                void openCaseCreationPage();
              }}
            >
              <PlusIcon />
              Create case
            </DropdownMenuItem>
            <DropdownMenuItem onSelect={handleRename}>
              <PencilIcon />
              Rename
            </DropdownMenuItem>
            {activeCase.activityState === "resolved" ? (
              <DropdownMenuItem
                onSelect={() => {
                  reopenCase(activeCase.id);
                  void trackTelemetryEvent("tihc_ext_case_status_changed", {
                    context: {
                      case_id: activeCase.id,
                      status: "ready",
                      surface: "sidepanel",
                    },
                  });
                }}
              >
                <RotateCcwIcon />
                Reopen
              </DropdownMenuItem>
            ) : (
              <DropdownMenuItem
                onSelect={() => {
                  resolveCase(activeCase.id);
                  void trackTelemetryEvent("tihc_ext_case_status_changed", {
                    context: {
                      case_id: activeCase.id,
                      status: "resolved",
                      surface: "sidepanel",
                    },
                  });
                }}
              >
                <RotateCcwIcon />
                Resolve
              </DropdownMenuItem>
            )}
            <DropdownMenuItem
              onSelect={() => {
                archiveCase(activeCase.id);
                void trackTelemetryEvent("tihc_ext_case_status_changed", {
                  context: {
                    case_id: activeCase.id,
                    status: "archived",
                    surface: "sidepanel",
                  },
                });
              }}
            >
              <ArchiveIcon />
              Archive
            </DropdownMenuItem>
            <DropdownMenuItem onSelect={handleDelete}>
              <Trash2Icon />
              Delete
            </DropdownMenuItem>
            <DropdownMenuItem onSelect={() => void openPluginSettingsPage()}>
              <Settings2Icon />
              Plugin Settings
            </DropdownMenuItem>
          </DropdownMenuGroup>
        </DropdownMenuContent>
      </DropdownMenu>
    </>
  );

  return (
    <>
      {limitDialog}
      <div className="flex h-full min-h-0 flex-col bg-transparent px-3 py-3 text-slate-900">
        <div className="min-w-0 flex flex-1 flex-col overflow-hidden">
          <div className="min-h-0 flex-1 overflow-hidden">
            <MLCProvider
              key={activeCase.id}
              caseWorkspace={activeCase}
            >
              <Thread
                assistantReplyFontSize={settings.assistantReplyFontSize}
                composerToolbar={composerToolbar}
              />
            </MLCProvider>
          </div>
        </div>
      </div>
    </>
  );
}

export const AgentShell = CaseShell;
