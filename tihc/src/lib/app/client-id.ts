let memoryClientId: string | null = null;

function createId(): string {
  if (typeof globalThis.crypto?.randomUUID === "function") {
    return globalThis.crypto.randomUUID();
  }
  return `client-${Date.now()}-${Math.random().toString(16).slice(2, 10)}`;
}

export async function getAppClientId(): Promise<string> {
  if (memoryClientId) return memoryClientId;
  memoryClientId = createId();
  return memoryClientId;
}
