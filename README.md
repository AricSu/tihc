# TIHC Monorepo

This repository contains two parts:

- `tihc/`: browser extension sidepanel client
- `tihc-serverless/`: serverless backend API

## Security Rules

- Never commit real API URLs, tokens, or credentials.
- Use `.env.local` for local values.
- Keep only `.env.example` in git.
- Do not commit generated traces, logs, or AI-generated runtime outputs.

## Quick Start

### Extension

```bash
cd tihc
npm i
cp .env.example .env.local
npm run dev
```

### Serverless

```bash
cd tihc-serverless
cp .env.example .env.local
cargo check
```
