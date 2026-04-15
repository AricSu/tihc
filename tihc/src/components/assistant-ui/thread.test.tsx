import { describe, expect, test } from "vitest";
import { renderToStaticMarkup } from "react-dom/server";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const threadSource = readFileSync(
  resolve(dirname(fileURLToPath(import.meta.url)), "thread.tsx"),
  "utf8",
);

describe("thread welcome", () => {
  test("exports the assistant-ui starter welcome copy instead of the TIHC-specific empty state", async () => {
    const threadModule = await import("./thread");
    const ThreadWelcome = (threadModule as Record<string, any>).ThreadWelcome;

    expect(typeof ThreadWelcome).toBe("function");

    const html = renderToStaticMarkup(<ThreadWelcome />);

    expect(html).toContain("Hello there!");
    expect(html).toContain("How can I help you today?");
    expect(html).not.toContain("TiHC is ready.");
    expect(html).not.toContain("Describe the issue, paste an error, or tell TiHC what to investigate.");
  });

  test("does not keep the extra sticky chrome layer above the composer", () => {
    expect(threadSource).not.toContain("backdrop-blur");
    expect(threadSource).not.toContain("rounded-t-xl");
    expect(threadSource).not.toContain("border-t bg-background/95");
  });

  test("keeps the composer pinned to the bottom and scrolls to bottom when the sidepanel opens", () => {
    expect(threadSource).toContain('className="aui-thread-viewport flex min-h-0 flex-1 flex-col overflow-y-auto px-4"');
    expect(threadSource).toContain('className="aui-thread-viewport-footer sticky bottom-0 mt-auto px-4 pb-4"');
    expect(threadSource).toContain('scrollToBottom({ behavior: "instant" })');
    expect(threadSource).toContain('window.addEventListener("focus", scrollToBottom)');
    expect(threadSource).toContain('document.addEventListener("visibilitychange", handleVisibilityChange)');
  });

  test("supports replacing the left attachment button and extending the right-side action group", () => {
    expect(threadSource).toContain("composerStart?: ReactNode");
    expect(threadSource).toContain("composerEnd?: ReactNode");
    expect(threadSource).toContain("{composerStart ?? <ComposerAddAttachment />}");
    expect(threadSource).toContain("{composerEnd}");
  });

  test("switches between stop and send using thread running state", () => {
    expect(threadSource).toContain("const isRunning = useAssistantState(({ thread }) => thread.isRunning);");
    expect(threadSource).toContain("{isRunning ? (");
    expect(threadSource).toContain("{!isRunning ? (");
    expect(threadSource).toContain('aria-label="Stop generating"');
    expect(threadSource).toContain('aria-label="Send message"');
  });

  test("keeps message shells shrinkable so long content wraps instead of widening the sidepanel", () => {
    expect(threadSource).toContain('<div className="aui-assistant-message min-w-0 max-w-full flex flex-col gap-2">');
    expect(threadSource).toContain(
      '<div className="justify-self-end min-w-0 max-w-full rounded-2xl bg-primary px-4 py-3 text-sm text-primary-foreground">',
    );
  });
});
