import { describe, expect, test } from "vitest";

import { parseUpstreamFrame } from "./upstream-protocol";

describe("parseUpstreamFrame", () => {
  test("parses delta frames into answer deltas", () => {
    expect(parseUpstreamFrame('0:{"delta":"Hello"}')).toEqual([
      { text: "Hello", type: "answer-delta" },
    ]);
  });

  test("parses snapshot completion frames", () => {
    expect(
      parseUpstreamFrame(
        '2:[{"assistant_message":{"content":"Hello world","finished_at":"2026-04-14T12:00:00Z"}}]',
      ),
    ).toEqual([
      { done: true, text: "Hello world", type: "answer-snapshot" },
    ]);
  });

  test("parses retrieval progress events into ordinary text chunks", () => {
    expect(
      parseUpstreamFrame(
        '8:[{"display":"kb-main","state":"RUNNING","message":"warming cache"}]',
      ),
    ).toEqual([
      { text: "Retrieving: kb-main\n", type: "progress-text" },
      { text: "Retrieval state: RUNNING\n", type: "progress-text" },
      { text: "Retrieval message: warming cache\n", type: "progress-text" },
    ]);
  });

  test("treats malformed upstream frames as plain text output", () => {
    expect(parseUpstreamFrame("Upstream stream error: socket closed")).toEqual([
      { text: "Upstream stream error: socket closed", type: "plain-text" },
    ]);
  });
});
