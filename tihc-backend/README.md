# TIHC Node Backend

Node.js `24.x` Vercel Functions backend for TIHC, implemented with `Hono`.

## Endpoints

- `GET /health`
- `GET /v1/settings`
- `PUT /v1/settings`
- `POST /v1/chat/completions`
- `POST /v1/telemetry`
- `GET /v1/cases`
- `POST /v1/cases`
- `PATCH /v1/cases/:caseId`
- `DELETE /v1/cases/:caseId`
- `GET /v1/cases/:caseId/history`
- `PUT /v1/cases/:caseId/history`
- `POST /v1/cases/import`

## Environment

- `PORT`
- `LOG_LEVEL`
- `LOG_FORMAT`
- `DATABASE_URL`
- `DATABASE_SSL_CA_PATH`
- `TIDB_API_URL`
- `TIDB_API_TOKEN`
- `OPENAI_API_KEY`
- `OPENAI_API_URL`
- `ANTHROPIC_API_KEY`
- `GOOGLE_API_KEY`
- `XAI_API_KEY`
- `OPENROUTER_API_KEY`
- `MISTRAL_API_KEY`
- `GROQ_API_KEY`
- `DEEPINFRA_API_KEY`
- `CEREBRAS_API_KEY`
- `COHERE_API_KEY`
- `TOGETHERAI_API_KEY`
- `PERPLEXITY_API_KEY`
- `ALIBABA_API_KEY`
- `MODELS_DEV_DISABLE_FETCH`
- `MODELS_DEV_URL`
- `REQUIRE_AUTH`
- `GOOGLE_CLIENT_ID`
- `GOOGLE_WORKSPACE_DOMAIN`
- `GA4_ENABLED`
- `GA4_MEASUREMENT_ID`
- `GA4_API_SECRET`
- `GA4_DEBUG`
- `GA4_USER_ID_SALT`

## Telemetry behavior

- `/v1/telemetry` only forwards the TIHC extension event allowlist to GA4
- Unknown telemetry fields are dropped and session ids are normalized before forwarding
- `GA4_DEBUG=true` or request `debug=true` switches forwarding to the GA4 debug endpoint
- If a valid Google bearer token is present, the backend hashes it into a stable GA4 `user_id`

## Case persistence behavior

- TIHC no longer relies on browser local persistence for app state, case history, or telemetry identifiers
- Authenticated user settings, cases, and histories are stored in TiDB Serverless
- `/v1/cases`, `/v1/cases/:caseId`, `/v1/cases/:caseId/history`, and `/v1/cases/import` require a valid Google bearer token
- `/v1/settings` requires a valid Google bearer token and stores per-principal runtime settings such as analytics consent and plugin configuration
- Case and history rows are isolated by Google principal in TiDB; lookups always scope by `principal_id + case_id`
- Anonymous sessions are in-memory only and never write TiDB rows
- `/v1/chat/completions` remains a stateless proxy and does not persist prompts or completions

## Database migrations

Apply the SQL in [drizzle/0000_tihc_case_cloud.sql](/Users/aric/GlobaFlux/tihc/tihc-backend/drizzle/0000_tihc_case_cloud.sql) to your TiDB Serverless database before serving authenticated case traffic. The backend expects the schema to exist at startup whenever `DATABASE_URL` is configured.

For `drizzle-kit migrate`, TiDB Cloud public endpoints require TLS. This project now enables TLS for migrations by default and optionally reads `DATABASE_SSL_CA_PATH` when you need to point `mysql2` at a specific CA bundle.

## Local development

```bash
npm install
npm run dev
```

Default local port: `3010`

Default local logging:

- `LOG_LEVEL=debug`
- `LOG_FORMAT=pretty`
- `DATABASE_URL` present or missing is reported at boot for TiDB troubleshooting

For a Vercel-faithful local environment, use:

```bash
npm run dev:vercel
```

## Vercel constraints baked into this service

- Node target is `24.x` via `package.json`
- Request bodies above `4.5 MB` are rejected
- Aggregated non-stream responses above `4.5 MB` are rejected
- `maxDuration` is pinned to `60` seconds in `api/index.ts` and `vercel.json`
- No persistent filesystem writes or in-memory auth session sharing are used
