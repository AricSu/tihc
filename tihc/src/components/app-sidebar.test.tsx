import { act } from "react";
import { createRoot } from "react-dom/client";
import { beforeEach, describe, expect, test, vi } from "vitest";
import { AppSidebar } from "./app-sidebar";
import { SidebarProvider } from "@/components/ui/sidebar";

(globalThis as typeof globalThis & { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT =
  true;

Object.defineProperty(window, "matchMedia", {
  configurable: true,
  writable: true,
  value: vi.fn().mockImplementation((query: string) => ({
    matches: false,
    media: query,
    onchange: null,
    addEventListener: vi.fn(),
    removeEventListener: vi.fn(),
    addListener: vi.fn(),
    removeListener: vi.fn(),
    dispatchEvent: vi.fn(),
  })),
});

class ResizeObserverMock {
  observe() {}
  unobserve() {}
  disconnect() {}
}

Object.defineProperty(globalThis, "ResizeObserver", {
  configurable: true,
  writable: true,
  value: ResizeObserverMock,
});

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

describe("AppSidebar", () => {
  beforeEach(() => {
    document.body.innerHTML = "";
  });

  test("opens a settings sheet with a reply font size slider", async () => {
    const onAssistantReplyFontSizeChange = vi.fn();
    const cleanup = await renderInDom(
      <SidebarProvider>
        <AppSidebar
          assistantReplyFontSize="default"
          caseItems={[
            {
              id: "case-1",
              title: "Primary case",
              status: "Investigating",
              updatedAt: "2026-04-15T10:00:00.000Z",
            },
          ]}
          onAssistantReplyFontSizeChange={onAssistantReplyFontSizeChange}
        />
      </SidebarProvider>,
    );

    const settingsTrigger = Array.from(document.querySelectorAll("button, a")).find((element) =>
      element.textContent?.trim() === "Settings",
    );
    expect(settingsTrigger).toBeTruthy();

    await act(async () => {
      settingsTrigger?.dispatchEvent(new MouseEvent("click", { bubbles: true, cancelable: true }));
      await Promise.resolve();
    });

    expect(document.body.textContent).toContain("Sidebar Settings");
    expect(document.body.textContent).toContain("Reply font size");
    expect(document.querySelector('[data-slot="slider"]')).toBeTruthy();

    const sliderThumb = document.querySelector('[data-slot="slider-thumb"]');
    expect(sliderThumb).toBeTruthy();

    await act(async () => {
      (sliderThumb as HTMLElement | null)?.focus();
      sliderThumb?.dispatchEvent(new KeyboardEvent("keydown", { key: "ArrowRight", bubbles: true }));
      await Promise.resolve();
    });

    expect(onAssistantReplyFontSizeChange).toHaveBeenCalledWith("large");

    await cleanup();
  });
});
