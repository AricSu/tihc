import { describe, expect, test, vi } from "vitest";

import {
  extractBearerToken,
  verifyGoogleToken,
} from "./google-auth";

describe("google auth helpers", () => {
  test("extracts bearer tokens case-insensitively", () => {
    expect(
      extractBearerToken(
        new Headers({
          Authorization: "Bearer abc123",
        }),
      ),
    ).toBe("abc123");
  });

  test("rejects audience mismatches", async () => {
    const fetchImpl = vi.fn(async () =>
      Response.json({
        aud: "wrong-client-id",
        email: "dev@example.com",
        hd: "example.com",
      }),
    );

    await expect(
      verifyGoogleToken({
        expectedAudience: "correct-client-id",
        expectedWorkspaceDomain: "example.com",
        fetchImpl,
        token: "token",
      }),
    ).rejects.toThrow(/audience mismatch/i);
  });

  test("rejects workspace domain mismatches", async () => {
    const fetchImpl = vi.fn(async () =>
      Response.json({
        aud: "client-id",
        email: "dev@other.com",
        hd: "other.com",
      }),
    );

    await expect(
      verifyGoogleToken({
        expectedAudience: "client-id",
        expectedWorkspaceDomain: "example.com",
        fetchImpl,
        token: "token",
      }),
    ).rejects.toThrow(/workspace domain/i);
  });

  test("accepts matching access tokens", async () => {
    const fetchImpl = vi.fn(async () =>
      Response.json({
        aud: "client-id",
        email: "dev@example.com",
        hd: "example.com",
      }),
    );

    await expect(
      verifyGoogleToken({
        expectedAudience: "client-id",
        expectedWorkspaceDomain: "example.com",
        fetchImpl,
        token: "token",
      }),
    ).resolves.toMatchObject({
      aud: "client-id",
      email: "dev@example.com",
    });
  });
});
