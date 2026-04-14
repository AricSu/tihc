export type LogLevel = "debug" | "info" | "warn" | "error" | "silent";
export type LogFormat = "pretty" | "json";

export type LogFields = Record<string, unknown>;

export type AppLogger = {
  debug(event: string, fields?: LogFields): void;
  info(event: string, fields?: LogFields): void;
  warn(event: string, fields?: LogFields): void;
  error(event: string, fields?: LogFields): void;
};

type LogSink = {
  debug?: (...args: unknown[]) => void;
  info?: (...args: unknown[]) => void;
  warn?: (...args: unknown[]) => void;
  error?: (...args: unknown[]) => void;
};

type CreateLoggerOptions = {
  component?: string;
  format?: string;
  level?: string;
  sink?: LogSink;
};

const LOG_LEVEL_ORDER: Record<LogLevel, number> = {
  debug: 10,
  info: 20,
  warn: 30,
  error: 40,
  silent: 50,
};

function isLogLevel(value: string): value is LogLevel {
  return value === "debug" || value === "info" || value === "warn" || value === "error" || value === "silent";
}

function resolveLogLevel(value: string | undefined, fallback: LogLevel): LogLevel {
  if (!value) return fallback;
  const normalized = value.trim().toLowerCase();
  return isLogLevel(normalized) ? normalized : fallback;
}

function resolveLogFormat(value: string | undefined): LogFormat {
  return value?.trim().toLowerCase() === "json" ? "json" : "pretty";
}

function normalizeFields(fields: LogFields | undefined): LogFields {
  if (!fields) return {};
  return Object.fromEntries(
    Object.entries(fields).filter(([, value]) => value !== undefined),
  );
}

function serializePrettyValue(value: unknown): string {
  if (typeof value === "string") return value;
  if (typeof value === "number" || typeof value === "boolean") return String(value);
  if (value === null) return "null";
  return JSON.stringify(value);
}

function formatPrettyLine(
  level: Exclude<LogLevel, "silent">,
  event: string,
  component: string,
  fields: LogFields,
): string {
  const head = `${new Date().toISOString()} ${level.toUpperCase()} ${component} ${event}`;
  const tail = Object.entries(fields)
    .map(([key, value]) => `${key}=${serializePrettyValue(value)}`)
    .join(" ");
  return tail ? `${head} ${tail}` : head;
}

function formatJsonLine(
  level: Exclude<LogLevel, "silent">,
  event: string,
  component: string,
  fields: LogFields,
): string {
  return JSON.stringify({
    component,
    event,
    level,
    timestamp: new Date().toISOString(),
    ...fields,
  });
}

function emitLine(
  sink: LogSink,
  level: Exclude<LogLevel, "silent">,
  line: string,
): void {
  const writer = sink[level] ?? sink.info ?? console.log;
  writer(line);
}

export function createLogger({
  component = "tihc-backend",
  format,
  level,
  sink = console,
}: CreateLoggerOptions = {}): AppLogger {
  const resolvedLevel = resolveLogLevel(level, "info");
  const resolvedFormat = resolveLogFormat(format);

  const shouldLog = (entryLevel: Exclude<LogLevel, "silent">): boolean =>
    LOG_LEVEL_ORDER[entryLevel] >= LOG_LEVEL_ORDER[resolvedLevel] &&
    resolvedLevel !== "silent";

  const write = (entryLevel: Exclude<LogLevel, "silent">, event: string, fields?: LogFields) => {
    if (!shouldLog(entryLevel)) return;
    const normalizedFields = normalizeFields(fields);
    const line =
      resolvedFormat === "json"
        ? formatJsonLine(entryLevel, event, component, normalizedFields)
        : formatPrettyLine(entryLevel, event, component, normalizedFields);
    emitLine(sink, entryLevel, line);
  };

  return {
    debug(event, fields) {
      write("debug", event, fields);
    },
    info(event, fields) {
      write("info", event, fields);
    },
    warn(event, fields) {
      write("warn", event, fields);
    },
    error(event, fields) {
      write("error", event, fields);
    },
  };
}

export function createNoopLogger(): AppLogger {
  return {
    debug() {},
    info() {},
    warn() {},
    error() {},
  };
}
