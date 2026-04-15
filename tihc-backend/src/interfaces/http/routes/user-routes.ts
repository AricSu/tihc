import {
  buildAnonymousCurrentUser,
  buildAuthenticatedCurrentUser,
} from "../../../domain/users/current-user";
import type { AppLogger } from "../../../lib/logger";
import type { AppInstance, HttpContextHelpers } from "../http-context";

type RegisterUserRoutesOptions = {
  app: AppInstance;
  helpers: HttpContextHelpers;
  logger: AppLogger;
};

export function registerUserRoutes({
  app,
  helpers,
  logger,
}: RegisterUserRoutesOptions) {
  app.get("/v1/me", async (context) => {
    const requestId = helpers.requestIdOf(context);
    const principal = await helpers.resolveOptionalAppPrincipal(context);
    if (principal instanceof Response) return principal;

    const user = principal
      ? buildAuthenticatedCurrentUser(principal)
      : buildAnonymousCurrentUser();

    logger.info("user.read", {
      auth_state: user.authState,
      principal_id: principal?.id ?? null,
      request_id: requestId,
    });

    return context.json({ user }, 200, helpers.noStoreHeaders);
  });
}
