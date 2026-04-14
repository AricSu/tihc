"use client";

import { useEffect, useMemo, useState, useSyncExternalStore } from "react";
import {
  getAppSettingsSnapshot,
  subscribeAppSettings,
  updateGlobalLlmRuntime,
} from "@/lib/app/runtime";
import {
  getStoredLlmCredentialStatus,
  listLlmProviders,
  saveStoredLlmCredential,
} from "@/lib/app/cloud-settings";
import type {
  GlobalLlmRuntimeConfig,
  LlmProviderCatalogEntry,
  StoredLlmCredentialStatus,
} from "@/lib/chat/agent-types";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";

const FALLBACK_PROVIDER_CATALOG: LlmProviderCatalogEntry[] = [
  {
    id: "openai",
    label: "OpenAI",
    authMode: "user-api-key",
    configured: false,
    defaultModel: "gpt-4.1-mini",
    models: [
      { id: "gpt-4.1-mini", label: "GPT-4.1 Mini" },
      { id: "gpt-4.1", label: "GPT-4.1" },
    ],
  },
  {
    id: "anthropic",
    label: "Anthropic",
    authMode: "user-api-key",
    configured: false,
    defaultModel: "claude-3-5-sonnet-latest",
    models: [
      { id: "claude-3-5-sonnet-latest", label: "Claude 3.5 Sonnet" },
    ],
  },
  {
    id: "google",
    label: "Google",
    authMode: "user-api-key",
    configured: false,
    defaultModel: "gemini-2.0-flash",
    models: [
      { id: "gemini-2.0-flash", label: "Gemini 2.0 Flash" },
    ],
  },
  {
    id: "xai",
    label: "xAI",
    authMode: "user-api-key",
    configured: false,
    defaultModel: "grok-3-mini-latest",
    models: [{ id: "grok-3-mini-latest", label: "Grok 3 Mini Latest" }],
  },
  {
    id: "openrouter",
    label: "OpenRouter",
    authMode: "user-api-key",
    configured: false,
    defaultModel: "openai/gpt-4.1-mini",
    models: [{ id: "openai/gpt-4.1-mini", label: "OpenAI GPT-4.1 Mini" }],
  },
  {
    id: "mistral",
    label: "Mistral",
    authMode: "user-api-key",
    configured: false,
    defaultModel: "mistral-small-latest",
    models: [{ id: "mistral-small-latest", label: "Mistral Small Latest" }],
  },
  {
    id: "groq",
    label: "Groq",
    authMode: "user-api-key",
    configured: false,
    defaultModel: "llama-3.3-70b-versatile",
    models: [{ id: "llama-3.3-70b-versatile", label: "Llama 3.3 70B Versatile" }],
  },
  {
    id: "deepinfra",
    label: "DeepInfra",
    authMode: "user-api-key",
    configured: false,
    defaultModel: "Qwen/Qwen3-Coder-480B-A35B-Instruct-Turbo",
    models: [
      {
        id: "Qwen/Qwen3-Coder-480B-A35B-Instruct-Turbo",
        label: "Qwen 3 Coder 480B Turbo",
      },
    ],
  },
  {
    id: "cerebras",
    label: "Cerebras",
    authMode: "user-api-key",
    configured: false,
    defaultModel: "gpt-oss-120b",
    models: [{ id: "gpt-oss-120b", label: "GPT OSS 120B" }],
  },
  {
    id: "cohere",
    label: "Cohere",
    authMode: "user-api-key",
    configured: false,
    defaultModel: "command-r7b-12-2024",
    models: [{ id: "command-r7b-12-2024", label: "Command R7B" }],
  },
  {
    id: "togetherai",
    label: "Together AI",
    authMode: "user-api-key",
    configured: false,
    defaultModel: "meta-llama/Llama-3.3-70B-Instruct-Turbo",
    models: [
      {
        id: "meta-llama/Llama-3.3-70B-Instruct-Turbo",
        label: "Llama 3.3 70B Instruct Turbo",
      },
    ],
  },
  {
    id: "perplexity",
    label: "Perplexity",
    authMode: "user-api-key",
    configured: false,
    defaultModel: "sonar",
    models: [{ id: "sonar", label: "Sonar" }],
  },
  {
    id: "alibaba",
    label: "Alibaba",
    authMode: "user-api-key",
    configured: false,
    defaultModel: "qwen3-coder-plus",
    models: [{ id: "qwen3-coder-plus", label: "Qwen 3 Coder Plus" }],
  },
];

function normalizeDraft(value: Partial<GlobalLlmRuntimeConfig> | null | undefined): GlobalLlmRuntimeConfig {
  return {
    baseUrl: value?.baseUrl?.trim() || "",
    providerId: value?.providerId?.trim() || "",
    model: value?.model?.trim() || "",
  };
}

export function UserLlmSettingsPanel() {
  const settings = useSyncExternalStore(
    subscribeAppSettings,
    getAppSettingsSnapshot,
    getAppSettingsSnapshot,
  );
  const [draft, setDraft] = useState<GlobalLlmRuntimeConfig>(() => normalizeDraft(settings.llmRuntime));
  const [providerCatalog, setProviderCatalog] = useState<LlmProviderCatalogEntry[]>([]);
  const [credentialStatus, setCredentialStatus] = useState<StoredLlmCredentialStatus | null>(null);
  const [apiKeyDraft, setApiKeyDraft] = useState("");
  const [saveState, setSaveState] = useState<"idle" | "saving">("idle");
  const requestSettings = useMemo(
    () => ({
      ...settings,
      llmRuntime: {
        ...settings.llmRuntime,
        baseUrl: draft.baseUrl.trim(),
      },
    }),
    [draft.baseUrl, settings],
  );

  useEffect(() => {
    setDraft(normalizeDraft(settings.llmRuntime));
  }, [settings.llmRuntime?.baseUrl, settings.llmRuntime?.model, settings.llmRuntime?.providerId]);

  useEffect(() => {
    let cancelled = false;

    if (!draft.baseUrl.trim()) {
      setProviderCatalog(FALLBACK_PROVIDER_CATALOG);
      return () => {
        cancelled = true;
      };
    }

    void listLlmProviders(requestSettings)
      .then((providers) => {
        if (cancelled) return;
        const nextCatalog = providers.length > 0 ? providers : FALLBACK_PROVIDER_CATALOG;
        setProviderCatalog(nextCatalog);
      })
      .catch(() => {
        if (cancelled) return;
        setProviderCatalog(FALLBACK_PROVIDER_CATALOG);
      });

    return () => {
      cancelled = true;
    };
  }, [draft.baseUrl, requestSettings]);

  useEffect(() => {
    if (!settings.googleAuth?.accessToken?.trim() || !draft.providerId.trim()) {
      setCredentialStatus(null);
      return;
    }

    let cancelled = false;
    void getStoredLlmCredentialStatus(requestSettings, draft.providerId)
      .then((status) => {
        if (cancelled) return;
        setCredentialStatus(status);
      })
      .catch(() => {
        if (cancelled) return;
        setCredentialStatus(null);
      });

    return () => {
      cancelled = true;
    };
  }, [draft.providerId, draft.baseUrl, requestSettings]);

  const selectedProvider = useMemo(
    () => providerCatalog.find((provider) => provider.id === draft.providerId) ?? null,
    [draft.providerId, providerCatalog],
  );
  const selectedProviderModels = selectedProvider?.models ?? [];
  const needsUserApiKey = selectedProvider?.authMode === "user-api-key";
  const canSaveRuntime =
    Boolean(draft.baseUrl.trim() && draft.providerId.trim() && draft.model.trim()) &&
    Boolean(settings.googleAuth?.accessToken?.trim());
  const canSave = canSaveRuntime;

  const updateProvider = (providerId: string) => {
    const provider = providerCatalog.find((item) => item.id === providerId) ?? null;
    setDraft((current) => ({
      ...current,
      providerId,
      model: provider?.defaultModel || provider?.models[0]?.id || "",
    }));
    setApiKeyDraft("");
  };

  const updateModel = (model: string) => {
    setDraft((current) => ({
      ...current,
      model,
    }));
  };

  const save = async () => {
    if (!canSave) return;

    setSaveState("saving");

    if (canSaveRuntime) {
      updateGlobalLlmRuntime(draft);

      if (needsUserApiKey && apiKeyDraft.trim()) {
        const nextCredential = await saveStoredLlmCredential(requestSettings, {
          providerId: draft.providerId,
          apiKey: apiKeyDraft.trim(),
        });
        if (!nextCredential) {
          setSaveState("idle");
          return;
        }
        setCredentialStatus(nextCredential);
        setApiKeyDraft("");
      }
    }

    setSaveState("idle");
  };

  return (
    <div className="flex flex-col gap-6">
      <div className="flex flex-col gap-2">
        <h2 className="text-2xl font-medium">User LLM Settings</h2>
        <p className="text-sm leading-6 text-muted-foreground">
          Manage the provider and model for this signed-in TIHC user. Any provider secret is sent
          directly to the backend and is not persisted in the extension runtime.
        </p>
      </div>

      <div className="grid gap-6">
        <Card>
          <CardHeader>
            <CardTitle>Runtime</CardTitle>
            <CardDescription>
              Choose the provider and model that power user-level case chat.
            </CardDescription>
          </CardHeader>
          <CardContent className="flex flex-col gap-5">
            <label className="flex flex-col gap-2">
              <span className="text-xs font-medium text-muted-foreground">Runtime Backend URL</span>
              <Input
                type="url"
                value={draft.baseUrl}
                onChange={(event) =>
                  setDraft((current) => ({
                    ...current,
                    baseUrl: event.target.value,
                  }))
                }
                placeholder="https://runtime.example.com"
              />
            </label>

            <label className="flex flex-col gap-2">
              <span className="text-xs font-medium text-muted-foreground">Provider</span>
              <select
                className="h-10 rounded-md border border-input bg-background px-3 text-sm"
                disabled={providerCatalog.length === 0}
                value={draft.providerId}
                onChange={(event) => updateProvider(event.target.value)}
              >
                <option value="">Select a provider</option>
                {providerCatalog.map((provider) => (
                  <option key={provider.id} value={provider.id}>
                    {provider.label}
                  </option>
                ))}
              </select>
            </label>

            <label className="flex flex-col gap-2">
              <span className="text-xs font-medium text-muted-foreground">Model</span>
              <select
                className="h-10 rounded-md border border-input bg-background px-3 text-sm"
                disabled={!selectedProviderModels.length}
                value={draft.model}
                onChange={(event) => updateModel(event.target.value)}
              >
                <option value="">Select a model</option>
                {selectedProviderModels.map((model) => (
                  <option key={model.id} value={model.id}>
                    {model.label}
                  </option>
                ))}
              </select>
            </label>

            {needsUserApiKey ? (
              <label className="flex flex-col gap-2">
                <span className="text-xs font-medium text-muted-foreground">Provider API Key</span>
                <Input
                  type="password"
                  value={apiKeyDraft}
                  onChange={(event) => setApiKeyDraft(event.target.value)}
                  placeholder="sk-..."
                />
                <span className="text-sm leading-6 text-muted-foreground">
                  {credentialStatus?.hasSecret
                    ? "A secret is already stored on the backend. Enter a new key only when you want to replace it."
                    : "This provider requires a user-level API key stored on the backend."}
                </span>
              </label>
            ) : null}

            <div className="flex gap-2">
              <Button type="button" onClick={() => void save()} disabled={!canSave || saveState === "saving"}>
                Save LLM Settings
              </Button>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}

export default UserLlmSettingsPanel;
