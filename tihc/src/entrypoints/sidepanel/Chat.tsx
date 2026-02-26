"use client";

import { Thread } from "@/components/assistant-ui/thread";
import { MLCProvider } from "../../components/MLCProvider";

export default function Chat() {
  return (
    <MLCProvider>
      <div className="flex h-full flex-col bg-white text-slate-900">
        <Thread />
      </div>
    </MLCProvider>
  );
}
