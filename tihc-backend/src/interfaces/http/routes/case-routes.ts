import type { CaseStore } from "../../../domain/cases/case-store";
import { deleteTidbChatUpstream, extractTidbChatIdFromHistory } from "../../../application/chat/tidb-chat-binding";
import type { AppLogger } from "../../../lib/logger";
import { jsonError } from "../http";
import type { AppInstance, HttpContextHelpers } from "../http-context";
import {
  sanitizeCaseHistoryRepository,
  sanitizeCreateCaseInput,
  sanitizeImportCasesInput,
  sanitizeUpdateCaseInput,
} from "../cases/case-payloads";
import type { AppEnv } from "../../../shared/support";

type RegisterCaseRoutesOptions = {
  app: AppInstance;
  caseStore: CaseStore | null;
  env: AppEnv;
  fetchImpl: typeof fetch;
  helpers: HttpContextHelpers;
  logger: AppLogger;
};

export function registerCaseRoutes({
  app,
  caseStore,
  env,
  fetchImpl,
  helpers,
  logger,
}: RegisterCaseRoutesOptions) {
  app.get("/v1/cases", async (context) => {
    const requestId = helpers.requestIdOf(context);
    const principal = await helpers.requireAppPrincipal(context);
    if (principal instanceof Response) return principal;

    const storedCases = await caseStore!.listCases(principal.id);
    logger.info("cases.read", {
      case_count: storedCases.length,
      principal_id: principal.id,
      request_id: requestId,
    });

    return context.json({ cases: storedCases }, 200, helpers.noStoreHeaders);
  });

  app.post("/v1/cases", async (context) => {
    const requestId = helpers.requestIdOf(context);
    const principal = await helpers.requireAppPrincipal(context);
    if (principal instanceof Response) return principal;

    const parsedBody = await helpers.readJsonBody<unknown>(context);
    if (!parsedBody.ok) return parsedBody.response;
    const input = sanitizeCreateCaseInput(parsedBody.value);
    if (!input) {
      return jsonError(400, "Invalid case payload.");
    }

    const created = await caseStore!.createCase(principal.id, input);
    logger.info("cases.created", {
      case_id: created.id,
      principal_id: principal.id,
      request_id: requestId,
    });

    return context.json({ case: created }, 201, helpers.noStoreHeaders);
  });

  app.patch("/v1/cases/:caseId", async (context) => {
    const requestId = helpers.requestIdOf(context);
    const principal = await helpers.requireAppPrincipal(context);
    if (principal instanceof Response) return principal;

    const parsedBody = await helpers.readJsonBody<unknown>(context);
    if (!parsedBody.ok) return parsedBody.response;
    const patch = sanitizeUpdateCaseInput(parsedBody.value);
    if (!patch) {
      return jsonError(400, "Invalid case patch payload.");
    }

    const updated = await caseStore!.updateCase(principal.id, context.req.param("caseId"), patch);
    if (!updated) {
      return jsonError(404, "Case not found.");
    }

    logger.info("cases.updated", {
      case_id: updated.id,
      principal_id: principal.id,
      request_id: requestId,
    });

    return context.json({ case: updated }, 200, helpers.noStoreHeaders);
  });

  app.delete("/v1/cases/:caseId", async (context) => {
    const requestId = helpers.requestIdOf(context);
    const principal = await helpers.requireAppPrincipal(context);
    if (principal instanceof Response) return principal;

    const caseId = context.req.param("caseId");
    const history = await caseStore!.getHistory(principal.id, caseId);
    const tidbChatId = extractTidbChatIdFromHistory(history);
    if (tidbChatId) {
      try {
        await deleteTidbChatUpstream({
          chatId: tidbChatId,
          env,
          fetchImpl,
          logger,
          requestId,
        });
      } catch (error) {
        return jsonError(502, error instanceof Error ? error.message : "Failed to delete tidb.ai chat.");
      }
    }

    const deleted = await caseStore!.deleteCase(principal.id, caseId);
    if (!deleted) {
      return jsonError(404, "Case not found.");
    }

    logger.info("cases.deleted", {
      case_id: caseId,
      principal_id: principal.id,
      request_id: requestId,
    });

    return new Response(null, {
      headers: helpers.noStoreHeaders,
      status: 204,
    });
  });

  app.get("/v1/cases/:caseId/history", async (context) => {
    const principal = await helpers.requireAppPrincipal(context);
    if (principal instanceof Response) return principal;

    const repository = await caseStore!.getHistory(principal.id, context.req.param("caseId"));
    if (!repository) {
      return jsonError(404, "Case not found.");
    }

    return context.json({ repository }, 200, helpers.noStoreHeaders);
  });

  app.put("/v1/cases/:caseId/history", async (context) => {
    const principal = await helpers.requireAppPrincipal(context);
    if (principal instanceof Response) return principal;

    const parsedBody = await helpers.readJsonBody<{ repository?: unknown }>(context);
    if (!parsedBody.ok) return parsedBody.response;
    const repository = sanitizeCaseHistoryRepository(parsedBody.value.repository);
    if (!repository) {
      return jsonError(400, "Invalid history repository payload.");
    }

    const updated = await caseStore!.saveHistory(principal.id, context.req.param("caseId"), repository);
    if (!updated) {
      return jsonError(404, "Case not found.");
    }

    return context.json({ case: updated }, 200, helpers.noStoreHeaders);
  });

  app.post("/v1/cases/import", async (context) => {
    const requestId = helpers.requestIdOf(context);
    const principal = await helpers.requireAppPrincipal(context);
    if (principal instanceof Response) return principal;

    const parsedBody = await helpers.readJsonBody<unknown>(context);
    if (!parsedBody.ok) return parsedBody.response;
    const input = sanitizeImportCasesInput(parsedBody.value);
    if (!input) {
      return jsonError(400, "Invalid cases import payload.");
    }

    const result = await caseStore!.importCases(principal.id, input);
    logger.info("cases.imported", {
      already_imported: result.alreadyImported,
      imported_cases: result.importedCases,
      principal_id: principal.id,
      request_id: requestId,
    });

    return context.json(result, 200, helpers.noStoreHeaders);
  });
}
