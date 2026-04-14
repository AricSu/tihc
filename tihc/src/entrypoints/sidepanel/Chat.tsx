"use client";

import { useEffect, useLayoutEffect, useSyncExternalStore } from "react";
import { CaseShell } from "@/components/agent/agent-shell";
import {
  createCase,
  ensureGoogleAuthSession,
  getAppSettingsSnapshot,
  setAnalyticsConsent,
  syncCloudCasesIfNeeded,
  subscribeAppSettings,
} from "@/lib/app/runtime";
import { trackTelemetryEvent } from "@/lib/telemetry";
import { Button } from "@/components/ui/button";

export default function Chat() {
  const settings = useSyncExternalStore(
    subscribeAppSettings,
    getAppSettingsSnapshot,
    getAppSettingsSnapshot,
  );
  const hasInstalledPlugins = settings.installedPlugins.length > 0;
  const hasVisibleCases = settings.cases.some((item) => item.archivedAt === null);
  const showAnalyticsPrompt = settings.analyticsConsent === "unknown";

  useLayoutEffect(() => {
    if (!hasInstalledPlugins || hasVisibleCases) return;
    createCase("");
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
      <div className="flex h-full items-center justify-center bg-transparent p-6 text-sm text-slate-500">
        No plugin is configured.
      </div>
    );
  }

  if (!hasVisibleCases) {
    return <div className="h-full bg-transparent" />;
  }

  return (
    <div className="relative h-full">
      <CaseShell settings={settings} />
      {showAnalyticsPrompt ? (
        <div className="absolute inset-x-3 bottom-3 z-50">
          <div className="rounded-[24px] border border-slate-200/90 bg-white/96 p-4 shadow-[0_20px_70px_-40px_rgba(15,23,42,0.55)] backdrop-blur">
            <div className="space-y-2">
              <h2 className="text-sm font-semibold text-slate-950">Help improve TIHC</h2>
              <p className="text-sm leading-6 text-slate-600">
                Allow analytics so TIHC can measure extension usage and attribute signed-in sessions when available.
              </p>
            </div>
            <div className="mt-4 flex gap-2">
              <Button
                type="button"
                size="sm"
                onClick={() => {
                  setAnalyticsConsent("granted");
                  void trackTelemetryEvent("tihc_ext_consent_updated", {
                    context: {
                      status: "granted",
                      surface: "sidepanel",
                    },
                  });
                }}
              >
                Allow analytics
              </Button>
              <Button
                type="button"
                size="sm"
                variant="outline"
                onClick={() => {
                  setAnalyticsConsent("denied");
                }}
              >
                No thanks
              </Button>
            </div>
          </div>
        </div>
      ) : null}
    </div>
  );
}
