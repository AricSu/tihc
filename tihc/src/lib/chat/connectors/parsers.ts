import { resolveJsonPath } from "./json-path";

export type ParsedFrame = {
  textDelta: string;
  snapshot: string;
  status: string;
  done: boolean;
};

type ConfiguredFrameOptions = {
  deltaPath: string;
  snapshotPath: string;
  donePath: string;
  doneSentinel: string;
};

function emptyFrame(): ParsedFrame {
  return {
    done: false,
    snapshot: "",
    status: "",
    textDelta: "",
  };
}

function parseJson(raw: string): unknown {
  try {
    return JSON.parse(raw);
  } catch {
    return raw;
  }
}

function stripDataPrefix(frame: string): string {
  return frame
    .split("\n")
    .map((line) => line.trim())
    .filter(Boolean)
    .map((line) => line.replace(/^data:\s*/i, ""))
    .join("\n")
    .trim();
}

function toStringValue(value: unknown): string {
  return typeof value === "string" ? value : "";
}

export function parseOpenAICompatibleFrame(frame: string): ParsedFrame {
  const normalized = stripDataPrefix(frame);
  if (!normalized) return emptyFrame();
  if (normalized === "[DONE]") {
    return {
      ...emptyFrame(),
      done: true,
    };
  }

  const payload = parseJson(normalized);
  return {
    done: false,
    snapshot: toStringValue(resolveJsonPath(payload, "choices.0.message.content")),
    status: "",
    textDelta: toStringValue(resolveJsonPath(payload, "choices.0.delta.content")),
  };
}

export function parseConfiguredJsonFrame(
  frame: string,
  options: ConfiguredFrameOptions,
): ParsedFrame {
  const normalized = stripDataPrefix(frame);
  if (!normalized) return emptyFrame();
  if (normalized === options.doneSentinel) {
    return {
      ...emptyFrame(),
      done: true,
    };
  }

  const payload = parseJson(normalized);
  if (typeof payload === "string") {
    return {
      done: normalized === options.doneSentinel,
      snapshot: "",
      status: "",
      textDelta: payload,
    };
  }

  const doneValue = options.donePath ? resolveJsonPath(payload, options.donePath) : undefined;
  return {
    done: doneValue === true,
    snapshot: toStringValue(resolveJsonPath(payload, options.snapshotPath)),
    status: "",
    textDelta: toStringValue(resolveJsonPath(payload, options.deltaPath)),
  };
}
