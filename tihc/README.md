# TIHC (Browser Extension)

Sidepanel chat UI for TiDB troubleshooting.

## Dev

- Install deps: `npm i`
- Run: `npm run dev`

## Configure

Copy `.env.example` to `.env.local` and fill:

- `VITE_SERVERLESS_BASE_URL`: your serverless base URL
- `VITE_CHAT_ENGINE`: default `tidb`

Do not commit `.env.local` to git.
