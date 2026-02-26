import type { IntakeDecision, IntakeSessionState, IntakeTraceRecord } from "./types";
import { decideIntake } from "./gate";

export type IntakeRuntimeSettings = {
  enabled: boolean;
  turnBudget: number;
  debugShowDecision: boolean;
  useModelInIntake: boolean;
  evalId: string;
};

type InternalSession = {
  phase: "intake" | "chat";
  budgetRemaining: number;
  accumulatedUserText: string;
  lastDecision?: IntakeDecision;
};

const settings: IntakeRuntimeSettings = {
  enabled: false,
  turnBudget: 3,
  debugShowDecision: false,
  useModelInIntake: false,
  evalId: "",
};

const session: InternalSession = {
  phase: "intake",
  budgetRemaining: settings.turnBudget,
  accumulatedUserText: "",
};

export function getIntakeSettings(): IntakeRuntimeSettings {
  return { ...settings };
}

export function setIntakeSettings(partial: Partial<IntakeRuntimeSettings>) {
  if (typeof partial.enabled === "boolean") settings.enabled = partial.enabled;
  if (typeof partial.turnBudget === "number" && Number.isFinite(partial.turnBudget)) {
    settings.turnBudget = Math.max(0, Math.floor(partial.turnBudget));
  }
  if (typeof partial.debugShowDecision === "boolean") settings.debugShowDecision = partial.debugShowDecision;
  if (typeof partial.useModelInIntake === "boolean") settings.useModelInIntake = partial.useModelInIntake;
  if (typeof partial.evalId === "string") settings.evalId = partial.evalId;
}

export function resetIntakeSession() {
  session.phase = "intake";
  session.budgetRemaining = settings.turnBudget;
  session.accumulatedUserText = "";
  session.lastDecision = undefined;
}

export function getIntakeSessionSnapshot(): IntakeSessionState {
  return {
    enabled: settings.enabled,
    budgetRemaining: session.budgetRemaining,
    accumulatedUserText: session.accumulatedUserText,
    lastDecision: session.lastDecision,
  };
}

export function applyIntakeAssist(userText: string): { accumulatedUserText: string; decision: IntakeDecision } {
  session.phase = "chat";
  session.accumulatedUserText = session.accumulatedUserText
    ? `${session.accumulatedUserText}\n\n补充信息：\n${userText}`
    : userText;

  const decision = decideIntake(session.accumulatedUserText);
  session.lastDecision = decision;
  return { accumulatedUserText: session.accumulatedUserText, decision };
}

function serializeDecisionForEval(decision: IntakeDecision): string {
  return JSON.stringify({
    taskType: decision.taskType,
    handleability: decision.handleability,
    missingMep: decision.missingMep,
    selfConfidence: decision.selfConfidence,
  });
}

export function buildIntakeTraceRecord(overrideId?: string): IntakeTraceRecord | null {
  const decision = session.lastDecision;
  if (!decision) return null;

  const id = (overrideId ?? settings.evalId).trim() || `local-${new Date().toISOString()}`;
  return {
    id,
    output: serializeDecisionForEval(decision),
    rawText: session.accumulatedUserText,
    meta: {
      budgetRemaining: session.budgetRemaining,
      phase: session.phase,
    },
  };
}

type IntakeGateResult =
  | {
      kind: "passthrough";
      phase: "chat";
      effectiveUserText: string;
      decision?: IntakeDecision;
    }
  | {
      kind: "short_circuit";
      phase: "intake" | "chat";
      responseText: string;
      decision: IntakeDecision;
    };

function formatDecisionDebug(decision: IntakeDecision): string {
  return `\n\n---\n\n**[debug] intake**\n- taskType: \`${decision.taskType}\`\n- handleability: \`${decision.handleability}\`\n- missingMep: ${decision.missingMep.length ? decision.missingMep.map((m) => `\`${m}\``).join(", ") : "(none)"}\n- selfConfidence: \`${decision.selfConfidence.toFixed(2)}\``;
}

export function applyIntakeGate(userText: string): IntakeGateResult {
  if (!settings.enabled) {
    session.phase = "chat";
    return { kind: "passthrough", phase: "chat", effectiveUserText: userText };
  }

  if (session.phase === "chat") {
    return { kind: "passthrough", phase: "chat", effectiveUserText: userText };
  }

  session.accumulatedUserText = session.accumulatedUserText
    ? `${session.accumulatedUserText}\n\n补充信息：\n${userText}`
    : userText;

  const decision = decideIntake(session.accumulatedUserText);
  session.lastDecision = decision;

  if (decision.handleability === "OUT_OF_SCOPE") {
    session.phase = "chat";
    const response = `这个请求看起来不在 TIHC（TiDB support）可处理范围内。\n\n如果你是在排查 TiDB/TiKV/PD 相关问题，请补充：集群版本/组件、现象、时间窗口、以及关键日志/SQL。`;
    return {
      kind: "short_circuit",
      phase: "chat",
      responseText: settings.debugShowDecision ? `${response}${formatDecisionDebug(decision)}` : response,
      decision,
    };
  }

  if (decision.handleability === "NEED_MORE_INFO") {
    if (session.budgetRemaining <= 0) {
      session.phase = "chat";
      const response =
        "信息还不够，我先给你一个最小信息模板（复制后补齐即可）：\n\n" +
        "- 症状/现象：\n" +
        "- 时间窗口（从何时开始）：\n" +
        "- 影响范围（哪些业务/库表/region/节点）：\n" +
        "- 错误日志/报错片段（前后 20 行）：\n" +
        "- 最近变更（升级/发布/参数/扩缩容/重启）：\n";
      return {
        kind: "short_circuit",
        phase: "chat",
        responseText: settings.debugShowDecision ? `${response}${formatDecisionDebug(decision)}` : response,
        decision,
      };
    }

    session.budgetRemaining -= 1;
    const question = decision.nextQuestion ?? "能否补充一下关键上下文？";
    const response = question;
    return {
      kind: "short_circuit",
      phase: "intake",
      responseText: settings.debugShowDecision ? `${response}${formatDecisionDebug(decision)}` : response,
      decision,
    };
  }

  session.phase = "chat";
  return {
    kind: "passthrough",
    phase: "chat",
    effectiveUserText: session.accumulatedUserText,
    decision,
  };
}
