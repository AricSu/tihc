import * as React from "react";
import { act } from "react";
import { createRoot } from "react-dom/client";
import { beforeEach, describe, expect, test, vi } from "vitest";
import { AppSidebar } from "./app-sidebar";
import { SidebarProvider } from "@/components/ui/sidebar";
import type { GoogleAuthState } from "@/lib/chat/agent-types";

const {
  clearGoogleAuthMock,
  deleteCaseMock,
  renameCaseMock,
  refreshGoogleAuthMock,
  setGoogleAuthMock,
} = vi.hoisted(() => ({
  clearGoogleAuthMock: vi.fn(),
  deleteCaseMock: vi.fn(),
  renameCaseMock: vi.fn(),
  refreshGoogleAuthMock: vi.fn(),
  setGoogleAuthMock: vi.fn(),
}));

const {
  refreshGoogleAuthSessionMock,
  signInWithGoogleMock,
  signOutFromGoogleMock,
} = vi.hoisted(() => ({
  refreshGoogleAuthSessionMock: vi.fn(),
  signInWithGoogleMock: vi.fn(),
  signOutFromGoogleMock: vi.fn(),
}));

vi.mock("@/lib/app/runtime", () => ({
  clearGoogleAuth: clearGoogleAuthMock,
  deleteCase: deleteCaseMock,
  renameCase: renameCaseMock,
  refreshGoogleAuth: refreshGoogleAuthMock,
  setGoogleAuth: setGoogleAuthMock,
}));

vi.mock("@/components/ui/context-menu", () => {
  const Context = React.createContext(false)
  const SetContext = React.createContext<((open: boolean) => void) | null>(null)

  return {
    ContextMenu: ({ children }: { children: React.ReactNode }) => {
      const [open, setOpen] = React.useState(false)
      return (
        <SetContext.Provider value={setOpen}>
          <Context.Provider value={open}>{children}</Context.Provider>
        </SetContext.Provider>
      )
    },
    ContextMenuTrigger: ({ children }: { children: React.ReactNode }) => {
      const setOpen = React.useContext(SetContext)
      return (
        <div
          onContextMenu={(event) => {
            event.preventDefault()
            setOpen?.(true)
          }}
        >
          {children}
        </div>
      )
    },
    ContextMenuContent: ({ children }: { children: React.ReactNode }) => {
      const open = React.useContext(Context)
      return open ? <div>{children}</div> : null
    },
  ContextMenuItem: ({
    children,
    onSelect,
  }: {
    children: React.ReactNode;
    onSelect?: (event: { preventDefault: () => void }) => void;
  }) => (
    <button
      type="button"
      onClick={() =>
        onSelect?.({
          preventDefault() {},
        })
      }
    >
      {children}
    </button>
    ),
  }
});

vi.mock("@/lib/auth/google-oauth", () => ({
  isGoogleOAuthConfigured: vi.fn(() => true),
  refreshGoogleAuthSession: refreshGoogleAuthSessionMock,
  signInWithGoogle: signInWithGoogleMock,
  signOutFromGoogle: signOutFromGoogleMock,
}));

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

class PointerEventMock extends MouseEvent {}

Object.defineProperty(globalThis, "PointerEvent", {
  configurable: true,
  writable: true,
  value: PointerEventMock,
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

async function clickByText(text: string) {
  const target = Array.from(document.querySelectorAll('button, a, [role="menuitem"]')).find((element) =>
    element.textContent?.includes(text),
  );

  expect(target).toBeTruthy();

  await act(async () => {
    target?.dispatchEvent(new PointerEvent("pointerdown", { bubbles: true, cancelable: true }));
    target?.dispatchEvent(new MouseEvent("mousedown", { bubbles: true }));
    target?.dispatchEvent(new MouseEvent("mouseup", { bubbles: true }));
    target?.dispatchEvent(new MouseEvent("click", { bubbles: true, cancelable: true }));
    await Promise.resolve();
  });
}

function buildGoogleAuth(): GoogleAuthState {
  return {
    accessToken: "google-token",
    clientId: "google-client-id",
    email: "alice.smith@example.com",
    hostedDomain: "example.com",
    expiresAt: "2026-04-15T18:00:00.000Z",
  };
}

describe("AppSidebar", () => {
  beforeEach(() => {
    document.body.innerHTML = "";
    vi.clearAllMocks();
  });

  test("renders the tihc brand and a docs link in the sidebar", async () => {
    const cleanup = await renderInDom(
      <SidebarProvider>
        <AppSidebar />
      </SidebarProvider>,
    );

    expect(document.body.textContent).toContain("tihc");
    expect(document.body.textContent).not.toContain("Acme Inc.");

    const docsLink = Array.from(document.querySelectorAll("a")).find(
      (element) => element.textContent?.trim() === "Docs",
    );
    const dashboardLink = Array.from(document.querySelectorAll("a")).find(
      (element) => element.textContent?.trim() === "Dashboard",
    );
    const settingsTrigger = Array.from(document.querySelectorAll("button, a")).find(
      (element) => element.textContent?.trim() === "Settings",
    );

    expect(dashboardLink).toBeTruthy();
    expect(docsLink).toBeTruthy();
    expect(docsLink?.getAttribute("href")).toBe("https://www.askaric.com/en/tihc");
    expect(docsLink?.getAttribute("target")).toBe("_blank");
    expect(docsLink?.getAttribute("rel")).toContain("noreferrer");
    expect(settingsTrigger).toBeTruthy();
    expect(
      dashboardLink!.compareDocumentPosition(docsLink as Node) & Node.DOCUMENT_POSITION_FOLLOWING,
    ).not.toBe(0);
    expect(
      docsLink!.compareDocumentPosition(settingsTrigger as Node) & Node.DOCUMENT_POSITION_FOLLOWING,
    ).not.toBe(0);

    await cleanup();
  });

  test("keeps Settings grouped with Get Help in the bottom utility section", async () => {
    const cleanup = await renderInDom(
      <SidebarProvider>
        <AppSidebar />
      </SidebarProvider>,
    );

    const settingsTrigger = Array.from(document.querySelectorAll("button, a")).find(
      (element) => element.textContent?.trim() === "Settings",
    );
    const getHelpLink = Array.from(document.querySelectorAll("a")).find(
      (element) => element.textContent?.trim() === "Get Help",
    );

    expect(settingsTrigger).toBeTruthy();
    expect(getHelpLink).toBeTruthy();
    expect(settingsTrigger?.closest('[data-slot="sidebar-group"]')).toBe(
      getHelpLink?.closest('[data-slot="sidebar-group"]'),
    );

    await cleanup();
  });

  test("renders the full cases list in descending updated order", async () => {
    const cleanup = await renderInDom(
      <SidebarProvider>
        <AppSidebar
          caseItems={[
            {
              id: "case-1",
              title: "Oldest case",
              status: "Resolved",
              updatedAt: "2026-04-15T08:00:00.000Z",
            },
            {
              id: "case-2",
              title: "Newest case",
              status: "Investigating",
              updatedAt: "2026-04-15T12:00:00.000Z",
            },
            {
              id: "case-3",
              title: "Second newest case",
              status: "Watching",
              updatedAt: "2026-04-15T11:00:00.000Z",
            },
            {
              id: "case-4",
              title: "Case 4",
              status: "Watching",
              updatedAt: "2026-04-15T10:00:00.000Z",
            },
            {
              id: "case-5",
              title: "Case 5",
              status: "Watching",
              updatedAt: "2026-04-15T09:30:00.000Z",
            },
            {
              id: "case-6",
              title: "Case 6",
              status: "Watching",
              updatedAt: "2026-04-15T09:00:00.000Z",
            },
            {
              id: "case-7",
              title: "Case 7",
              status: "Watching",
              updatedAt: "2026-04-15T08:30:00.000Z",
            },
          ]}
        />
      </SidebarProvider>,
    );

    const caseLinks = Array.from(document.querySelectorAll('a[href^="#case-"]')).map((element) =>
      element.textContent?.trim(),
    );

    expect(caseLinks).toEqual([
      "Newest case",
      "Second newest case",
      "Case 4",
      "Case 5",
      "Case 6",
      "Case 7",
      "Oldest case",
    ]);

    await cleanup();
  });

  test("does not render case status badges in the sidebar", async () => {
    const cleanup = await renderInDom(
      <SidebarProvider>
        <AppSidebar
          caseItems={[
            {
              id: "case-1",
              title: "Primary case",
              status: "Investigating",
              updatedAt: "2026-04-15T12:00:00.000Z",
            },
            {
              id: "case-2",
              title: "Follow-up case",
              status: "Resolved",
              updatedAt: "2026-04-15T11:00:00.000Z",
            },
          ]}
        />
      </SidebarProvider>,
    );

    expect(document.body.textContent).toContain("Primary case");
    expect(document.body.textContent).toContain("Follow-up case");
    expect(document.querySelectorAll('[data-slot="sidebar-menu-badge"]')).toHaveLength(0);
    expect(document.body.textContent).not.toContain("Investigating");
    expect(document.body.textContent).not.toContain("Resolved");

    await cleanup();
  });

  test("selects a case from the sidebar list", async () => {
    const onSelectCase = vi.fn();
    const cleanup = await renderInDom(
      <SidebarProvider>
        <AppSidebar
          activeCaseId="case-1"
          caseItems={[
            {
              id: "case-1",
              title: "Current case",
              status: "Investigating",
              updatedAt: "2026-04-15T12:00:00.000Z",
            },
            {
              id: "case-2",
              title: "Other case",
              status: "Watching",
              updatedAt: "2026-04-15T11:00:00.000Z",
            },
          ]}
          onSelectCase={onSelectCase}
        />
      </SidebarProvider>,
    );

    await clickByText("Other case");

    expect(onSelectCase).toHaveBeenCalledWith("case-2");

    await cleanup();
  });

  test("shows a delete action on right click and deletes only after clicking it", async () => {
    const cleanup = await renderInDom(
      <SidebarProvider>
        <AppSidebar
          caseItems={[
            {
              id: "case-1",
              title: "Delete me",
              status: "Investigating",
              updatedAt: "2026-04-15T12:00:00.000Z",
            },
          ]}
        />
      </SidebarProvider>,
    );

    expect(deleteCaseMock).not.toHaveBeenCalled();
    expect(document.body.textContent).not.toContain("Delete case");

    const caseLink = document.querySelector('a[href="#case-case-1"]');
    expect(caseLink).toBeTruthy();

    await act(async () => {
      caseLink?.dispatchEvent(new MouseEvent("contextmenu", { bubbles: true, cancelable: true }));
      await Promise.resolve();
    });

    expect(document.body.textContent).toContain("Delete case");

    await clickByText("Delete case");

    expect(deleteCaseMock).toHaveBeenCalledWith("case-1");

    await cleanup();
  });

  test("renames a case from the sidebar context menu", async () => {
    const cleanup = await renderInDom(
      <SidebarProvider>
        <AppSidebar
          caseItems={[
            {
              id: "case-1",
              title: "Rename me",
              status: "Investigating",
              updatedAt: "2026-04-15T12:00:00.000Z",
            },
          ]}
        />
      </SidebarProvider>,
    );

    const caseLink = document.querySelector('a[href="#case-case-1"]');
    expect(caseLink).toBeTruthy();

    await act(async () => {
      caseLink?.dispatchEvent(new MouseEvent("contextmenu", { bubbles: true, cancelable: true }));
      await Promise.resolve();
    });

    await clickByText("Rename case");

    const input = document.querySelector('input[aria-label="Case name"]') as HTMLInputElement | null;
    expect(input).toBeTruthy();

    await act(async () => {
      if (input) {
        const descriptor = Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, "value");
        descriptor?.set?.call(input, "Renamed case");
        input.dispatchEvent(new Event("input", { bubbles: true }));
      }
      await Promise.resolve();
    });

    const renameButton = Array.from(document.querySelectorAll("button")).find(
      (button) => button.textContent?.trim() === "Rename",
    );
    expect(renameButton).toBeTruthy();

    await act(async () => {
      renameButton?.dispatchEvent(new MouseEvent("click", { bubbles: true, cancelable: true }));
      await Promise.resolve();
    });

    expect(renameCaseMock).toHaveBeenCalledWith("case-1", "Renamed case");

    await cleanup();
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
          currentUser={{
            id: null,
            authState: "anonymous",
            displayName: "匿名",
            email: "",
            hostedDomain: "",
          }}
          googleAuth={null}
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

  test("renders the anonymous user label in the sidebar footer", async () => {
    const cleanup = await renderInDom(
      <SidebarProvider>
        <AppSidebar
          currentUser={{
            id: null,
            authState: "anonymous",
            displayName: "匿名",
            email: "",
            hostedDomain: "",
          }}
          googleAuth={null}
        />
      </SidebarProvider>,
    );

    expect(document.body.textContent).toContain("匿名");

    await cleanup();
  });

  test("opens a login dialog for anonymous users and signs in with Google", async () => {
    signInWithGoogleMock.mockResolvedValue(buildGoogleAuth());

    const cleanup = await renderInDom(
      <SidebarProvider>
        <AppSidebar
          currentUser={{
            id: null,
            authState: "anonymous",
            displayName: "匿名",
            email: "",
            hostedDomain: "",
          }}
          googleAuth={null}
        />
      </SidebarProvider>,
    );

    await clickByText("匿名");

    expect(document.body.textContent).toContain("Sign in to unlock full features");
    expect(document.body.textContent).toContain("Cloud sync");
    expect(document.body.textContent).toContain("Usage analytics");
    expect(document.body.textContent).toContain("Personal LLM settings");

    await clickByText("Sign in with Google");

    expect(signInWithGoogleMock).toHaveBeenCalledTimes(1);
    expect(setGoogleAuthMock).toHaveBeenCalledWith(buildGoogleAuth());

    await cleanup();
  });

  test("shows refresh and sign-out actions for authenticated users", async () => {
    refreshGoogleAuthSessionMock.mockResolvedValue(buildGoogleAuth());

    const cleanup = await renderInDom(
      <SidebarProvider>
        <AppSidebar
          currentUser={{
            id: "principal-1",
            authState: "authenticated",
            displayName: "Alice Smith",
            email: "alice.smith@example.com",
            hostedDomain: "example.com",
          }}
          googleAuth={buildGoogleAuth()}
        />
      </SidebarProvider>,
    );

    await clickByText("Alice Smith");

    expect(document.body.textContent).toContain("Refresh Google Token");
    expect(document.body.textContent).toContain("Sign out");
    expect(document.body.textContent).not.toContain("Account");
    expect(document.body.textContent).not.toContain("Billing");
    expect(document.body.textContent).not.toContain("Notifications");

    await clickByText("Refresh Google Token");

    expect(refreshGoogleAuthSessionMock).toHaveBeenCalledTimes(1);
    expect(refreshGoogleAuthMock).toHaveBeenCalledWith(buildGoogleAuth());

    await cleanup();
  });

  test("signs out the authenticated user from the sidebar footer", async () => {
    const googleAuth = buildGoogleAuth();
    const cleanup = await renderInDom(
      <SidebarProvider>
        <AppSidebar
          currentUser={{
            id: "principal-1",
            authState: "authenticated",
            displayName: "Alice Smith",
            email: "alice.smith@example.com",
            hostedDomain: "example.com",
          }}
          googleAuth={googleAuth}
        />
      </SidebarProvider>,
    );

    await clickByText("Alice Smith");
    await clickByText("Sign out");

    expect(signOutFromGoogleMock).toHaveBeenCalledWith("google-token");
    expect(clearGoogleAuthMock).toHaveBeenCalledTimes(1);

    await cleanup();
  });
});
