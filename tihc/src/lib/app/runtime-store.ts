import type { AppRuntimeSettings } from "@/lib/chat/agent-types";
import {
  clonePublicSettings,
  createInitialRuntimeState,
  type RuntimeState,
} from "@/lib/app/runtime-state";

export type RuntimeStore = {
  readonly state: RuntimeState;
  assign(next: Partial<RuntimeState>): void;
  commit(): void;
  getPublicSettings(): AppRuntimeSettings;
  getSnapshot(): AppRuntimeSettings;
  subscribe(listener: () => void): () => void;
};

export function createRuntimeStore(initialState: RuntimeState = createInitialRuntimeState()): RuntimeStore {
  const listeners = new Set<() => void>();
  const state = initialState;
  let snapshot = clonePublicSettings(state);

  const refreshSnapshot = () => {
    snapshot = clonePublicSettings(state);
  };

  const emitChange = () => {
    for (const listener of listeners) {
      listener();
    }
  };

  return {
    state,
    assign(next) {
      Object.assign(state, next);
    },
    commit() {
      refreshSnapshot();
      emitChange();
    },
    getPublicSettings() {
      return clonePublicSettings(state);
    },
    getSnapshot() {
      return snapshot;
    },
    subscribe(listener) {
      listeners.add(listener);
      return () => {
        listeners.delete(listener);
      };
    },
  };
}
