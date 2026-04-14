import { createAlibaba } from "@ai-sdk/alibaba";
import { createAnthropic } from "@ai-sdk/anthropic";
import { createCerebras } from "@ai-sdk/cerebras";
import { createCohere } from "@ai-sdk/cohere";
import { createDeepInfra } from "@ai-sdk/deepinfra";
import { createGoogleGenerativeAI } from "@ai-sdk/google";
import { createGroq } from "@ai-sdk/groq";
import { createMistral } from "@ai-sdk/mistral";
import { createOpenAI } from "@ai-sdk/openai";
import { createPerplexity } from "@ai-sdk/perplexity";
import { createTogetherAI } from "@ai-sdk/togetherai";
import { createXai } from "@ai-sdk/xai";
import type { LanguageModelV3 } from "@ai-sdk/provider";
import type { FetchFunction } from "@ai-sdk/provider-utils";
import { createOpenRouter } from "@openrouter/ai-sdk-provider";
import { resolveEnvValue, type AppEnv } from "../../shared/support";
import type { LlmProviderCatalogEntry } from "./chat-types";
import { loadModelsDevProviders, type ModelsDevModel } from "./models-dev";

type CatalogAuthMode = Exclude<LlmProviderCatalogEntry["authMode"], "codex-oauth">;

type CatalogModel = {
  id: string;
  label: string;
};

export const HOSTED_PROVIDER_IDS = [
  "openai",
  "anthropic",
  "google",
  "xai",
  "openrouter",
  "mistral",
  "groq",
  "deepinfra",
  "cerebras",
  "cohere",
  "togetherai",
  "perplexity",
  "alibaba",
] as const;
export type HostedProviderId = (typeof HOSTED_PROVIDER_IDS)[number];

const NON_CHAT_MODEL_FAMILY_PATTERN =
  /(embedding|transcription|speech|audio-generation|text-to-speech|moderation|rerank)/i;
const NON_CHAT_MODEL_ID_PATTERN =
  /(embedding|transcription|whisper|tts|moderation|rerank)/i;

type LanguageModelFactoryInput = {
  apiKey?: string;
  env: AppEnv;
  fetchImpl?: typeof fetch;
  modelId: string;
};

type HostedProviderCatalogState = {
  authMode: CatalogAuthMode;
  configured: boolean;
};

type HostedProviderRequestResolution =
  | {
      ok: true;
      apiKey?: string;
    }
  | {
      ok: false;
      errorMessage: string;
    };

type HostedProviderDefinition = {
  id: HostedProviderId;
  label: string;
  envKey?: keyof AppEnv;
  defaultModelHints: readonly string[];
  fallbackModels: readonly CatalogModel[];
  resolveCatalogState?(env: AppEnv): HostedProviderCatalogState;
  resolveRequest?(env: AppEnv, apiKeyOverride?: string | null): HostedProviderRequestResolution;
  resolveLogUrl?(env: AppEnv): string | undefined;
  buildLanguageModel(input: LanguageModelFactoryInput): LanguageModelV3;
};

function resolveFetchOption(fetchImpl?: typeof fetch): { fetch?: FetchFunction } {
  if (!fetchImpl) return {};
  return {
    fetch: fetchImpl as FetchFunction,
  };
}

function envValuePresent(env: AppEnv, key: keyof AppEnv): boolean {
  return Boolean(resolveEnvValue(env, key));
}

function normalizeSecret(value: string | null | undefined): string | undefined {
  const trimmed = value?.trim();
  return trimmed ? trimmed : undefined;
}

function resolveDefaultCatalogState(
  definition: HostedProviderDefinition,
  env: AppEnv,
): HostedProviderCatalogState {
  if (!definition.envKey) {
    return {
      authMode: "backend-managed",
      configured: true,
    };
  }

  const configured = envValuePresent(env, definition.envKey);
  return {
    authMode: configured ? "backend-managed" : "user-api-key",
    configured,
  };
}

function resolveDefaultRequest(
  definition: HostedProviderDefinition,
  env: AppEnv,
  apiKeyOverride?: string | null,
): HostedProviderRequestResolution {
  const apiKey = normalizeSecret(apiKeyOverride)
    ?? (definition.envKey ? normalizeSecret(resolveEnvValue(env, definition.envKey)) : undefined);

  if (apiKey) {
    return {
      ok: true,
      apiKey,
    };
  }

  return {
    ok: false,
    errorMessage: definition.envKey ? `Missing ${definition.envKey}` : "Missing provider credentials",
  };
}

const HOSTED_PROVIDER_DEFINITIONS: readonly HostedProviderDefinition[] = [
  {
    id: "openai",
    label: "OpenAI",
    envKey: "OPENAI_API_KEY",
    defaultModelHints: ["gpt-4.1-mini", "gpt-5-mini", "gpt-4.1", "gpt-4o-mini"],
    fallbackModels: [
      { id: "gpt-4.1-mini", label: "GPT-4.1 Mini" },
      { id: "gpt-4.1", label: "GPT-4.1" },
      { id: "gpt-5-mini", label: "GPT-5 Mini" },
    ],
    resolveLogUrl(env) {
      return normalizeSecret(resolveEnvValue(env, "OPENAI_API_URL"));
    },
    buildLanguageModel({ apiKey, env, fetchImpl, modelId }) {
      const baseURL = normalizeSecret(resolveEnvValue(env, "OPENAI_API_URL"));
      const provider = createOpenAI({
        apiKey,
        ...(baseURL ? { baseURL } : {}),
        ...resolveFetchOption(fetchImpl),
      });
      return provider.chat(modelId);
    },
  },
  {
    id: "anthropic",
    label: "Anthropic",
    envKey: "ANTHROPIC_API_KEY",
    defaultModelHints: [
      "claude-3-5-sonnet-latest",
      "claude-sonnet-4-5",
      "claude-3-7-sonnet-latest",
    ],
    fallbackModels: [
      { id: "claude-3-5-sonnet-latest", label: "Claude 3.5 Sonnet" },
      { id: "claude-3-5-haiku-20241022", label: "Claude 3.5 Haiku" },
    ],
    buildLanguageModel({ apiKey, fetchImpl, modelId }) {
      return createAnthropic({
        apiKey,
        ...resolveFetchOption(fetchImpl),
      }).languageModel(modelId);
    },
  },
  {
    id: "google",
    label: "Google",
    envKey: "GOOGLE_API_KEY",
    defaultModelHints: ["gemini-2.0-flash", "gemini-2.5-flash", "gemini-flash-latest"],
    fallbackModels: [
      { id: "gemini-2.0-flash", label: "Gemini 2.0 Flash" },
      { id: "gemini-2.5-flash", label: "Gemini 2.5 Flash" },
    ],
    buildLanguageModel({ apiKey, fetchImpl, modelId }) {
      return createGoogleGenerativeAI({
        apiKey,
        ...resolveFetchOption(fetchImpl),
      }).languageModel(modelId);
    },
  },
  {
    id: "xai",
    label: "xAI",
    envKey: "XAI_API_KEY",
    defaultModelHints: ["grok-3-mini-latest", "grok-3-fast", "grok-4-latest"],
    fallbackModels: [
      { id: "grok-3-mini-latest", label: "Grok 3 Mini Latest" },
      { id: "grok-4-latest", label: "Grok 4 Latest" },
    ],
    buildLanguageModel({ apiKey, fetchImpl, modelId }) {
      return createXai({
        apiKey,
        ...resolveFetchOption(fetchImpl),
      }).languageModel(modelId);
    },
  },
  {
    id: "openrouter",
    label: "OpenRouter",
    envKey: "OPENROUTER_API_KEY",
    defaultModelHints: [
      "openai/gpt-4.1-mini",
      "anthropic/claude-3.7-sonnet",
      "google/gemini-2.5-flash",
    ],
    fallbackModels: [
      { id: "openai/gpt-4.1-mini", label: "OpenAI GPT-4.1 Mini" },
      { id: "anthropic/claude-3.7-sonnet", label: "Anthropic Claude 3.7 Sonnet" },
      { id: "google/gemini-2.5-flash", label: "Google Gemini 2.5 Flash" },
    ],
    resolveLogUrl() {
      return "https://openrouter.ai/api/v1";
    },
    buildLanguageModel({ apiKey, fetchImpl, modelId }) {
      return createOpenRouter({
        apiKey,
        ...resolveFetchOption(fetchImpl),
      }).languageModel(modelId);
    },
  },
  {
    id: "mistral",
    label: "Mistral",
    envKey: "MISTRAL_API_KEY",
    defaultModelHints: [
      "mistral-small-latest",
      "mistral-medium-latest",
      "mistral-large-latest",
    ],
    fallbackModels: [
      { id: "mistral-small-latest", label: "Mistral Small Latest" },
      { id: "mistral-medium-latest", label: "Mistral Medium Latest" },
    ],
    buildLanguageModel({ apiKey, fetchImpl, modelId }) {
      return createMistral({
        apiKey,
        ...resolveFetchOption(fetchImpl),
      }).languageModel(modelId);
    },
  },
  {
    id: "groq",
    label: "Groq",
    envKey: "GROQ_API_KEY",
    defaultModelHints: [
      "llama-3.3-70b-versatile",
      "moonshotai/kimi-k2-instruct-0905",
      "qwen/qwen3-32b",
    ],
    fallbackModels: [
      { id: "llama-3.3-70b-versatile", label: "Llama 3.3 70B Versatile" },
      { id: "qwen/qwen3-32b", label: "Qwen 3 32B" },
    ],
    buildLanguageModel({ apiKey, fetchImpl, modelId }) {
      return createGroq({
        apiKey,
        ...resolveFetchOption(fetchImpl),
      }).languageModel(modelId);
    },
  },
  {
    id: "deepinfra",
    label: "DeepInfra",
    envKey: "DEEPINFRA_API_KEY",
    defaultModelHints: [
      "Qwen/Qwen3-Coder-480B-A35B-Instruct-Turbo",
      "meta-llama/Llama-3.3-70B-Instruct-Turbo",
    ],
    fallbackModels: [
      {
        id: "Qwen/Qwen3-Coder-480B-A35B-Instruct-Turbo",
        label: "Qwen 3 Coder 480B Turbo",
      },
    ],
    buildLanguageModel({ apiKey, fetchImpl, modelId }) {
      return createDeepInfra({
        apiKey,
        ...resolveFetchOption(fetchImpl),
      }).languageModel(modelId);
    },
  },
  {
    id: "cerebras",
    label: "Cerebras",
    envKey: "CEREBRAS_API_KEY",
    defaultModelHints: ["gpt-oss-120b", "qwen-3-235b-a22b-instruct-2507", "llama3.1-8b"],
    fallbackModels: [
      { id: "gpt-oss-120b", label: "GPT OSS 120B" },
      { id: "llama3.1-8b", label: "Llama 3.1 8B" },
    ],
    buildLanguageModel({ apiKey, fetchImpl, modelId }) {
      return createCerebras({
        apiKey,
        ...resolveFetchOption(fetchImpl),
      }).languageModel(modelId);
    },
  },
  {
    id: "cohere",
    label: "Cohere",
    envKey: "COHERE_API_KEY",
    defaultModelHints: ["command-r7b-12-2024", "command-r-plus-08-2024"],
    fallbackModels: [
      { id: "command-r7b-12-2024", label: "Command R7B" },
      { id: "command-r-plus-08-2024", label: "Command R Plus" },
    ],
    buildLanguageModel({ apiKey, fetchImpl, modelId }) {
      return createCohere({
        apiKey,
        ...resolveFetchOption(fetchImpl),
      }).languageModel(modelId);
    },
  },
  {
    id: "togetherai",
    label: "Together AI",
    envKey: "TOGETHERAI_API_KEY",
    defaultModelHints: [
      "meta-llama/Llama-3.3-70B-Instruct-Turbo",
      "Qwen/Qwen3-Coder-480B-A35B-Instruct-FP8",
    ],
    fallbackModels: [
      {
        id: "meta-llama/Llama-3.3-70B-Instruct-Turbo",
        label: "Llama 3.3 70B Instruct Turbo",
      },
    ],
    buildLanguageModel({ apiKey, fetchImpl, modelId }) {
      return createTogetherAI({
        apiKey,
        ...resolveFetchOption(fetchImpl),
      }).languageModel(modelId);
    },
  },
  {
    id: "perplexity",
    label: "Perplexity",
    envKey: "PERPLEXITY_API_KEY",
    defaultModelHints: ["sonar", "sonar-pro", "sonar-reasoning-pro"],
    fallbackModels: [
      { id: "sonar", label: "Sonar" },
      { id: "sonar-pro", label: "Sonar Pro" },
    ],
    buildLanguageModel({ apiKey, fetchImpl, modelId }) {
      return createPerplexity({
        apiKey,
        ...resolveFetchOption(fetchImpl),
      }).languageModel(modelId);
    },
  },
  {
    id: "alibaba",
    label: "Alibaba",
    envKey: "ALIBABA_API_KEY",
    defaultModelHints: ["qwen3-coder-plus", "qwen3-235b-a22b", "qwen3-8b"],
    fallbackModels: [
      { id: "qwen3-coder-plus", label: "Qwen 3 Coder Plus" },
      { id: "qwen3-235b-a22b", label: "Qwen 3 235B A22B" },
    ],
    buildLanguageModel({ apiKey, fetchImpl, modelId }) {
      return createAlibaba({
        apiKey,
        ...resolveFetchOption(fetchImpl),
      }).languageModel(modelId);
    },
  },
];

const HOSTED_PROVIDER_REGISTRY = HOSTED_PROVIDER_DEFINITIONS.reduce(
  (registry, provider) => {
    registry[provider.id] = provider;
    return registry;
  },
  {} as Record<HostedProviderId, HostedProviderDefinition>,
);

function isChatCapableCatalogModel(model: ModelsDevModel): boolean {
  if (model.status === "deprecated") {
    return false;
  }

  const haystack = `${model.id} ${model.name} ${model.family ?? ""}`;
  if (NON_CHAT_MODEL_ID_PATTERN.test(model.id) || NON_CHAT_MODEL_FAMILY_PATTERN.test(haystack)) {
    return false;
  }

  return true;
}

function dedupeCatalogModels(models: readonly CatalogModel[]): CatalogModel[] {
  const seen = new Set<string>();
  const deduped: CatalogModel[] = [];
  for (const model of models) {
    if (!model.id.trim() || seen.has(model.id)) continue;
    seen.add(model.id);
    deduped.push(model);
  }
  return deduped;
}

function resolveCatalogModels(
  definition: HostedProviderDefinition,
  dynamicModels: readonly CatalogModel[],
): CatalogModel[] {
  const merged = dedupeCatalogModels([...dynamicModels, ...definition.fallbackModels]);
  const defaultModelId = resolveDefaultModelId(definition, merged);
  const defaultModel = merged.find((model) => model.id === defaultModelId);
  const withoutDefault = merged.filter((model) => model.id !== defaultModelId);
  return defaultModel ? [defaultModel, ...withoutDefault] : withoutDefault;
}

function resolveDefaultModelId(
  definition: HostedProviderDefinition,
  models: readonly CatalogModel[],
): string {
  for (const hint of definition.defaultModelHints) {
    const exact = models.find((model) => model.id === hint);
    if (exact) return exact.id;
  }
  for (const hint of definition.defaultModelHints) {
    const partial = models.find((model) => model.id.includes(hint));
    if (partial) return partial.id;
  }
  return models[0]?.id ?? definition.fallbackModels[0]?.id ?? "";
}

function modelsFromModelsDevProvider(
  definition: HostedProviderDefinition,
  provider: {
    models: Record<string, ModelsDevModel>;
  } | null,
): CatalogModel[] {
  if (!provider) {
    return [...definition.fallbackModels];
  }

  const dynamicModels = Object.values(provider.models)
    .filter(isChatCapableCatalogModel)
    .map((model) => ({
      id: model.id,
      label: model.name,
    }));

  return resolveCatalogModels(definition, dynamicModels);
}

export function getHostedProviderDefinition(
  providerId: string,
): HostedProviderDefinition | null {
  if (!HOSTED_PROVIDER_IDS.includes(providerId as HostedProviderId)) {
    return null;
  }
  return HOSTED_PROVIDER_REGISTRY[providerId as HostedProviderId];
}

export function isHostedProviderId(providerId: string): providerId is HostedProviderId {
  return getHostedProviderDefinition(providerId) !== null;
}

export function supportsUserLlmCredentialProvider(
  providerId: string,
): providerId is HostedProviderId {
  return isHostedProviderId(providerId);
}

export async function buildHostedProviderCatalog(
  env: AppEnv,
  fetchImpl: typeof fetch,
): Promise<LlmProviderCatalogEntry[]> {
  const modelsDevProviders = await loadModelsDevProviders(env, fetchImpl);

  return HOSTED_PROVIDER_DEFINITIONS.map((provider) => {
    const catalogState = provider.resolveCatalogState?.(env)
      ?? resolveDefaultCatalogState(provider, env);
    const models = modelsFromModelsDevProvider(provider, modelsDevProviders[provider.id] ?? null);

    return {
      id: provider.id,
      label: provider.label,
      authMode: catalogState.authMode,
      configured: catalogState.configured,
      defaultModel: resolveDefaultModelId(provider, models),
      models,
    };
  });
}

export function resolveHostedProviderRequest(
  env: AppEnv,
  providerId: HostedProviderId,
  apiKeyOverride?: string | null,
): HostedProviderRequestResolution {
  const provider = HOSTED_PROVIDER_REGISTRY[providerId];
  return provider.resolveRequest?.(env, apiKeyOverride)
    ?? resolveDefaultRequest(provider, env, apiKeyOverride);
}

export function resolveHostedProviderLogUrl(
  env: AppEnv,
  providerId: HostedProviderId,
): string | undefined {
  return HOSTED_PROVIDER_REGISTRY[providerId].resolveLogUrl?.(env);
}

export function buildHostedProviderLanguageModel(input: {
  env: AppEnv;
  providerId: HostedProviderId;
  apiKey?: string;
  fetchImpl?: typeof fetch;
  modelId: string;
}): LanguageModelV3 {
  return HOSTED_PROVIDER_REGISTRY[input.providerId].buildLanguageModel(input);
}
