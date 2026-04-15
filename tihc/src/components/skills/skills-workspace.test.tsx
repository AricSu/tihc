import { act } from "react";
import { createRoot } from "react-dom/client";
import { beforeEach, describe, expect, test, vi } from "vitest";
import { SkillsWorkspace } from "./skills-workspace";

(globalThis as typeof globalThis & { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT =
  true;

async function renderInDom(element: React.ReactNode): Promise<() => Promise<void>> {
  const container = document.createElement("div");
  document.body.append(container);
  const root = createRoot(container);

  await act(async () => {
    root.render(element);
  });

  return async () => {
    await act(async () => {
      root.unmount();
    });
    container.remove();
  };
}

describe("SkillsWorkspace", () => {
  beforeEach(() => {
    window.history.replaceState({}, "", "?section=skills&editor=new");
    vi.stubGlobal("confirm", vi.fn(() => true));
  });

  async function createSkill() {
    const editor = document.querySelector("textarea[aria-label='Markdown editor']");
    expect(editor).toBeInstanceOf(HTMLTextAreaElement);

    await act(async () => {
      if (editor instanceof HTMLTextAreaElement) {
        const descriptor = Object.getOwnPropertyDescriptor(HTMLTextAreaElement.prototype, "value");
        descriptor?.set?.call(
          editor,
          [
            "---",
            "name: Briefing Writer",
            "description: Generates structured briefings.",
            "---",
            "# Briefing Writer",
            "Focus on concise updates.",
          ].join("\n"),
        );
        editor.dispatchEvent(new Event("input", { bubbles: true }));
        editor.dispatchEvent(new Event("change", { bubbles: true }));
      }
      await Promise.resolve();
    });

    const createSkillButton = Array.from(document.querySelectorAll("button")).find(
      (button) => button.textContent?.trim() === "Create skill",
    );
    expect(createSkillButton).toBeTruthy();

    await act(async () => {
      createSkillButton?.dispatchEvent(new MouseEvent("click", { bubbles: true, cancelable: true }));
      await Promise.resolve();
    });
  }

  test("deletes a saved skill from the editor and returns to the empty library", async () => {
    vi.useFakeTimers();

    try {
      vi.setSystemTime(new Date("2026-04-15T12:34:56.789Z"));
      const cleanup = await renderInDom(<SkillsWorkspace />);
      await createSkill();

      const deleteSkillButton = Array.from(document.querySelectorAll("button")).find(
        (button) => button.textContent?.trim() === "Delete skill",
      );
      expect(deleteSkillButton).toBeTruthy();

      await act(async () => {
        deleteSkillButton?.dispatchEvent(new MouseEvent("click", { bubbles: true, cancelable: true }));
        await Promise.resolve();
      });

      expect(window.location.search).toBe("?section=skills");
      expect(document.body.textContent).toContain("No skills yet");
      expect(document.body.textContent).not.toContain("Briefing Writer");

      await cleanup();
    } finally {
      vi.useRealTimers();
    }
  });

  test("deletes a saved skill from the library card", async () => {
    vi.useFakeTimers();

    try {
      vi.setSystemTime(new Date("2026-04-15T12:34:56.789Z"));
      const cleanup = await renderInDom(<SkillsWorkspace />);
      await createSkill();

      const backButton = Array.from(document.querySelectorAll("button")).find(
        (button) => button.textContent?.trim() === "Back to Knowledge",
      );
      expect(backButton).toBeTruthy();

      await act(async () => {
        backButton?.dispatchEvent(new MouseEvent("click", { bubbles: true, cancelable: true }));
        await Promise.resolve();
      });

      expect(document.body.textContent).toContain("Briefing Writer");

      const deleteButton = Array.from(document.querySelectorAll("button")).find(
        (button) => button.textContent?.trim() === "Delete",
      );
      expect(deleteButton).toBeTruthy();

      await act(async () => {
        deleteButton?.dispatchEvent(new MouseEvent("click", { bubbles: true, cancelable: true }));
        await Promise.resolve();
      });

      expect(window.location.search).toBe("?section=skills");
      expect(document.body.textContent).toContain("No skills yet");
      expect(document.body.textContent).not.toContain("Briefing Writer");

      await cleanup();
    } finally {
      vi.useRealTimers();
    }
  });
});
