# TIHC Serverless

Serverless backend for TIHC extension.

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
  "chat_engine": "tidb",
  "stream": true,
  "chat_id": "optional"
}
```

Response: streaming plain text.

## Auth (optional)

Send header:

- `Authorization: Bearer <google_token>`

If auth is enabled, server verifies token using Google tokeninfo and enforces:

- `GOOGLE_CLIENT_ID` (audience)
- `GOOGLE_WORKSPACE_DOMAIN` (domain)

## Env Vars

- `TIDB_API_URL`, `TIDB_API_TOKEN` (required)
- `GOOGLE_CLIENT_ID` (optional)
- `GOOGLE_WORKSPACE_DOMAIN` (optional)
- `REQUIRE_AUTH=true|false`
- `LOG_LEVEL` (default: `info`)

Only `.env.example` should be committed.

## Local Dev

```bash
cargo check
```
