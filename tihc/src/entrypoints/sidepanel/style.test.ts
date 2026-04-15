import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, test } from "vitest";

const sidepanelCss = readFileSync(
  resolve(dirname(fileURLToPath(import.meta.url)), "style.css"),
  "utf8",
);

describe("sidepanel root layout", () => {
  test("builds a constrained root flex shell instead of masking overflow with overflow-hidden", () => {
    expect(sidepanelCss).toContain("@apply h-full min-h-0;");
    expect(sidepanelCss).toContain("@apply flex min-h-0 flex-col;");
    expect(sidepanelCss).toContain("@apply flex min-h-0 flex-1 flex-col;");
    expect(sidepanelCss).not.toContain("overflow-hidden");
    expect(sidepanelCss).not.toContain("overflow-x-hidden");
    expect(sidepanelCss).not.toContain("overflow-y-hidden");
  });
});
