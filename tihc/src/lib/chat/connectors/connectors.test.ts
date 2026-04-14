import { describe, expect, test } from "vitest";

import { getConnectorForTemplate } from "./registry";
import { resolveJsonPath } from "./json-path";
import {
  parseConfiguredJsonFrame,
  parseOpenAICompatibleFrame,
} from "./parsers";

describe("connector helpers", () => {
  test("resolves nested array paths from JSON payloads", () => {
    const payload = {
      choices: [
        {
          delta: {
            content: "hello",
          },
        },
      ],
    };

    expect(resolveJsonPath(payload, "choices.0.delta.content")).toBe("hello");
  });

  test("parses OpenAI-compatible delta frames", () => {
    expect(
      parseOpenAICompatibleFrame(
        'data: {"choices":[{"delta":{"content":"hello"},"finish_reason":null}]}',
      ),
    ).toEqual({
      done: false,
      snapshot: "",
      status: "",
      textDelta: "hello",
    });
  });

  test("parses configured JSON frames with snapshot and done path", () => {
    expect(
      parseConfiguredJsonFrame('data: {"text":"full answer","done":true}', {
        deltaPath: "delta",
        donePath: "done",
        doneSentinel: "[DONE]",
        snapshotPath: "text",
      }),
    ).toEqual({
      done: true,
      snapshot: "full answer",
      status: "",
      textDelta: "",
    });
  });

  test("registers a connector for each supported template", () => {
    expect(getConnectorForTemplate("openai-compatible").id).toBe("openai-compatible");
    expect(getConnectorForTemplate("generic-http-sse").id).toBe("generic-http-sse");
    expect(getConnectorForTemplate("generic-websocket").id).toBe("generic-websocket");
  });
});
