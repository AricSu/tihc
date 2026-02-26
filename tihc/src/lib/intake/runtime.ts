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
    ? `${session.accumulatedUserText}\n\nAdditional details:\n${userText}`
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
    ? `${session.accumulatedUserText}\n\nAdditional details:\n${userText}`
    : userText;

  const decision = decideIntake(session.accumulatedUserText);
  session.lastDecision = decision;

  if (decision.handleability === "OUT_OF_SCOPE") {
    session.phase = "chat";
    const response = `This request appears to be outside TIHC (TiDB support) scope.\n\nIf you are troubleshooting TiDB/TiKV/PD, please include: cluster version/components, symptoms, time window, and key logs/SQL.`;
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
        "Information is still insufficient. Please fill this minimum template:\n\n" +
        "- Symptom:\n" +
        "- Time window (when it started):\n" +
        "- Impact scope (businesses/tables/regions/nodes):\n" +
        "- Error log snippet (around 20 lines before/after):\n" +
        "- Recent changes (upgrade/release/parameter/scale/restart):\n";
      return {
        kind: "short_circuit",
        phase: "chat",
        responseText: settings.debugShowDecision ? `${response}${formatDecisionDebug(decision)}` : response,
        decision,
      };
    }

    session.budgetRemaining -= 1;
    const question = decision.nextQuestion ?? "Could you provide key missing context?";
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
