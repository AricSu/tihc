# TIHC Workspace

Active projects:

- `tihc/`: browser extension sidepanel client
- `tihc-backend/`: Node backend API

Adjacent directories:

- `askaric/`: website, maintained as a separate git repo
- `tihc-serverless/`: legacy Rust backend kept only for reference

## Quick Start

### Extension

```bash
cd tihc
npm install
cp .env.example .env.local
npm run dev
```

### Backend

```bash
cd tihc-backend
npm install
cp .env.example .env.local
npm test
```

## Repo Rules

- Never commit real API URLs, tokens, or credentials.
- Use `.env.local` for local values.
- Keep only `.env.example` in git.
- Do not commit generated traces, logs, or AI-generated runtime outputs.
