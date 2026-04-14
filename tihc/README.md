# TIHC (Browser Extension)

Sidepanel chat UI for TiDB troubleshooting.

## Dev

- Install deps: `npm i`
- Run: `npm run dev`

Default local ports:

- Extension dev server: `3002`
- TIHC Node backend dev server: `3010`

## Configure

Copy `.env.example` to `.env.local` and fill:

- `VITE_BACKEND_BASE_URL`: your TIHC Node backend base URL

Do not commit `.env.local` to git.

## Analytics

- Extension analytics use the configured backend at `/v1/telemetry`
- GA4 runs in strict basic-consent mode: nothing is sent before the user opts in
- Outbound link telemetry sends only `target_domain`, `target_path`, and `link_source`

## Case Model

TIHC v1 uses `Case = Thread`.

- A case stores its own local conversation history by `caseId`
- A case can be `ready`, `active`, or `resolved`
- A case can also be archived independently of its activity state
- The sidepanel shows only non-archived cases in the case switcher

## Plugin Model

TIHC v1 ships with a single built-in plugin:

- `tidb.ai`: compatible with the TIHC Node backend `/v1/chat/completions` API

Every case binds to that global plugin instance. The runtime is already plugin-shaped so future `agent` or `mcp` integrations can plug into the same contract, but v1 does not expose plugin install/remove/switch flows.

Open the extension `Settings` page and use the plugin settings view to edit:

- `Base URL`
- `Model`

The settings page also includes a `Test Connection` action for quick reachability checks.
