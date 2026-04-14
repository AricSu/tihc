"use client";

import { useEffect, useMemo, useRef, useState, useSyncExternalStore } from "react";
import {
  BotIcon,
  CheckIcon,
  ExternalLinkIcon,
  GlobeIcon,
  PlusIcon,
} from "lucide-react";
import {
  clearGoogleAuth,
  getAppSettingsSnapshot,
  refreshGoogleAuth,
  setAnalyticsConsent,
  setGoogleAuth,
  subscribeAppSettings,
  updateInstalledPluginConfig,
} from "@/lib/app/runtime";
import {
  isGoogleOAuthConfigured,
  refreshGoogleAuthSession,
  signInWithGoogle,
  signOutFromGoogle,
} from "@/lib/auth/google-oauth";
import { trackTelemetryEvent } from "@/lib/telemetry";
import { getPluginManifest, listMarketplacePluginCatalog } from "@/lib/plugins/registry";
import type {
  PluginConfigValue,
  TidbAiPluginConfig,
  WebSearchPluginConfig,
} from "@/lib/chat/agent-types";
import { cn } from "@/lib/utils";
import { Avatar, AvatarFallback } from "@/components/ui/avatar";
import { Badge } from "@/components/ui/badge";
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from "@/components/ui/breadcrumb";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Checkbox } from "@/components/ui/checkbox";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Separator } from "@/components/ui/separator";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Table, TableBody, TableCell, TableRow } from "@/components/ui/table";
import { Toggle } from "@/components/ui/toggle";

type PluginDraftConfig = Record<string, PluginConfigValue>;

const DEFAULT_PLUGIN_CONFIG: TidbAiPluginConfig = {
  baseUrl: "",
};

const DEFAULT_WEBSEARCH_CONFIG: WebSearchPluginConfig = {
  enabled: true,
  mode: "aggressive",
  primaryEngine: "duckduckgo",
};

function defaultDraftConfigForPlugin(pluginId: string | null): PluginDraftConfig {
  if (pluginId === "websearch") {
    return DEFAULT_WEBSEARCH_CONFIG;
  }
  return DEFAULT_PLUGIN_CONFIG;
}

function initialsFor(title: string) {
  return title
    .split(/\s+/)
    .slice(0, 2)
    .map((chunk) => chunk[0]?.toUpperCase() ?? "")
    .join("");
}

function pluginVisualSpec(catalogId: string) {
  if (catalogId === "tidb.ai") {
    return {
      icon: BotIcon,
      avatarClassName: "border-slate-900/10 bg-slate-950 text-white shadow-sm",
      fallbackClassName: "bg-transparent text-inherit",
    };
  }

  if (catalogId === "websearch") {
    return {
      icon: GlobeIcon,
      avatarClassName: "border-sky-200 bg-sky-50 text-sky-700 shadow-sm",
      fallbackClassName: "bg-transparent text-inherit",
    };
  }

  return {
    icon: null,
    avatarClassName: "border-slate-200 bg-background text-slate-700",
    fallbackClassName: "bg-transparent text-inherit",
  };
}

function renderPluginAvatar(
  catalogId: string,
  title: string,
  kind: "card" | "detail",
) {
  const spec = pluginVisualSpec(catalogId);
  const Icon = spec.icon;
  const sizeClassName = kind === "detail" ? "size-16" : "size-12";
  const iconClassName = kind === "detail" ? "size-8" : "size-5";
  const dataProps =
    kind === "detail"
      ? { "data-plugin-detail-icon": catalogId }
      : { "data-plugin-icon": catalogId };

  return (
    <Avatar
      {...dataProps}
      aria-hidden="true"
      className={`${sizeClassName} border ${spec.avatarClassName}`}
    >
      <AvatarFallback className={spec.fallbackClassName}>
        {Icon ? <Icon className={iconClassName} /> : initialsFor(title)}
      </AvatarFallback>
    </Avatar>
  );
}

function displayGroupTitle(group: "Featured" | "Coding"): string {
  if (group === "Featured") {
    return "TiHC Native Supported";
  }
  return group;
}

function openExtensionPage(path: string) {
  const browserUrl = (globalThis as typeof globalThis & {
    browser?: {
      runtime?: {
        getURL?: (value: string) => string;
      };
    };
  }).browser?.runtime?.getURL?.(path);

  const chromeUrl = (globalThis as typeof globalThis & {
    chrome?: {
      runtime?: {
        getURL?: (value: string) => string;
      };
    };
  }).chrome?.runtime?.getURL?.(path);

  const targetUrl = browserUrl ?? chromeUrl ?? `/${path}`;
  window.open(targetUrl, "_blank", "noopener,noreferrer");
}

export function PluginManager() {
  const settings = useSyncExternalStore(
    subscribeAppSettings,
    getAppSettingsSnapshot,
    getAppSettingsSnapshot,
  );
  const installedPluginIds = settings.installedPlugins.map((plugin) => plugin.pluginId);
  const catalog = listMarketplacePluginCatalog(installedPluginIds).filter((entry) =>
    settings.googleAuth?.accessToken?.trim() ? true : entry.catalogId === "tidb.ai",
  );
  const [detailCatalogId, setDetailCatalogId] = useState<string | null>(null);
  const [draftConfig, setDraftConfig] = useState<PluginDraftConfig>(
    settings.installedPlugins[0]?.config ??
      defaultDraftConfigForPlugin(settings.installedPlugins[0]?.pluginId ?? null),
  );
  const [authState, setAuthState] = useState<{
    status: "idle" | "running" | "success" | "error";
    message: string;
  }>({
    status: "idle",
    message: "Sign in once and the tidb.ai plugin will send the Google bearer token automatically.",
  });
  const oauthConfigured = isGoogleOAuthConfigured();
  const settingsSectionRef = useRef<HTMLElement | null>(null);

  const selectedEntry =
    catalog.find((entry) => entry.catalogId === detailCatalogId) ?? null;
  const plugin = selectedEntry?.installedPluginId
    ? settings.installedPlugins.find((item) => item.pluginId === selectedEntry.installedPluginId) ?? null
    : null;
  const manifest = plugin ? getPluginManifest(plugin.pluginId) : null;

  useEffect(() => {
    setDraftConfig(plugin?.config ?? defaultDraftConfigForPlugin(selectedEntry?.installedPluginId ?? null));
  }, [plugin, selectedEntry?.installedPluginId]);

  const groupedCatalog = useMemo(
    () => ({
      Featured: catalog.filter((entry) => entry.group === "Featured"),
      Coding: catalog.filter((entry) => entry.group === "Coding"),
    }),
    [catalog],
  );
  const visibleCatalogSections = useMemo(
    () =>
      (Object.entries(groupedCatalog) as Array<["Featured" | "Coding", typeof catalog]>).filter(
        ([, entries]) => entries.length > 0,
      ),
    [groupedCatalog, catalog],
  );
  const save = () => {
    if (!plugin) return;
    if (plugin.pluginId === "tidb.ai") {
      updateInstalledPluginConfig(plugin.pluginId, {
        baseUrl: String(draftConfig.baseUrl ?? "").trim(),
      });
    } else {
      updateInstalledPluginConfig(plugin.pluginId, {
        enabled: draftConfig.enabled === true,
        mode: draftConfig.mode === "off" ? "off" : "aggressive",
        primaryEngine:
          draftConfig.primaryEngine === "baidu" || draftConfig.primaryEngine === "bing"
            ? draftConfig.primaryEngine
            : "duckduckgo",
      });
    }
    void trackTelemetryEvent("tihc_ext_plugin_settings_saved", {
      context: {
        plugin_id: plugin.pluginId,
        surface: "options",
      },
    });
  };

  const focusRuntimeSettings = () => {
    settingsSectionRef.current?.scrollIntoView({ behavior: "smooth", block: "start" });
  };

  const updateDraftField = (key: string, value: PluginConfigValue) => {
    setDraftConfig((current) => ({
      ...current,
      [key]: value,
    }));
  };

  const handleGoogleSignIn = async () => {
    if (!oauthConfigured) {
      setAuthState({
        status: "error",
        message:
          "Google OAuth is not configured. Set WXT_GOOGLE_OAUTH_CLIENT_ID or a browser-specific override.",
      });
      return;
    }

    setAuthState({
      status: "running",
      message: "Opening the Google sign-in flow...",
    });

    try {
      const googleAuth = await signInWithGoogle();
      setGoogleAuth(googleAuth);
      setAuthState({
        status: "success",
        message: `Signed in as ${googleAuth.email || "your Google account"}.`,
      });
    } catch (error) {
      setAuthState({
        status: "error",
        message: error instanceof Error && error.message ? error.message : "Google sign-in failed.",
      });
    }
  };

  const handleGoogleRefresh = async () => {
    setAuthState({
      status: "running",
      message: "Refreshing the Google bearer token...",
    });

    try {
      const googleAuth = await refreshGoogleAuthSession();
      refreshGoogleAuth(googleAuth);
      setAuthState({
        status: "success",
        message: `Refreshed Google auth for ${googleAuth.email || "your account"}.`,
      });
    } catch (error) {
      setAuthState({
        status: "error",
        message: error instanceof Error && error.message ? error.message : "Google token refresh failed.",
      });
    }
  };

  const handleGoogleSignOut = async () => {
    const accessToken = settings.googleAuth?.accessToken ?? "";
    setAuthState({
      status: "running",
      message: "Signing out and revoking the current bearer token...",
    });

    try {
      if (accessToken) {
        await signOutFromGoogle(accessToken);
      }
    } catch (error) {
      setAuthState({
        status: "error",
        message: error instanceof Error && error.message ? error.message : "Google sign-out failed.",
      });
    } finally {
      clearGoogleAuth();
      setAuthState({
        status: "success",
        message: "Signed out. Future tidb.ai requests will no longer include Google auth.",
      });
    }
  };

  const renderConfigField = (field: NonNullable<typeof manifest>["settingsFields"][number]) => {
    if (!plugin) return null;

    if (field.type === "checkbox") {
      return (
        <div key={field.key} className="flex items-start gap-3 rounded-xl border border-slate-200/80 px-4 py-3">
          <Checkbox
            id={`plugin-setting-${field.key}`}
            checked={draftConfig[field.key] === true}
            onCheckedChange={(checked) => updateDraftField(field.key, checked === true)}
          />
          <div className="space-y-1">
            <Label htmlFor={`plugin-setting-${field.key}`}>{field.label}</Label>
            {field.description ? (
              <p className="text-sm leading-6 text-muted-foreground">{field.description}</p>
            ) : null}
          </div>
        </div>
      );
    }

    if (field.type === "select") {
      return (
        <label key={field.key} className="flex flex-col gap-2">
          <span className="text-xs font-medium text-muted-foreground">{field.label}</span>
          <Select
            value={String(draftConfig[field.key])}
            onValueChange={(value) => updateDraftField(field.key, value)}
          >
            <SelectTrigger className="w-full">
              <SelectValue placeholder={field.placeholder ?? field.label} />
            </SelectTrigger>
            <SelectContent>
              {(field.options ?? []).map((option) => (
                <SelectItem key={option.value} value={option.value}>
                  {option.label}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
          {field.description ? (
            <span className="text-sm leading-6 text-muted-foreground">{field.description}</span>
          ) : null}
        </label>
      );
    }

    return (
      <label key={field.key} className="flex flex-col gap-2">
        <span className="text-xs font-medium text-muted-foreground">{field.label}</span>
        <Input
          value={String(draftConfig[field.key] ?? "")}
          onChange={(event) => updateDraftField(field.key, event.target.value)}
          placeholder={field.placeholder}
        />
        {field.description ? (
          <span className="text-sm leading-6 text-muted-foreground">{field.description}</span>
        ) : null}
      </label>
    );
  };

  const renderCatalogSection = (title: "Featured" | "Coding") => {
    const entries = groupedCatalog[title];
    if (!entries.length) return null;

    return (
      <section key={title} className="flex flex-col gap-4">
        <div className="flex items-center gap-3">
          <h2 className="text-lg font-medium">{displayGroupTitle(title)}</h2>
          <Separator className="flex-1" />
        </div>

        <div className="grid gap-3 xl:grid-cols-2">
          {entries.map((entry) => {
            const isInstalled = entry.status === "installed";

            return (
              <button
                key={entry.catalogId}
                type="button"
                className="flex min-h-24 items-center gap-4 rounded-2xl border bg-card px-4 py-4 text-left transition-colors hover:bg-muted/40"
                onClick={() => setDetailCatalogId(entry.catalogId)}
              >
                {renderPluginAvatar(entry.catalogId, entry.title, "card")}

                <div className="min-w-0 flex-1">
                  <div className="flex items-center gap-2">
                    <span className="truncate text-xl font-medium">{entry.title}</span>
                    {isInstalled ? <Badge>Installed</Badge> : null}
                  </div>
                  <p className="mt-1 line-clamp-2 text-sm leading-6 text-muted-foreground">
                    {entry.summary}
                  </p>
                </div>

                <div className="flex size-11 shrink-0 items-center justify-center rounded-full border bg-muted/40">
                  {isInstalled ? <CheckIcon className="size-5" /> : <PlusIcon className="size-5" />}
                </div>
              </button>
            );
          })}
        </div>
      </section>
    );
  };

  if (selectedEntry) {
    return (
      <div className="flex flex-col gap-6">
        <div className="flex items-start justify-between gap-4">
          <div className="flex flex-col gap-3">
            <p className="text-sm font-medium tracking-tight text-muted-foreground">Plugins</p>

            <Breadcrumb>
              <BreadcrumbList>
                <BreadcrumbItem>
                  <BreadcrumbLink asChild>
                    <button type="button" onClick={() => setDetailCatalogId(null)}>
                      Plugins
                    </button>
                  </BreadcrumbLink>
                </BreadcrumbItem>
                <BreadcrumbSeparator />
                <BreadcrumbItem>
                  <BreadcrumbPage>{selectedEntry.title}</BreadcrumbPage>
                </BreadcrumbItem>
              </BreadcrumbList>
            </Breadcrumb>
          </div>

          <div className="flex flex-wrap gap-2">
            {plugin ? (
              <>
                <Button type="button" variant="outline" onClick={focusRuntimeSettings}>
                  Manage Settings
                </Button>
                <Button type="button" onClick={() => openExtensionPage("sidepanel.html")}>
                  Try in case
                </Button>
              </>
            ) : (
              <Button type="button" disabled>
                Coming Soon
              </Button>
            )}
          </div>
        </div>

        <div className="mx-auto flex w-full max-w-5xl flex-col gap-6">
          <div className="flex items-start gap-4">
            {renderPluginAvatar(selectedEntry.catalogId, selectedEntry.title, "detail")}
            <div className="flex flex-col gap-3">
              <div className="flex flex-wrap items-center gap-2">
                <Badge variant="outline">Plugin Marketplace</Badge>
                <Badge variant={plugin ? "secondary" : "outline"}>
                  {plugin ? "Installed" : "Coming Soon"}
                </Badge>
              </div>
              <h1 className="text-4xl font-semibold tracking-tight">{selectedEntry.title}</h1>
              <p className="text-xl text-muted-foreground">{selectedEntry.summary}</p>
            </div>
          </div>

          <Card className="overflow-hidden">
            <CardContent className="p-4 sm:p-6">
              <div className="rounded-[2rem] border bg-gradient-to-br from-muted/90 via-background to-muted/70 px-4 py-8 sm:px-8">
                <div className="mx-auto flex max-w-2xl flex-col gap-3 rounded-2xl border bg-background/95 px-5 py-4 shadow-sm backdrop-blur">
                  <div className="flex items-center gap-2 text-sm text-muted-foreground">
                    <Badge variant="secondary">{selectedEntry.title}</Badge>
                    <span>{selectedEntry.provider}</span>
                  </div>
                  <p className="text-lg leading-8">{selectedEntry.heroPrompt}</p>
                </div>
              </div>
            </CardContent>
          </Card>

          <p className="max-w-4xl text-base leading-8 text-muted-foreground">{selectedEntry.description}</p>

          <section className="flex flex-col gap-4">
            <h2 className="text-2xl font-medium">Includes</h2>
            <Card>
              <CardContent className="p-0">
                {selectedEntry.includes.map((item, index) => (
                  <div key={item.name}>
                    {index > 0 ? <Separator /> : null}
                    <div className="flex items-center gap-4 px-5 py-5">
                      <Avatar className="size-11 border">
                        <AvatarFallback>{initialsFor(item.name)}</AvatarFallback>
                      </Avatar>

                      <div className="min-w-0 flex-1">
                        <div className="flex items-center gap-2">
                          <span className="truncate text-base font-medium">{item.name}</span>
                          <span className="text-sm text-muted-foreground">{item.type}</span>
                        </div>
                        <p className="mt-1 text-sm text-muted-foreground">{item.description}</p>
                      </div>

                      <Toggle
                        pressed={item.enabled}
                        disabled
                        variant="outline"
                        size="sm"
                        className="min-w-16 rounded-full"
                        aria-label={`${item.name} enabled`}
                      >
                        {item.enabled ? "On" : "Off"}
                      </Toggle>
                    </div>
                  </div>
                ))}
              </CardContent>
            </Card>
          </section>

          <section className="flex flex-col gap-4">
            <h2 className="text-2xl font-medium">Information</h2>
            <Card>
              <CardContent className="p-0">
                <Table>
                  <TableBody>
                    {selectedEntry.information.map((item) => (
                      <TableRow key={item.label}>
                        <TableCell className="w-full max-w-52 px-5 py-5 text-base text-muted-foreground md:w-52">
                          {item.label}
                        </TableCell>
                        <TableCell className="px-5 py-5 text-base">
                          {item.href ? (
                            <a
                              className="inline-flex items-center gap-1.5 underline underline-offset-4"
                              href={item.href}
                              target="_blank"
                              rel="noreferrer"
                            >
                              <span>{item.value}</span>
                              <ExternalLinkIcon className="size-4" />
                            </a>
                          ) : (
                            item.value
                          )}
                        </TableCell>
                      </TableRow>
                    ))}
                  </TableBody>
                </Table>
              </CardContent>
            </Card>
          </section>

          {plugin && manifest ? (
            <section ref={settingsSectionRef} className="flex flex-col gap-6">
              {plugin.pluginId === "websearch" ? (
                <Card>
                  <CardHeader>
                    <CardTitle>How It Works</CardTitle>
                    <CardDescription>
                      Runs automatically before case chats and only injects search context into the outbound request.
                    </CardDescription>
                  </CardHeader>
                  <CardContent className="grid gap-3 sm:grid-cols-2">
                    <div className="rounded-xl border bg-muted/30 px-4 py-3 text-sm leading-6">
                      Runs automatically before case chats when Web Search is enabled.
                    </div>
                    <div className="rounded-xl border bg-muted/30 px-4 py-3 text-sm leading-6">
                      No separate remote connection test is required because search runs inside the extension.
                    </div>
                  </CardContent>
                </Card>
              ) : null}

              <div className="flex flex-col gap-2">
                <h2 className="text-2xl font-medium">Runtime Settings</h2>
                <p className="text-sm leading-6 text-muted-foreground">
                  {plugin.pluginId === "websearch"
                    ? "Manage Web Search independently from case chat without leaving the marketplace detail page."
                    : `Manage ${manifest.label} independently from user-level LLM settings without leaving the marketplace detail page.`}
                </p>
              </div>

              <div className="grid gap-6 lg:grid-cols-[minmax(0,1fr)_320px]">
                <Card>
                  <CardHeader>
                    <CardTitle>Configuration</CardTitle>
                    <CardDescription>
                      Update the installed {manifest.label} plugin without touching user-level LLM settings.
                    </CardDescription>
                  </CardHeader>
                  <CardContent className="flex flex-col gap-5">
                    {manifest.settingsFields.map((field) => renderConfigField(field))}

                    <div className="flex gap-2">
                      <Button type="button" onClick={save}>
                        Save Plugin Settings
                      </Button>
                    </div>
                  </CardContent>
                </Card>

                <Card>
                  <CardHeader>
                    <CardTitle>Plugin Notes</CardTitle>
                    <CardDescription>
                      {plugin.pluginId === "tidb.ai"
                        ? "tidb.ai is modeled as a shared MCP client and no longer owns user-level LLM settings."
                        : "Web Search injects live context into case chat without changing provider configuration."}
                  </CardDescription>
                </CardHeader>
                  <CardContent className="rounded-xl border bg-muted/30 px-4 py-3 text-sm leading-6">
                    {plugin.pluginId === "tidb.ai"
                      ? "Keep this plugin focused on the shared backend endpoint and Google auth. Use the dedicated LLM workspace for provider and model changes."
                      : "Web Search stays independent from provider selection and only influences prompt grounding."}
                  </CardContent>
                </Card>
              </div>

              {plugin.pluginId === "tidb.ai" ? (
                <Card>
                  <CardHeader>
                    <CardTitle>Google Workspace Auth</CardTitle>
                    <CardDescription>
                      Sign in once so TIHC can attach your Google bearer token to backend requests when workspace auth is enabled.
                    </CardDescription>
                  </CardHeader>
                  <CardContent className="flex flex-col gap-4">
                    <div className="rounded-xl border bg-muted/30 px-4 py-3 text-sm leading-6">
                      {settings.googleAuth ? (
                        <>
                          <div>Signed in as {settings.googleAuth.email || "your Google account"}.</div>
                          <div>Hosted domain: {settings.googleAuth.hostedDomain || "Unknown"}.</div>
                          <div>Token expiry: {settings.googleAuth.expiresAt ?? "Unknown"}.</div>
                        </>
                      ) : (
                        <div>No Google session is active.</div>
                      )}
                    </div>
                    <div className="rounded-xl border bg-muted/30 px-4 py-3 text-sm leading-6">
                      {authState.message}
                    </div>
                    <div className="flex flex-wrap gap-2">
                      {settings.googleAuth ? (
                        <>
                          <Button
                            type="button"
                            variant="outline"
                            disabled={authState.status === "running"}
                            onClick={() => void handleGoogleRefresh()}
                          >
                            Refresh Google Token
                          </Button>
                          <Button
                            type="button"
                            variant="outline"
                            disabled={authState.status === "running"}
                            onClick={() => void handleGoogleSignOut()}
                          >
                            Sign out
                          </Button>
                        </>
                      ) : (
                        <Button
                          type="button"
                          disabled={authState.status === "running"}
                          onClick={() => void handleGoogleSignIn()}
                        >
                          Sign in with Google
                        </Button>
                      )}
                    </div>
                  </CardContent>
                </Card>
              ) : null}

              <Card>
                <CardHeader>
                  <CardTitle>Usage Analytics</CardTitle>
                  <CardDescription>
                    Control whether TIHC can send extension usage analytics to the shared GA4 property.
                  </CardDescription>
                </CardHeader>
                <CardContent className="flex flex-col gap-4">
                  <div className="rounded-xl border bg-muted/30 px-4 py-3 text-sm leading-6">
                    {settings.analyticsConsent === "granted"
                      ? "Analytics is enabled for this extension."
                      : settings.analyticsConsent === "denied"
                        ? "Analytics is disabled for this extension."
                        : "Analytics has not been configured for this extension yet."}
                  </div>
                  <div className="flex flex-wrap gap-2">
                    {settings.analyticsConsent !== "granted" ? (
                      <Button
                        type="button"
                        onClick={() => {
                          setAnalyticsConsent("granted");
                          void trackTelemetryEvent("tihc_ext_consent_updated", {
                            context: {
                              status: "granted",
                              surface: "options",
                            },
                          });
                        }}
                      >
                        Allow analytics
                      </Button>
                    ) : null}
                    {settings.analyticsConsent !== "denied" ? (
                      <Button type="button" variant="outline" onClick={() => setAnalyticsConsent("denied")}>
                        Disable analytics
                      </Button>
                    ) : null}
                  </div>
                </CardContent>
              </Card>
            </section>
          ) : null}
        </div>
      </div>
    );
  }

  return (
    <div className="mx-auto flex w-full max-w-5xl flex-col gap-8">
        {visibleCatalogSections.map(([title]) => renderCatalogSection(title))}
    </div>
  );
}
