import { describe, expect, test } from "vitest";
import { renderToStaticMarkup } from "react-dom/server";
import { CaseReadyState, ComposerMeta } from "./thread";

describe("case ready state", () => {
  test("keeps the empty state focused on investigation instead of case metadata or target badges", () => {
    const html = renderToStaticMarkup(
      <CaseReadyState title="Ticket 417" executionTargetName="Local Lobster" />,
    );

    expect(html).toContain("Describe the issue, paste an error, or tell TiHC what to investigate.");
    expect(html).not.toContain("Ticket 417");
    expect(html).not.toContain("Local Lobster");
    expect(html).not.toContain("Running on");
    expect(html).not.toContain("A quieter way");
    expect(html).not.toContain("work with agents");
    expect(html).not.toContain("Switch agents on the right");
  });
});

describe("composer meta", () => {
  test("renders custom case controls in place of the newline hint", () => {
    const html = renderToStaticMarkup(
      <ComposerMeta
        toolbar={
          <div>
            <button aria-label="Case switcher">Ticket 417</button>
            <button aria-label="More actions">More</button>
          </div>
        }
      />,
    );

    expect(html).toContain('aria-label="Case switcher"');
    expect(html).toContain('aria-label="More actions"');
    expect(html).not.toContain("Shift + Enter for newline");
  });
});

describe("assistant reply typography", () => {
  test("exports assistant reply font size variables for small, default, and large replies", async () => {
    const threadModule = await import("./thread");
    const resolveAssistantReplyFontSizeVars = (threadModule as Record<string, any>)
      .resolveAssistantReplyFontSizeVars;

    expect(typeof resolveAssistantReplyFontSizeVars).toBe("function");

    expect(resolveAssistantReplyFontSizeVars("small")).toMatchObject({
      "--tihc-assistant-body-font-size": "14px",
    });
    expect(resolveAssistantReplyFontSizeVars("default")).toMatchObject({
      "--tihc-assistant-body-font-size": "15px",
    });
    expect(resolveAssistantReplyFontSizeVars("large")).toMatchObject({
      "--tihc-assistant-body-font-size": "16px",
    });
  });
});
