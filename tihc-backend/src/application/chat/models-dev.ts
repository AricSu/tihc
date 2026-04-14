import { z } from "zod";
import { resolveEnvValue, truthyEnvFlag, type AppEnv } from "../../shared/support";

const MODELS_DEV_DEFAULT_URL = "https://models.dev/api.json";
const MODELS_DEV_CACHE_TTL_MS = 5 * 60 * 1000;

const ModelsDevModelSchema = z.object({
  family: z.string().optional(),
  id: z.string(),
  modalities: z
    .object({
      input: z.array(z.string()).optional(),
      output: z.array(z.string()).optional(),
    })
    .optional(),
  name: z.string(),
  status: z.enum(["alpha", "beta", "deprecated"]).optional(),
});

const ModelsDevProviderSchema = z.object({
  api: z.string().optional(),
  env: z.array(z.string()).optional(),
  id: z.string(),
  models: z.record(z.string(), ModelsDevModelSchema),
  name: z.string(),
  npm: z.string().optional(),
});

const ModelsDevCatalogSchema = z.record(z.string(), ModelsDevProviderSchema);

export type ModelsDevModel = z.infer<typeof ModelsDevModelSchema>;
export type ModelsDevProvider = z.infer<typeof ModelsDevProviderSchema>;

let modelsDevCache:
  | {
      expiresAt: number;
      providers: Record<string, ModelsDevProvider>;
    }
  | null = null;

function resolveModelsDevUrl(env: AppEnv): string {
  return resolveEnvValue(env, "MODELS_DEV_URL") || MODELS_DEV_DEFAULT_URL;
}

function shouldFetchModelsDev(env: AppEnv): boolean {
  const configured = resolveEnvValue(env, "MODELS_DEV_DISABLE_FETCH");
  if (configured) {
    return !truthyEnvFlag(configured);
  }
  return !Boolean(process.env.VITEST);
}

export async function loadModelsDevProviders(
  env: AppEnv,
  fetchImpl: typeof fetch,
): Promise<Record<string, ModelsDevProvider>> {
  if (!shouldFetchModelsDev(env)) {
    return modelsDevCache?.providers ?? {};
  }

  const now = Date.now();
  if (modelsDevCache && modelsDevCache.expiresAt > now) {
    return modelsDevCache.providers;
  }

  try {
    const response = await fetchImpl(resolveModelsDevUrl(env), {
      headers: {
        Accept: "application/json",
      },
      method: "GET",
    });
    if (!response.ok) {
      return modelsDevCache?.providers ?? {};
    }

    const payload = await response.json().catch(() => null);
    const parsed = ModelsDevCatalogSchema.safeParse(payload);
    if (!parsed.success) {
      return modelsDevCache?.providers ?? {};
    }

    modelsDevCache = {
      expiresAt: now + MODELS_DEV_CACHE_TTL_MS,
      providers: parsed.data,
    };
    return parsed.data;
  } catch {
    return modelsDevCache?.providers ?? {};
  }
}
