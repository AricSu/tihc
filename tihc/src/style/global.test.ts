import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, test } from "vitest";

const globalCss = readFileSync(
  resolve(dirname(fileURLToPath(import.meta.url)), "global.css"),
  "utf8",
);

describe("global theme", () => {
  test("keeps the default starter theme without TIHC-specific font or animation overrides", () => {
    expect(globalCss).not.toContain("--font-ui");
    expect(globalCss).not.toContain("--font-display");
    expect(globalCss).not.toContain("Avenir Next");
    expect(globalCss).not.toContain("Iowan Old Style");
    expect(globalCss).not.toContain("font-family: var(--font-ui)");
    expect(globalCss).not.toContain(".tihc-display");
    expect(globalCss).not.toContain(".tihc-scrollbar");
    expect(globalCss).not.toContain("@keyframes tihcFloat");
    expect(globalCss).not.toContain("@keyframes tihcFadeUp");
  });
});
