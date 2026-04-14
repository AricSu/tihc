import type {
  InstalledPlugin,
  PluginMarketplaceEntry,
  PluginMarketplaceStatus,
  PluginId,
  PluginManifest,
} from "@/lib/chat/agent-types";
import type {
  AgentConnectionTestResult,
  AgentEvent,
  ChatMessage,
  UnifiedChatRequest,
} from "@/lib/chat/connectors/types";

export type PluginAdapter = {
  streamChat(
    plugin: InstalledPlugin,
    request: UnifiedChatRequest,
  ): AsyncGenerator<AgentEvent>;
  testConnection(plugin: InstalledPlugin): Promise<AgentConnectionTestResult>;
};

const TIDB_MANIFEST: PluginManifest = {
  pluginId: "tidb.ai",
  label: "tidb.ai",
  kind: "mcp",
  capabilities: ["mcp"],
  settingsFields: [
    {
      key: "baseUrl",
      label: "Base URL",
      type: "url",
      description: "Optional remote endpoint for the tidb.ai MCP client.",
      placeholder: "https://example.tidb.ai",
    },
  ],
};

const WEBSEARCH_MANIFEST: PluginManifest = {
  pluginId: "websearch",
  label: "Web Search",
  kind: "mcp",
  capabilities: ["mcp"],
  settingsFields: [
    {
      key: "enabled",
      label: "Enable Web Search",
      type: "checkbox",
      description: "Run extension-managed web search before sending prompts upstream.",
    },
    {
      key: "mode",
      label: "Web Search Mode",
      type: "select",
      description: "When enabled, aggressive mode searches the web before each request unless the user opts out in-message.",
      options: [
        { label: "Off", value: "off" },
        { label: "Aggressive", value: "aggressive" },
      ],
    },
    {
      key: "primaryEngine",
      label: "Primary Search Engine",
      type: "select",
      description: "Preferred engine for extension-managed web search.",
      options: [
        { label: "DuckDuckGo", value: "duckduckgo" },
        { label: "Bing", value: "bing" },
        { label: "Baidu", value: "baidu" },
      ],
    },
  ],
};

const TIDB_ADAPTER: PluginAdapter = {
  async *streamChat(_plugin, _request) {
    yield {
      type: "error",
      message: "tidb.ai is configured as an MCP client and does not provide the primary chat runtime.",
    };
  },
  async testConnection(_plugin) {
    return {
      ok: true,
      message: "tidb.ai is configured as an MCP client and does not provide a standalone chat connection test.",
    };
  },
};

const WEBSEARCH_ADAPTER: PluginAdapter = {
  async *streamChat(_plugin, _request) {
    yield {
      type: "error",
      message: "Web Search does not provide a chat runtime.",
    };
  },
  async testConnection(_plugin) {
    return {
      ok: true,
      message: "Web Search runs inside the extension and does not require a remote connection test.",
    } satisfies AgentConnectionTestResult;
  },
};

const MARKETPLACE_CATALOG: Array<
  Omit<PluginMarketplaceEntry, "status"> & {
    defaultStatus: Exclude<PluginMarketplaceStatus, "installed">;
    sortOrder: number;
  }
> = [
  {
    catalogId: "tidb.ai",
    title: "tidb.ai",
    provider: "PingCAP",
    builtBy: "PingCAP",
    group: "Featured",
    kind: "mcp",
    capabilities: ["mcp"],
    installedPluginId: "tidb.ai",
    defaultStatus: "available",
    sortOrder: 10,
    summary: "TiDB-specific MCP client for retrieval and tool context.",
    description:
      "Attach tidb.ai alongside other MCP clients so TIHC can pull TiDB-specific context without conflating that integration with the primary global LLM runtime.",
    heroPrompt:
      "Use tidb.ai as a TiDB-focused MCP client while the primary case chat runtime stays global and provider-driven.",
    tags: ["installed", "mcp", "tidb"],
    highlights: [
      "Lives alongside Web Search as an MCP layer",
      "Decoupled from the primary case chat runtime",
      "Optional remote endpoint configuration",
    ],
    includes: [
      {
        name: "tidb.ai MCP Client",
        type: "Connector",
        description: "Connects TiDB-specific context sources without owning the primary case chat runtime.",
        enabled: true,
      },
      {
        name: "TiDB Context",
        type: "App",
        description: "Provides space for TiDB-focused MCP functionality as the integration expands.",
        enabled: true,
      },
    ],
    information: [
      { label: "Category", value: "Built by PingCAP, MCP" },
      { label: "Capabilities", value: "Context, Retrieval" },
      { label: "Developer", value: "PingCAP" },
      { label: "Website", value: "tidb.ai", href: "https://tidb.ai" },
      {
        label: "Privacy policy",
        value: "PingCAP Privacy",
        href: "https://www.pingcap.com/privacy-policy/",
      },
      {
        label: "Terms of service",
        value: "PingCAP Terms",
        href: "https://www.pingcap.com/terms-of-use/",
      },
    ],
  },
  {
    catalogId: "websearch",
    title: "Web Search",
    provider: "TIHC",
    builtBy: "TIHC",
    group: "Featured",
    kind: "mcp",
    capabilities: ["mcp"],
    installedPluginId: "websearch",
    defaultStatus: "available",
    sortOrder: 20,
    summary: "Automatic web grounding that runs before global-runtime chats.",
    description:
      "Use a reusable browser worker tab to search DuckDuckGo, Bing, or Baidu, extract results, and inject source context into the outbound prompt without mixing search settings into the global LLM runtime configuration.",
    heroPrompt:
      "Search the live web inside the extension, auto-ground case answers with links, and keep the global chat runtime focused on synthesis instead of discovery.",
    tags: ["search", "grounding", "browser"],
    highlights: [
      "Runs automatically before case chats",
      "Uses a reusable browser worker tab",
      "Supports DuckDuckGo, Bing, and Baidu fallback",
    ],
    includes: [
      {
        name: "Search Worker",
        type: "App",
        description: "Uses a dedicated browser tab to load search result pages and result documents.",
        enabled: true,
      },
      {
        name: "SERP Extraction",
        type: "Connector",
        description: "Normalizes titles, links, snippets, and page excerpts into prompt-ready search context.",
        enabled: true,
      },
    ],
    information: [
      { label: "Category", value: "Built by TIHC, Retrieval" },
      { label: "Capabilities", value: "Interactive, Browse" },
      { label: "Developer", value: "TIHC" },
      { label: "Website", value: "TIHC", href: "https://github.com" },
    ],
  },
];

const MANIFESTS: Record<PluginId, PluginManifest> = {
  "tidb.ai": TIDB_MANIFEST,
  websearch: WEBSEARCH_MANIFEST,
};

const ADAPTERS: Record<PluginId, PluginAdapter> = {
  "tidb.ai": TIDB_ADAPTER,
  websearch: WEBSEARCH_ADAPTER,
};

export function listPluginManifests(): PluginManifest[] {
  return Object.values(MANIFESTS);
}

export function getPluginManifest(pluginId: PluginId): PluginManifest {
  return MANIFESTS[pluginId];
}

export function getPluginAdapter(pluginId: PluginId): PluginAdapter {
  return ADAPTERS[pluginId];
}

export function buildPluginMessages(messages: ChatMessage[]): ChatMessage[] {
  return messages;
}

export function listMarketplacePluginCatalog(installedPluginIds: PluginId[]): PluginMarketplaceEntry[] {
  return MARKETPLACE_CATALOG.map((entry): PluginMarketplaceEntry => {
    const { defaultStatus, sortOrder: _sortOrder, ...rest } = entry;
    return {
      ...rest,
      status:
        entry.installedPluginId && installedPluginIds.includes(entry.installedPluginId)
          ? "installed"
          : defaultStatus,
    };
  }).sort((left, right) => {
    const order: Record<PluginMarketplaceStatus, number> = {
      installed: 0,
      available: 1,
      "coming-soon": 2,
    };
    const groupOrder: Record<PluginMarketplaceEntry["group"], number> = {
      Featured: 0,
      Coding: 1,
    };
    const groupDelta = groupOrder[left.group] - groupOrder[right.group];
    if (groupDelta !== 0) return groupDelta;
    const statusDelta = order[left.status] - order[right.status];
    if (statusDelta !== 0) return statusDelta;
    const leftSort = MARKETPLACE_CATALOG.find((entry) => entry.catalogId === left.catalogId)?.sortOrder ?? 999;
    const rightSort = MARKETPLACE_CATALOG.find((entry) => entry.catalogId === right.catalogId)?.sortOrder ?? 999;
    const sortDelta = leftSort - rightSort;
    if (sortDelta !== 0) return sortDelta;
    return left.title.localeCompare(right.title);
  });
}
