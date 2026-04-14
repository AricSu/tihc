import { describe, expect, test, vi } from "vitest";

import {
  refreshGoogleAuthSession,
  resolveGoogleOAuthClientId,
  revokeGoogleAuthToken,
  signInWithGoogle,
} from "./google-oauth";

describe("google oauth", () => {
  test("prefers the Firefox client id when the browser is Firefox", () => {
    expect(
      resolveGoogleOAuthClientId(
        {
          chromeClientId: "chrome-client-id",
          firefoxClientId: "firefox-client-id",
        },
        "Mozilla/5.0 Firefox/126.0",
      ),
    ).toBe("firefox-client-id");
  });

  test("falls back to the Chrome client id outside Firefox", () => {
    expect(
      resolveGoogleOAuthClientId(
        {
          chromeClientId: "chrome-client-id",
          firefoxClientId: "firefox-client-id",
        },
        "Mozilla/5.0 Chrome/136.0.0.0 Safari/537.36",
      ),
    ).toBe("chrome-client-id");
  });

  test("completes the browser OAuth flow and returns a persisted auth session", async () => {
    const browserApi = {
      getRedirectURL: vi.fn(() => "https://extension-id.chromiumapp.org/provider_cb"),
      launchWebAuthFlow: vi.fn(async () =>
        "https://extension-id.chromiumapp.org/provider_cb?code=oauth-code&state=state-123",
      ),
    };
    const fetchImpl = vi.fn(async (input: RequestInfo | URL) => {
      const url = String(input);
      if (url.includes("oauth2.googleapis.com/token?") || url.endsWith("oauth2.googleapis.com/token")) {
        return Response.json({
          access_token: "google-access-token",
          expires_in: 3600,
          token_type: "Bearer",
        });
      }
      return Response.json({
        aud: "chrome-client-id",
        email: "dev@example.com",
        hd: "example.com",
      });
    });

    const session = await signInWithGoogle({
      browserApi,
      config: {
        chromeClientId: "chrome-client-id",
      },
      fetchImpl,
      now: () => new Date("2026-04-14T15:00:00.000Z").getTime(),
      stateFactory: () => "state-123",
      userAgent: "Mozilla/5.0 Chrome/136.0.0.0 Safari/537.36",
    });

    expect(browserApi.launchWebAuthFlow).toHaveBeenCalledTimes(1);
    expect(session).toMatchObject({
      accessToken: "google-access-token",
      clientId: "chrome-client-id",
      email: "dev@example.com",
      hostedDomain: "example.com",
      expiresAt: "2026-04-14T16:00:00.000Z",
    });
  });

  test("refreshes the session silently before falling back to an interactive auth flow", async () => {
    const launchWebAuthFlow = vi
      .fn()
      .mockRejectedValueOnce(new Error("Interactive login required"))
      .mockResolvedValueOnce("https://extension-id.chromiumapp.org/provider_cb?code=oauth-code&state=state-123");
    const browserApi = {
      getRedirectURL: vi.fn(() => "https://extension-id.chromiumapp.org/provider_cb"),
      launchWebAuthFlow,
    };
    const fetchImpl = vi.fn(async (input: RequestInfo | URL) => {
      const url = String(input);
      if (url.includes("oauth2.googleapis.com/token?") || url.endsWith("oauth2.googleapis.com/token")) {
        return Response.json({
          access_token: "refreshed-access-token",
          expires_in: 1800,
          token_type: "Bearer",
        });
      }
      return Response.json({
        aud: "chrome-client-id",
        email: "dev@example.com",
        hd: "example.com",
      });
    });

    const session = await refreshGoogleAuthSession({
      browserApi,
      config: {
        chromeClientId: "chrome-client-id",
      },
      fetchImpl,
      now: () => new Date("2026-04-14T15:00:00.000Z").getTime(),
      stateFactory: () => "state-123",
      userAgent: "Mozilla/5.0 Chrome/136.0.0.0 Safari/537.36",
    });

    expect(launchWebAuthFlow).toHaveBeenNthCalledWith(1, expect.objectContaining({ interactive: false }));
    expect(launchWebAuthFlow).toHaveBeenNthCalledWith(2, expect.objectContaining({ interactive: true }));
    expect(session.accessToken).toBe("refreshed-access-token");
  });

  test("revokes a token through Google's revoke endpoint", async () => {
    const fetchImpl = vi.fn(async () => new Response(null, { status: 200 }));

    await revokeGoogleAuthToken("google-access-token", fetchImpl);

    expect(fetchImpl).toHaveBeenCalledWith(
      "https://oauth2.googleapis.com/revoke?token=google-access-token",
      expect.objectContaining({
        method: "POST",
      }),
    );
  });
});
