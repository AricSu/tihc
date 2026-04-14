export function resolveJsonPath(payload: unknown, path: string): unknown {
  const normalized = path.trim();
  if (!normalized) return undefined;

  const segments = normalized.split(".").filter(Boolean);
  let current: unknown = payload;

  for (const segment of segments) {
    if (current == null) return undefined;
    if (Array.isArray(current)) {
      const index = Number(segment);
      if (!Number.isInteger(index)) return undefined;
      current = current[index];
      continue;
    }
    if (typeof current !== "object") return undefined;
    current = (current as Record<string, unknown>)[segment];
  }

  return current;
}
