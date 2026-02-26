---
name: TIHC Serverless
slug: tihc-serverless
description: Vercel serverless backend for TIHC (OpenAI / Manus / TiDB doc GraphRAG) with Google Workspace auth.
framework:
  - Other
type:
  - Backend
css:
  - None
publisher: AskAric
relatedTemplates:
  - rust-hello-world
---

# TIHC Serverless (Vercel Rust)

Streaming backend used by `tihc` extension.

## Project Structure

- `api/stream_chat.rs` - `POST /api/stream_chat` handler (streaming)
- `Cargo.toml` - Rust deps
- `vercel.json` - rewrite to `/api/stream_chat`

## API

### `POST /api/stream_chat`

Request JSON:

```json
{
  "messages": [{ "role": "user", "content": "..." }],
  "chat_engine": "openai",
  "stream": true,
  "chat_id": "optional"
}
```

Response: streaming text (chunked).

### Engines

- `openai`: OpenAI streaming → plain text stream
- `openai_rag`: call TiDB doc GraphRAG first, then call OpenAI with the retrieved context
- `manus`: proxy to `MANUS_API_URL`
- `tidb` (or anything else): proxy to `TIDB_API_URL`

## Auth (Workspace domain)

Send:

- `Authorization: Bearer <google_token>`

Server verifies via Google `tokeninfo`, enforces `aud=GOOGLE_CLIENT_ID` (if set), and domain via `GOOGLE_WORKSPACE_DOMAIN`.

## Env Vars

- `OPENAI_API_KEY` (required for `openai`/`openai_rag`)
- `OPENAI_MODEL` (default: `gpt-4o-mini`)
- `TIDB_API_URL`, `TIDB_API_TOKEN` (required for `tidb` and `openai_rag`)
- `MANUS_API_URL`, `MANUS_API_TOKEN` (required for `manus`)
- `GOOGLE_CLIENT_ID` (if set, auth is required unless overridden)
- `GOOGLE_WORKSPACE_DOMAIN` (e.g. `pingcap.com`)
- `REQUIRE_AUTH=true|false`
- `RAG_MAX_CHARS` (default: `20000`)
- `.env.example` is provided for local setup.

## Local dev

This repo is designed for Vercel, but you can still sanity-check Rust compile:

```bash
cargo check
```
