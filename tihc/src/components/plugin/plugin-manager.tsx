"use client";

import { useEffect, useMemo, useRef, useState, useSyncExternalStore } from "react";
import {
  BotIcon,
  CheckIcon,
  ExternalLinkIcon,
  GlobeIcon,
  LockIcon,
  PlusIcon,
} from "lucide-react";
import {
  getAppSettingsSnapshot,
  subscribeAppSettings,
  updateInstalledPluginConfig,
} from "@/lib/app/runtime";
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
  const hasAuthenticatedAccess = Boolean(settings.googleAuth?.accessToken?.trim());
  const installedPluginIds = settings.installedPlugins.map((plugin) => plugin.pluginId);
  const catalog = listMarketplacePluginCatalog(installedPluginIds);
  const [detailCatalogId, setDetailCatalogId] = useState<string | null>(null);
  const [draftConfig, setDraftConfig] = useState<PluginDraftConfig>(
    settings.installedPlugins[0]?.config ??
      defaultDraftConfigForPlugin(settings.installedPlugins[0]?.pluginId ?? null),
  );
  const settingsSectionRef = useRef<HTMLElement | null>(null);

  const selectedEntry =
    catalog.find((entry) => entry.catalogId === detailCatalogId) ?? null;
  const plugin = selectedEntry?.installedPluginId
    ? settings.installedPlugins.find((item) => item.pluginId === selectedEntry.installedPluginId) ?? null
    : null;
  const manifest = plugin ? getPluginManifest(plugin.pluginId) : null;
  const exposesDetailSettings = plugin?.pluginId === "websearch";

  useEffect(() => {
    setDraftConfig(plugin?.config ?? defaultDraftConfigForPlugin(selectedEntry?.installedPluginId ?? null));
  }, [plugin, selectedEntry?.installedPluginId]);

  useEffect(() => {
    if (!detailCatalogId) return;
    if (hasAuthenticatedAccess || detailCatalogId === "tidb.ai") return;
    setDetailCatalogId(null);
  }, [detailCatalogId, hasAuthenticatedAccess]);

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
    if (plugin?.pluginId !== "websearch") return;
    updateInstalledPluginConfig(plugin.pluginId, {
      enabled: draftConfig.enabled === true,
      mode: draftConfig.mode === "off" ? "off" : "aggressive",
      primaryEngine:
        draftConfig.primaryEngine === "baidu" || draftConfig.primaryEngine === "bing"
          ? draftConfig.primaryEngine
          : "duckduckgo",
    });
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
            const isLoginLocked = !hasAuthenticatedAccess && entry.catalogId !== "tidb.ai";

            return (
              <button
                key={entry.catalogId}
                type="button"
                className={cn(
                  "flex min-h-24 items-center gap-4 rounded-2xl border bg-card px-4 py-4 text-left transition-colors",
                  isLoginLocked
                    ? "cursor-not-allowed border-dashed opacity-55"
                    : "hover:bg-muted/40",
                )}
                disabled={isLoginLocked}
                onClick={() => setDetailCatalogId(entry.catalogId)}
              >
                {renderPluginAvatar(entry.catalogId, entry.title, "card")}

                <div className="min-w-0 flex-1">
                  <div className="flex items-center gap-2">
                    <span className="truncate text-xl font-medium">{entry.title}</span>
                    {isInstalled ? <Badge>Installed</Badge> : null}
                    {isLoginLocked ? <Badge variant="outline">Login required</Badge> : null}
                  </div>
                  <p className="mt-1 line-clamp-2 text-sm leading-6 text-muted-foreground">
                    {entry.summary}
                  </p>
                  {isLoginLocked ? (
                    <p className="mt-2 text-xs leading-5 text-muted-foreground">
                      Sign in to use this plugin.
                    </p>
                  ) : null}
                </div>

                <div className="flex size-11 shrink-0 items-center justify-center rounded-full border bg-muted/40">
                  {isLoginLocked ? (
                    <LockIcon className="size-5" />
                  ) : isInstalled ? (
                    <CheckIcon className="size-5" />
                  ) : (
                    <PlusIcon className="size-5" />
                  )}
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
                {exposesDetailSettings ? (
                  <Button type="button" variant="outline" onClick={focusRuntimeSettings}>
                    Manage Settings
                  </Button>
                ) : null}
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

          {plugin && manifest && exposesDetailSettings ? (
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
                      Web Search injects live context into case chat without changing provider configuration.
                  </CardDescription>
                </CardHeader>
                  <CardContent className="rounded-xl border bg-muted/30 px-4 py-3 text-sm leading-6">
                    Web Search stays independent from provider selection and only influences prompt grounding.
                  </CardContent>
                </Card>
              </div>
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
