import { describe, expect, test, vi } from "vitest";
import { createRuntimeSync, type RuntimeSyncChannel } from "./runtime-sync";

class MockChannel implements RuntimeSyncChannel {
  static channels = new Map<string, Set<MockChannel>>();

  readonly listeners = new Set<(event: { data: unknown }) => void>();

  constructor(private readonly name: string) {
    if (!MockChannel.channels.has(name)) {
      MockChannel.channels.set(name, new Set());
    }
    MockChannel.channels.get(name)?.add(this);
  }

  close() {
    MockChannel.channels.get(this.name)?.delete(this);
  }

  postMessage(message: unknown) {
    const peers = MockChannel.channels.get(this.name) ?? new Set();
    for (const peer of peers) {
      if (peer === this) continue;
      for (const listener of peer.listeners) {
        listener({ data: message });
      }
      peer.onmessage?.({ data: message });
    }
  }

  addEventListener(type: "message", listener: (event: { data: unknown }) => void) {
    if (type !== "message") return;
    this.listeners.add(listener);
  }

  removeEventListener(type: "message", listener: (event: { data: unknown }) => void) {
    if (type !== "message") return;
    this.listeners.delete(listener);
  }

  onmessage: ((event: { data: unknown }) => void) | null = null;
}

describe("runtime sync", () => {
  test("publishes snapshots across contexts and ignores self echoes", () => {
    let leftSnapshot = { activeCaseId: "case-1" };
    let rightSnapshot = { activeCaseId: "case-1" };
    const applyRight = vi.fn((snapshot: typeof rightSnapshot) => {
      rightSnapshot = snapshot;
    });
    const applyLeft = vi.fn((snapshot: typeof leftSnapshot) => {
      leftSnapshot = snapshot;
    });

    const left = createRuntimeSync({
      applySnapshot: applyLeft,
      channelName: "runtime-sync-test",
      createChannel: (name) => new MockChannel(name),
      createId: () => "left",
      getSnapshot: () => leftSnapshot,
    });
    const right = createRuntimeSync({
      applySnapshot: applyRight,
      channelName: "runtime-sync-test",
      createChannel: (name) => new MockChannel(name),
      createId: () => "right",
      getSnapshot: () => rightSnapshot,
    });

    leftSnapshot = { activeCaseId: "case-2" };
    left.publish();

    expect(applyRight).toHaveBeenCalledWith({ activeCaseId: "case-2" });
    expect(rightSnapshot).toEqual({ activeCaseId: "case-2" });
    expect(applyLeft).not.toHaveBeenCalled();

    left.close();
    right.close();
  });
});
