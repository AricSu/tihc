import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, test } from "vitest";

const markdownTextSource = readFileSync(
  resolve(dirname(fileURLToPath(import.meta.url)), "markdown-text.tsx"),
  "utf8",
);

describe("markdown text theme", () => {
  test("uses starter theme tokens instead of TIHC-specific typography and slate colors", () => {
    expect(markdownTextSource).not.toContain("tihc-display");
    expect(markdownTextSource).not.toContain("text-slate");
    expect(markdownTextSource).not.toContain("bg-slate");
    expect(markdownTextSource).not.toContain("border-slate");
    expect(markdownTextSource).not.toContain("var(--tihc-assistant-");
  });

  test("wraps long markdown content inside the sidepanel instead of allowing horizontal scrolling", () => {
    expect(markdownTextSource).toContain('className="aui-md min-w-0 max-w-full [overflow-wrap:anywhere]"');
    expect(markdownTextSource).toContain('"aui-md-p mt-4 leading-7 [overflow-wrap:anywhere] first:mt-0"');
    expect(markdownTextSource).toContain('"aui-md-a font-medium text-primary underline underline-offset-4 [overflow-wrap:anywhere]"');
    expect(markdownTextSource).toContain('"aui-md-table my-6 w-full table-fixed rounded-lg border text-sm"');
    expect(markdownTextSource).toContain(
      '"aui-md-th border-b bg-muted/50 px-4 py-2 text-left font-medium text-muted-foreground whitespace-normal break-words [[align=center]]:text-center [[align=right]]:text-right"',
    );
    expect(markdownTextSource).toContain(
      '"aui-md-td border-b px-4 py-2 text-left align-top whitespace-normal break-words [[align=center]]:text-center [[align=right]]:text-right"',
    );
    expect(markdownTextSource).toContain(
      '"aui-md-pre rounded-b-lg border border-t-0 bg-muted px-4 py-3 text-sm whitespace-pre-wrap [overflow-wrap:anywhere]"',
    );
    expect(markdownTextSource).not.toContain("overflow-x-auto");
  });
});
