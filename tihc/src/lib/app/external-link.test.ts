import { beforeEach, describe, expect, test, vi } from "vitest";

const { trackOutboundClickMock } = vi.hoisted(() => ({
  trackOutboundClickMock: vi.fn(),
}));

vi.mock("@/lib/telemetry", () => ({
  trackOutboundClick: trackOutboundClickMock,
}));

describe("external link tracking", () => {
  beforeEach(() => {
    vi.restoreAllMocks();
    trackOutboundClickMock.mockReset();
  });

  test("adds TIHC UTMs and emits an outbound click telemetry event", async () => {
    const windowOpenMock = vi.fn();
    vi.stubGlobal("open", windowOpenMock);

    const { openExternalUrl } = await import("./external-link");

    openExternalUrl("https://docs.askaric.com/guides/ga4?from=chat");

    expect(trackOutboundClickMock).toHaveBeenCalledWith("https://docs.askaric.com/guides/ga4?from=chat");
    expect(windowOpenMock).toHaveBeenCalledWith(
      expect.stringContaining("utm_source=tihc_extension"),
      "_blank",
      "noopener,noreferrer",
    );
    expect(windowOpenMock).toHaveBeenCalledWith(
      expect.stringContaining("utm_campaign=tihc_referral"),
      "_blank",
      "noopener,noreferrer",
    );
  });
});
