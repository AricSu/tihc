export type RuntimeSyncChannel = {
  close?: () => void;
  postMessage: (message: unknown) => void;
  addEventListener?: (
    type: "message",
    listener: (event: { data: unknown }) => void,
  ) => void;
  removeEventListener?: (
    type: "message",
    listener: (event: { data: unknown }) => void,
  ) => void;
  onmessage?: ((event: { data: unknown }) => void) | null;
};

type RuntimeSyncMessage<T> = {
  senderId: string;
  snapshot: T;
  type: "snapshot";
};

function createRandomId(): string {
  if (typeof globalThis.crypto?.randomUUID === "function") {
    return globalThis.crypto.randomUUID();
  }
  return `runtime-${Date.now()}-${Math.random().toString(16).slice(2, 10)}`;
}

function createBroadcastChannel(name: string): RuntimeSyncChannel | null {
  try {
    if (typeof globalThis.BroadcastChannel !== "function") return null;
    return new globalThis.BroadcastChannel(name) as RuntimeSyncChannel;
  } catch {
    return null;
  }
}

export function createRuntimeSync<T>({
  applySnapshot,
  channelName,
  createChannel = createBroadcastChannel,
  createId = createRandomId,
  getSnapshot,
}: {
  applySnapshot: (snapshot: T) => void;
  channelName: string;
  createChannel?: (name: string) => RuntimeSyncChannel | null;
  createId?: () => string;
  getSnapshot: () => T;
}) {
  const senderId = createId();
  const channel = createChannel(channelName);

  const handleMessage = (event: { data: unknown }) => {
    const message = event.data as Partial<RuntimeSyncMessage<T>> | null;
    if (!message || message.type !== "snapshot" || !("snapshot" in message)) return;
    if (message.senderId === senderId) return;
    applySnapshot(message.snapshot as T);
  };

  if (channel?.addEventListener) {
    channel.addEventListener("message", handleMessage);
  } else if (channel) {
    channel.onmessage = handleMessage;
  }

  return {
    close() {
      if (channel?.removeEventListener) {
        channel.removeEventListener("message", handleMessage);
      } else if (channel) {
        channel.onmessage = null;
      }
      channel?.close?.();
    },
    publish() {
      channel?.postMessage({
        senderId,
        snapshot: getSnapshot(),
        type: "snapshot",
      } satisfies RuntimeSyncMessage<T>);
    },
  };
}
