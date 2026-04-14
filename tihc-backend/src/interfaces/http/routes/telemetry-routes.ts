import {
  forwardTelemetry,
  type TelemetryRequest,
} from "../../../application/telemetry/telemetry-forwarder";
import type { AppLogger } from "../../../lib/logger";
import type { AppEnv } from "../../../shared/support";
import type { AppInstance, HttpContextHelpers } from "../http-context";

type RegisterTelemetryRoutesOptions = {
  app: AppInstance;
  env: AppEnv;
  fetchImpl: typeof fetch;
  helpers: HttpContextHelpers;
  logger: AppLogger;
};

export function registerTelemetryRoutes({
  app,
  env,
  fetchImpl,
  helpers,
  logger,
}: RegisterTelemetryRoutesOptions) {
  app.post("/v1/telemetry", async (context) => {
    const requestId = helpers.requestIdOf(context);
    const parsedBody = await helpers.readJsonBody<TelemetryRequest>(context);
    if (!parsedBody.ok) {
      logger.warn("telemetry.invalid_request", {
        request_id: requestId,
        status: parsedBody.response.status,
      });
      return parsedBody.response;
    }
    return forwardTelemetry(context.req.raw, parsedBody.value, env, fetchImpl, logger, requestId);
  });
}
