import type { CSSProperties } from "react";
import { useEffect, useState, useSyncExternalStore } from "react";
import {
  createCase,
  ensureGoogleAuthSession,
  getAppSettingsSnapshot,
  syncCloudCasesIfNeeded,
  subscribeAppSettings,
  updateAssistantReplyFontSize,
} from "@/lib/app/runtime";
import { getStoredUsageSummary, getStoredUsageTimeseries } from "@/lib/app/cloud-usage";
import { trackTelemetryEvent } from "@/lib/telemetry";
import { AppSidebar } from "@/components/app-sidebar";
import { ChartAreaInteractive } from "@/components/chart-area-interactive";
import { UserLlmSettingsPanel } from "@/components/llm/user-llm-settings-panel";
import { PluginManager } from "@/components/plugin/plugin-manager";
import { SkillsWorkspace } from "@/components/skills/skills-workspace";
import { SectionCards } from "@/components/section-cards";
import { SiteHeader } from "@/components/site-header";
import { TokenUsageCards } from "@/components/token-usage-cards";
import { TokenUsageChart } from "@/components/token-usage-chart";
import { AnonymousLocalCaseLimitDialog } from "@/components/agent/anonymous-local-case-limit-dialog";
import { isAnonymousLocalStorageLimitReached } from "@/lib/app/anonymous-local-case-limit";
import { listDashboardCases, type DashboardCaseRecord } from "@/lib/app/cases-api";
import type { StoredUsageSummaryRecord, StoredUsageTimeseriesPoint } from "@/lib/chat/agent-types";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { SidebarInset, SidebarProvider } from "@/components/ui/sidebar";

const dashboardShellStyle = {
  "--sidebar-width": "calc(var(--spacing) * 72)",
  "--header-height": "calc(var(--spacing) * 12)",
} as CSSProperties;

export type DashboardDialog = "create-case";
export type DashboardSection = "dashboard" | "usage" | "plugin" | "skills" | "llm";

type DashboardShellProps = {
  initialDialog?: DashboardDialog | null;
  initialSection?: DashboardSection | null;
};

function resolveInitialDialog(search: string | undefined): DashboardDialog | null {
  if (!search) return null;
  return new URLSearchParams(search).get("dialog") === "create-case"
    ? "create-case"
    : null;
}

function resolveInitialSection(search: string | undefined): DashboardSection {
  if (!search) return "dashboard";
  const section = new URLSearchParams(search).get("section");
  if (section === "usage" || section === "token-usage") return "usage";
  if (section === "skills") return "skills";
  if (section === "llm") return "llm";
  return section === "plugin" ? "plugin" : "dashboard";
}

function syncSectionQuery(section: DashboardSection) {
  if (typeof window === "undefined") return;

  const url = new URL(window.location.href);
  if (section === "dashboard") {
    url.searchParams.delete("section");
  } else {
    url.searchParams.set("section", section);
  }

  const nextSearch = url.searchParams.toString();
  const nextUrl = `${url.pathname}${nextSearch ? `?${nextSearch}` : ""}${url.hash}`;
  const currentUrl = `${window.location.pathname}${window.location.search}${window.location.hash}`;
  if (nextUrl === currentUrl) return;

  window.history.pushState({}, "", nextUrl);
}

export function DashboardShell({
  initialDialog = null,
  initialSection = null,
}: DashboardShellProps = {}) {
  const appSettings = useSyncExternalStore(
    subscribeAppSettings,
    getAppSettingsSnapshot,
    getAppSettingsSnapshot,
  );
  const resolvedInitialDialog =
    initialDialog ?? resolveInitialDialog(globalThis.location?.search);
  const resolvedInitialSection =
    initialSection ?? resolveInitialSection(globalThis.location?.search);
  const [isCreateCaseOpen, setIsCreateCaseOpen] = useState(
    resolvedInitialDialog === "create-case",
  );
  const [currentSection, setCurrentSection] = useState(resolvedInitialSection);
  const [caseTitleDraft, setCaseTitleDraft] = useState("");
  const [dashboardCases, setDashboardCases] = useState<DashboardCaseRecord[]>([]);
  const [usageSummary, setUsageSummary] = useState<StoredUsageSummaryRecord | null>(null);
  const [usagePoints, setUsagePoints] = useState<StoredUsageTimeseriesPoint[]>([]);
  const isAnonymousLocalCaseLimitBlocked = isAnonymousLocalStorageLimitReached(appSettings);

  useEffect(() => {
    if (typeof window === "undefined") return;

    const handlePopState = () => {
      setCurrentSection(resolveInitialSection(window.location.search));
    };

    window.addEventListener("popstate", handlePopState);
    return () => window.removeEventListener("popstate", handlePopState);
  }, []);

  useEffect(() => {
    void trackTelemetryEvent("tihc_ext_surface_viewed", {
      context: {
        surface: "options",
      },
    });
  }, []);

  useEffect(() => {
    void ensureGoogleAuthSession().finally(() => {
      void syncCloudCasesIfNeeded();
    });
  }, [appSettings.cloudSync.mode, appSettings.googleAuth?.accessToken]);

  useEffect(() => {
    let cancelled = false;
    void listDashboardCases().then((nextCases) => {
      if (cancelled) return;
      setDashboardCases(nextCases);
    });

    return () => {
      cancelled = true;
    };
  }, [appSettings]);

  useEffect(() => {
    let cancelled = false;

    void Promise.all([
      getStoredUsageSummary(appSettings, 30),
      getStoredUsageTimeseries(appSettings, 90),
    ]).then(([nextSummary, nextPoints]) => {
      if (cancelled) return;
      setUsageSummary(nextSummary);
      setUsagePoints(nextPoints);
    });

    return () => {
      cancelled = true;
    };
  }, [appSettings]);

  const closeCreateCaseDialog = () => {
    setIsCreateCaseOpen(false);
    setCaseTitleDraft("");

    if (typeof window === "undefined") return;
    const url = new URL(window.location.href);
    if (!url.searchParams.has("dialog")) return;
    url.searchParams.delete("dialog");
    const nextSearch = url.searchParams.toString();
    window.history.replaceState(
      {},
      "",
      `${url.pathname}${nextSearch ? `?${nextSearch}` : ""}${url.hash}`,
    );
  };

  const handleCreateCase = () => {
    const created = createCase(caseTitleDraft.trim());
    if (!created) return;
    void trackTelemetryEvent("tihc_ext_case_created", {
      context: {
        case_id: created.id,
        surface: "options",
      },
    });
    closeCreateCaseDialog();
  };

  const handleSectionChange = (section: DashboardSection) => {
    setCurrentSection(section);
    syncSectionQuery(section);
  };

  const headerTitle =
    currentSection === "llm"
      ? "LLM"
      : currentSection === "skills"
        ? "Skills"
      : currentSection === "plugin"
        ? "Plugins"
        : currentSection === "usage"
          ? "Usage"
          : "Cases";
  const headerBadgeLabel =
    currentSection === "llm"
      ? "Per-user settings"
      : currentSection === "skills"
        ? "Library"
      : currentSection === "plugin"
        ? "Marketplace"
      : currentSection === "usage"
        ? "Overview"
        : "Overview";

  return (
    <div className="min-h-screen bg-muted/30">
      <SidebarProvider style={dashboardShellStyle}>
        <AppSidebar
          variant="inset"
          currentSection={currentSection}
          assistantReplyFontSize={appSettings.assistantReplyFontSize}
          caseItems={dashboardCases.map((item) => ({
            id: item.id,
            title: item.title,
            status: item.status,
            updatedAt: item.updatedAt,
          }))}
          onAssistantReplyFontSizeChange={updateAssistantReplyFontSize}
          onNavigateSection={handleSectionChange}
          onQuickCreate={() => {
            if (isAnonymousLocalCaseLimitBlocked) return;
            setIsCreateCaseOpen(true);
          }}
        />
        <SidebarInset>
          <SiteHeader title={headerTitle} badgeLabel={headerBadgeLabel} />
          <div className="flex flex-1 flex-col">
            <div className="@container/main flex flex-1 flex-col gap-2">
              {currentSection === "plugin" ? (
                <div className="flex flex-1 flex-col gap-4 px-4 py-4 md:gap-6 md:px-6 md:py-6">
                  <PluginManager />
                </div>
              ) : currentSection === "skills" ? (
                <div className="flex flex-1 flex-col gap-4 px-4 py-4 md:gap-6 md:px-6 md:py-6">
                  <SkillsWorkspace />
                </div>
              ) : currentSection === "llm" ? (
                <div className="flex flex-1 flex-col gap-4 px-4 py-4 md:gap-6 md:px-6 md:py-6">
                  <UserLlmSettingsPanel />
                </div>
              ) : currentSection === "usage" ? (
                <div className="flex flex-col gap-4 py-4 md:gap-6 md:py-6">
                  <TokenUsageCards summary={usageSummary} />
                  <div className="px-4 lg:px-6">
                    <TokenUsageChart points={usagePoints} />
                  </div>
                </div>
              ) : (
                <div className="flex flex-col gap-4 py-4 md:gap-6 md:py-6">
                  <SectionCards cases={dashboardCases} />
                  <div className="px-4 lg:px-6">
                    <ChartAreaInteractive cases={dashboardCases} />
                  </div>
                </div>
              )}
            </div>
          </div>
        </SidebarInset>
      </SidebarProvider>

      <Dialog
        open={isCreateCaseOpen && !isAnonymousLocalCaseLimitBlocked}
        onOpenChange={(open) => {
          if (open) {
            if (isAnonymousLocalCaseLimitBlocked) return;
            setIsCreateCaseOpen(true);
            return;
          }
          closeCreateCaseDialog();
        }}
      >
        <DialogContent
          showCloseButton={false}
          className="max-w-[420px] rounded-[28px] border border-slate-200/90 bg-white p-6 shadow-[0_28px_80px_-48px_rgba(15,23,42,0.38)]"
        >
          <DialogHeader className="space-y-2 text-left">
            <DialogTitle className="tihc-display text-[1.8rem] font-semibold tracking-[-0.045em] text-slate-950">
              Create case
            </DialogTitle>
            <DialogDescription className="text-[14px] leading-6 text-slate-500">
              Create a fresh case workspace and start the investigation.
            </DialogDescription>
          </DialogHeader>

          <form
            className="mt-5 space-y-5"
            onSubmit={(event) => {
              event.preventDefault();
              handleCreateCase();
            }}
          >
            <label className="block space-y-2">
              <span className="text-[11px] font-medium tracking-[0.12em] text-slate-400 uppercase">
                Case title
              </span>
              <Input
                value={caseTitleDraft}
                onChange={(event) => setCaseTitleDraft(event.target.value)}
                placeholder="Ticket 417"
                autoFocus
                className="h-11 rounded-2xl border-slate-200/90 bg-white px-4 text-[15px] text-slate-900 placeholder:text-slate-400"
              />
            </label>

            <DialogFooter className="gap-2">
              <Button
                type="button"
                variant="ghost"
                className="rounded-full px-4 text-slate-500 hover:bg-slate-100 hover:text-slate-900"
                onClick={closeCreateCaseDialog}
              >
                Cancel
              </Button>
              <Button
                type="submit"
                disabled={!caseTitleDraft.trim()}
                className="rounded-full bg-slate-950 px-5 text-white hover:bg-slate-900"
              >
                Start the investigation
              </Button>
            </DialogFooter>
          </form>
        </DialogContent>
      </Dialog>
      <AnonymousLocalCaseLimitDialog settings={appSettings} />
    </div>
  );
}
