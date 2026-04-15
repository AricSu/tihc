import { describe, expect, test } from "vitest";
import { isPlaceholderCase, listOpenCases, listVisibleCases } from "./case-list";

describe("case list helpers", () => {
  test("treats a ready Default case as a placeholder", () => {
    expect(
      isPlaceholderCase({
        id: "case-placeholder",
        title: "Default",
        activityState: "ready",
        archivedAt: null,
        updatedAt: "2026-04-15T10:00:00.000Z",
      }),
    ).toBe(true);
  });

  test("hides placeholder cases from user-visible lists", () => {
    const cases = [
      {
        id: "case-placeholder",
        title: "Default",
        activityState: "ready" as const,
        archivedAt: null,
        updatedAt: "2026-04-15T10:00:00.000Z",
      },
      {
        id: "case-real",
        title: "Checkout timeout",
        activityState: "active" as const,
        archivedAt: null,
        updatedAt: "2026-04-15T12:00:00.000Z",
      },
    ];

    expect(listVisibleCases(cases).map((item) => item.id)).toEqual(["case-real"]);
  });

  test("keeps placeholder cases in the open-case set for empty sidepanel fallback", () => {
    const cases = [
      {
        id: "case-placeholder",
        title: "Default",
        activityState: "ready" as const,
        archivedAt: null,
        updatedAt: "2026-04-15T10:00:00.000Z",
      },
    ];

    expect(listOpenCases(cases).map((item) => item.id)).toEqual(["case-placeholder"]);
    expect(listVisibleCases(cases)).toEqual([]);
  });
});
