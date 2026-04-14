export type AppEnv = {
  ALIBABA_API_KEY?: string;
  ANTHROPIC_API_KEY?: string;
  CEREBRAS_API_KEY?: string;
  COHERE_API_KEY?: string;
  DATABASE_URL?: string;
  DEEPINFRA_API_KEY?: string;
  GA4_API_SECRET?: string;
  GA4_DEBUG?: string;
  GA4_ENABLED?: string;
  GA4_MEASUREMENT_ID?: string;
  GA4_USER_ID_SALT?: string;
  GOOGLE_CLIENT_ID?: string;
  GOOGLE_API_KEY?: string;
  GOOGLE_WORKSPACE_DOMAIN?: string;
  GROQ_API_KEY?: string;
  LOG_FORMAT?: string;
  LOG_LEVEL?: string;
  MISTRAL_API_KEY?: string;
  MODELS_DEV_DISABLE_FETCH?: string;
  MODELS_DEV_URL?: string;
  OPENAI_API_KEY?: string;
  OPENAI_API_URL?: string;
  OPENROUTER_API_KEY?: string;
  PERPLEXITY_API_KEY?: string;
  REQUIRE_AUTH?: string;
  TIDB_API_TOKEN?: string;
  TIDB_API_URL?: string;
  TOGETHERAI_API_KEY?: string;
  XAI_API_KEY?: string;
};

export function asString(value: unknown): string | null {
  return typeof value === "string" ? value : null;
}

export function resolveEnvValue(env: AppEnv, key: keyof AppEnv): string {
  return env[key]?.trim() ?? "";
}

export function truthyEnvFlag(value: string): boolean {
  return ["1", "true", "yes", "y"].includes(value.toLowerCase());
}

export function authRequired(env: AppEnv): boolean {
  return truthyEnvFlag(resolveEnvValue(env, "REQUIRE_AUTH"));
}

export function ga4Enabled(env: AppEnv): boolean {
  return truthyEnvFlag(resolveEnvValue(env, "GA4_ENABLED"));
}

export function ga4DebugEnabled(env: AppEnv): boolean {
  return truthyEnvFlag(resolveEnvValue(env, "GA4_DEBUG"));
}

export function defaultLogLevel(env: AppEnv): string {
  const configured = resolveEnvValue(env, "LOG_LEVEL");
  if (configured) return configured;
  return process.env.VITEST ? "silent" : "info";
}

export function defaultLogFormat(env: AppEnv): string {
  const configured = resolveEnvValue(env, "LOG_FORMAT");
  if (configured) return configured;
  return "pretty";
}

export function redactUrlForLogs(rawUrl: string | null | undefined): string | undefined {
  if (!rawUrl?.trim()) return undefined;
  try {
    const parsed = new URL(rawUrl);
    return `${parsed.origin}${parsed.pathname}`;
  } catch {
    return rawUrl.trim();
  }
}

export function errorMessage(error: unknown): string {
  if (error instanceof Error && error.message.trim()) return error.message.trim();
  return String(error || "Unknown error");
}

export function truncateValue(value: string | null, maxLength: number): string | undefined {
  if (!value) return undefined;
  const trimmed = value.trim();
  if (!trimmed) return undefined;
  return trimmed.length <= maxLength ? trimmed : trimmed.slice(0, maxLength);
}
