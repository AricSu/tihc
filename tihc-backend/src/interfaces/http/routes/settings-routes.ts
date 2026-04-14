import type { CaseStore } from "../../../domain/cases/case-store";
import type { AppLogger } from "../../../lib/logger";
import { jsonError } from "../http";
import type { AppInstance, HttpContextHelpers } from "../http-context";
import { sanitizePrincipalSettingsInput } from "../cases/case-payloads";

type RegisterSettingsRoutesOptions = {
  app: AppInstance;
  caseStore: CaseStore | null;
  helpers: HttpContextHelpers;
  logger: AppLogger;
};

export function registerSettingsRoutes({
  app,
  caseStore,
  helpers,
  logger,
}: RegisterSettingsRoutesOptions) {
  app.get("/v1/settings", async (context) => {
    const requestId = helpers.requestIdOf(context);
    const principal = await helpers.requireAppPrincipal(context);
    if (principal instanceof Response) return principal;

    const storedSettings = await caseStore!.getSettings(principal.id);
    logger.info("settings.read", {
      principal_id: principal.id,
      request_id: requestId,
      settings_present: Boolean(storedSettings),
    });

    return context.json({ settings: storedSettings }, 200, helpers.noStoreHeaders);
  });

  app.put("/v1/settings", async (context) => {
    const requestId = helpers.requestIdOf(context);
    const principal = await helpers.requireAppPrincipal(context);
    if (principal instanceof Response) return principal;

    const parsedBody = await helpers.readJsonBody<unknown>(context);
    if (!parsedBody.ok) return parsedBody.response;
    const input = sanitizePrincipalSettingsInput(parsedBody.value);
    if (!input) {
      return jsonError(400, "Invalid settings payload.");
    }

    const storedSettings = await caseStore!.saveSettings(principal.id, input);
    logger.info("settings.updated", {
      principal_id: principal.id,
      request_id: requestId,
    });

    return context.json({ settings: storedSettings }, 200, helpers.noStoreHeaders);
  });
}
