"use client";

import type { ReactNode } from "react";
import { useMemo } from "react";
import {
  AssistantRuntimeProvider,
  useLocalRuntime,
  type ChatModelAdapter,
} from "@assistant-ui/react";
import type {
  CaseWorkspace,
  WebSearchInstalledPlugin,
} from "@/lib/chat/agent-types";
import type { ChatMessage } from "@/lib/chat/connectors/types";
import { isCloudSyncEnabled } from "@/lib/app/cloud-cases";
import { createCloudCaseHistoryAdapter } from "@/lib/app/cloud-history";
import { createCaseHistoryAdapter } from "@/lib/app/thread-history";
import { autoTitleCaseFromPrompt, getAppSettingsSnapshot } from "@/lib/app/runtime";
import { hasCompleteGlobalLlmRuntime, streamGlobalRuntime } from "@/lib/llm/runtime";
import { trackTelemetryEvent } from "@/lib/telemetry";
import { runWebSearch } from "@/lib/websearch/client";
import type { WebSearchBundle } from "@/lib/websearch/types";
import {
  injectWebSearchContext,
  shouldRunWebSearch,
} from "@/lib/websearch/policy";

function stripHtmlComments(text: string): string {
  return text.replace(/<!--[\s\S]*?-->/g, "").trim();
}

function toConnectorMessages(
  messages: Parameters<ChatModelAdapter["run"]>[0]["messages"],
): ChatMessage[] {
  const tail = messages.slice(-12);
  return tail
    .filter((msg) => msg.role === "user" || msg.role === "assistant")
    .map((msg) => ({
      role: msg.role,
      content: stripHtmlComments(
        msg.content
          .filter((content) => content.type === "text")
          .map((content) => (content as { type: "text"; text: string }).text)
          .join("\n"),
      ),
    }))
    .filter((msg) => msg.content);
}

function renderOutput(statusLines: string[], answerText: string): string {
  const sections: string[] = [];
  if (statusLines.length) {
    sections.push(`Retrieval Process:\n${statusLines.map((line) => `- ${line}`).join("\n")}`);
  }
  if (answerText.trim()) {
    sections.push(`Answer:\n${answerText}`);
  } else if (statusLines.length) {
    sections.push("Answer:\n");
  }
  return sections.join("\n\n").trim();
}

function classifyFailureKind(message: string): "auth" | "network" | "upstream" | "timeout" | "unknown" {
  const normalized = message.trim().toLowerCase();
  if (
    normalized.includes("timed out") ||
    normalized.includes("timeout") ||
    normalized.includes("deadline exceeded")
  ) {
    return "timeout";
  }
  if (
    normalized.includes("unauthorized") ||
    normalized.includes("forbidden") ||
    normalized.includes("invalid token") ||
    normalized.includes("oauth") ||
    normalized.includes("sign in")
  ) {
    return "auth";
  }
  if (
    normalized.includes("network") ||
    normalized.includes("failed to fetch") ||
    normalized.includes("econn") ||
    normalized.includes("enotfound") ||
    normalized.includes("socket")
  ) {
    return "network";
  }
  if (
    normalized.includes("upstream") ||
    normalized.includes("bad gateway") ||
    normalized.includes("gateway") ||
    normalized.includes("server error") ||
    /\b5\d{2}\b/.test(normalized)
  ) {
    return "upstream";
  }
  return "unknown";
}

export function buildModelAdapter(
  caseId: string,
): ChatModelAdapter {
  return {
    async *run({ messages, abortSignal }) {
      const connectorMessages = toConnectorMessages(messages);
      const lastUser = connectorMessages.filter((message) => message.role === "user").at(-1);
      if (!lastUser) {
        yield { content: [{ type: "text", text: "Please enter your question first." }] };
        return;
      }

      const appSettings = getAppSettingsSnapshot();
      if (!hasCompleteGlobalLlmRuntime(appSettings.llmRuntime)) {
        yield {
          content: [
            {
              type: "text",
              text: "User LLM Settings are not configured. Open Settings and choose a provider and model before chatting.",
            },
          ],
        };
        return;
      }

      autoTitleCaseFromPrompt(caseId, lastUser.content);
      void trackTelemetryEvent("tihc_ext_chat_submitted", {
        context: {
          case_id: caseId,
          surface: "sidepanel",
        },
      });

      const statusLines: string[] = [];
      let answerText = "";
      let renderedText = "";
      const seenStatuses = new Set<string>();
      let outboundMessages = connectorMessages;

      const emitSnapshot = async function* () {
        const nextText = renderOutput(statusLines, answerText);
        if (!nextText || nextText === renderedText) return;
        renderedText = nextText;
        yield {
          content: [{ type: "text" as const, text: nextText }],
        };
      };

      const pushStatus = async function* (text: string) {
        const key = text.trim();
        if (!key || seenStatuses.has(key)) return;
        seenStatuses.add(key);
        statusLines.push(key);
        yield* emitSnapshot();
      };

      const webSearchPlugin =
        appSettings.installedPlugins.find(
          (plugin): plugin is WebSearchInstalledPlugin => plugin.pluginId === "websearch",
        ) ?? null;

      if (webSearchPlugin && shouldRunWebSearch(webSearchPlugin.config, lastUser.content)) {
        const pendingStatuses: string[] = [];
        let wakeSearchLoop: (() => void) | null = null;
        let searchDone = false;

        const waitForSearchEvent = () =>
          new Promise<void>((resolve) => {
            wakeSearchLoop = resolve;
          });

        const signalSearchEvent = () => {
          const resolver = wakeSearchLoop;
          wakeSearchLoop = null;
          resolver?.();
        };

        const searchPromise: Promise<WebSearchBundle | null> = runWebSearch({
          abortSignal,
          onStatus: (status) => {
            pendingStatuses.push(status);
            signalSearchEvent();
          },
          primaryEngine: webSearchPlugin.config.primaryEngine,
          query: lastUser.content,
        })
          .catch((error) => {
            if (!abortSignal.aborted) {
              pendingStatuses.push(
                `Web search unavailable: ${error instanceof Error ? error.message : String(error)}`,
              );
            }
            return null;
          })
          .finally(() => {
            searchDone = true;
            signalSearchEvent();
          });

        while (!searchDone || pendingStatuses.length) {
          if (!pendingStatuses.length) {
            await waitForSearchEvent();
            continue;
          }

          const nextStatus = pendingStatuses.shift();
          if (!nextStatus) continue;
          yield* pushStatus(nextStatus);
        }

        const resolvedSearchBundle = await searchPromise;
        if (resolvedSearchBundle && resolvedSearchBundle.results.length > 0) {
          const injectedStatus = `Injected ${resolvedSearchBundle.results.length} sources into prompt`;
          if (!seenStatuses.has(injectedStatus)) {
            yield* pushStatus(injectedStatus);
          }
          outboundMessages = injectWebSearchContext(connectorMessages, resolvedSearchBundle);
        }
      }

      for await (const event of streamGlobalRuntime({
        abortSignal,
        messages: outboundMessages,
      }, appSettings, caseId)) {
        if (event.type === "status") {
          yield* pushStatus(event.text);
          continue;
        }
        if (event.type === "text-delta") {
          answerText += event.text;
          yield* emitSnapshot();
          continue;
        }
        if (event.type === "replace-text") {
          answerText = event.text;
          yield* emitSnapshot();
          continue;
        }
        if (event.type === "error") {
          const message = event.message.trim() || "Unknown connector error.";
          void trackTelemetryEvent("tihc_ext_chat_failed", {
            context: {
              case_id: caseId,
              failure_kind: classifyFailureKind(message),
              surface: "sidepanel",
            },
          });
          yield { content: [{ type: "text", text: message }] };
          return;
        }
        if (event.type === "done") {
          void trackTelemetryEvent("tihc_ext_chat_completed", {
            context: {
              case_id: caseId,
              surface: "sidepanel",
            },
          });
          if (!renderedText && !answerText.trim()) {
            yield {
              content: [
                {
                  type: "text",
                  text: "Answer:\n(No displayable content was returned by the backend.)",
                },
              ],
            };
          }
          return;
        }
      }
    },
  };
}

export function MLCProvider({
  caseWorkspace,
  children,
}: Readonly<{
  caseWorkspace: CaseWorkspace;
  children: ReactNode;
}>) {
  const historyAdapter = useMemo(
    () =>
      isCloudSyncEnabled(getAppSettingsSnapshot())
        ? createCloudCaseHistoryAdapter(caseWorkspace.id)
        : createCaseHistoryAdapter(caseWorkspace.id),
    [caseWorkspace.id],
  );
  const runtimeOptions = useMemo(
    () => ({
      adapters: {
        history: historyAdapter,
      },
    }),
    [historyAdapter],
  );
  const modelAdapter = useMemo(
    () => buildModelAdapter(caseWorkspace.id),
    [caseWorkspace.id],
  );
  const runtime = useLocalRuntime(modelAdapter, runtimeOptions);

  return (
    <AssistantRuntimeProvider runtime={runtime}>
      {children}
    </AssistantRuntimeProvider>
  );
}
