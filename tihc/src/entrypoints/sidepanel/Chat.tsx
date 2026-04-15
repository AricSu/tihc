"use client";

import { useEffect, useLayoutEffect, useSyncExternalStore } from "react";
import { CaseShell } from "@/components/agent/agent-shell";
import {
  createCase,
  ensureGoogleAuthSession,
  getAppSettingsSnapshot,
  syncCloudCasesIfNeeded,
  subscribeAppSettings,
} from "@/lib/app/runtime";
import { DEFAULT_CASE_TITLE } from "@/lib/app/runtime-state";
import { trackTelemetryEvent } from "@/lib/telemetry";

export default function Chat() {
  const settings = useSyncExternalStore(
    subscribeAppSettings,
    getAppSettingsSnapshot,
    getAppSettingsSnapshot,
  );
  const hasInstalledPlugins = settings.installedPlugins.length > 0;
  const hasVisibleCases = settings.cases.some((item) => item.archivedAt === null);

  useLayoutEffect(() => {
    if (!hasInstalledPlugins || hasVisibleCases) return;
    createCase(DEFAULT_CASE_TITLE, undefined, { transient: true });
  }, [hasInstalledPlugins, hasVisibleCases]);

  useEffect(() => {
    void trackTelemetryEvent("tihc_ext_surface_viewed", {
      context: {
        surface: "sidepanel",
      },
    });
  }, []);

  useEffect(() => {
    void ensureGoogleAuthSession().finally(() => {
      void syncCloudCasesIfNeeded();
    });
  }, [settings.cloudSync.mode, settings.googleAuth?.accessToken]);

  if (!hasInstalledPlugins) {
    return (
      <div className="flex h-full min-h-0 flex-col">
        <div className="text-muted-foreground flex min-h-0 flex-1 items-center justify-center bg-transparent p-6 text-sm">
          No plugin is configured.
        </div>
      </div>
    );
  }

  if (!hasVisibleCases) {
    return (
      <div className="flex h-full min-h-0 flex-col">
        <div className="min-h-0 flex-1 bg-transparent" />
      </div>
    );
  }

  return (
    <div className="flex h-full min-h-0 flex-col">
      <div className="min-h-0 flex-1">
        <CaseShell settings={settings} />
      </div>
    </div>
  );
}
