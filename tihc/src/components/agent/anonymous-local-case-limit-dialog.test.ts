import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, test } from "vitest";

const dialogSource = readFileSync(
  resolve(dirname(fileURLToPath(import.meta.url)), "anonymous-local-case-limit-dialog.tsx"),
  "utf8",
);

describe("anonymous local case limit dialog theme", () => {
  test("uses shared theme tokens instead of TIHC-specific heading or slate colors", () => {
    expect(dialogSource).not.toContain("tihc-display");
    expect(dialogSource).not.toContain("text-slate");
    expect(dialogSource).not.toContain("bg-slate");
    expect(dialogSource).not.toContain("border-slate");
  });
});
