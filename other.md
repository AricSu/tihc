

```
                ┌─────────────────────┐
                │     Client / CLI    │
                │  (MCP / REST / Web) │
                └─────────┬──────────┘
                          │ HTTP / WebSocket
                          ▼
                  ┌─────────────────┐
                  │ Router / Entry  │
                  └─────────┬───────┘
                            │
                            ▼
                     ┌─────────────┐
                     │ MessageBus  │
                     │ (双向总线)  │
                     └─────────┬───┘
        ┌────────────┬─────────┼────────────┐
        ▼            ▼         ▼            ▼
  ┌──────────┐  ┌──────────┐ ┌───────────┐ ┌──────────────┐
  │plugin_auth│ │plugin_tihc│ │plugin_rca │ │plugin_alert │
  │(身份/权限) │ │_mcp_server│ │_engine    │ │_webhook     │
  └─────┬─────┘ └─────┬─────┘ └───────────┘ └──────────────┘
        │             │
        │             │
        ▼             ▼
     Identity/Perms Requests/Events
        │             │
        └─────┬───────┘
              ▼
       ┌─────────────┐
       │   Backend   │
       │  Handler    │
       │  /API Logic │
       └─────┬───────┘
            │
            └───► MessageBus ─► 插件处理业务

```