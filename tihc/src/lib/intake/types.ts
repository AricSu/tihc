export type TaskType =
  | "EXPLAIN_ERROR"
  | "TROUBLESHOOT_SYMPTOM"
  | "SQL_TUNING"
  | "CONFIG_GUIDANCE"
  | "OPERATION_CHANGE"
  | "HOW_TO_COLLECT"
  | "BEST_PRACTICE"
  | "NON_TECH_ADMIN";

export const TASK_TYPES: readonly TaskType[] = [
  "EXPLAIN_ERROR",
  "TROUBLESHOOT_SYMPTOM",
  "SQL_TUNING",
  "CONFIG_GUIDANCE",
  "OPERATION_CHANGE",
  "HOW_TO_COLLECT",
  "BEST_PRACTICE",
  "NON_TECH_ADMIN",
];

export type HandleabilityState = "HANDLE_NOW" | "NEED_MORE_INFO" | "OUT_OF_SCOPE";

export type IntakeDecision = {
  taskType: TaskType;
  handleability: HandleabilityState;
  missingMep: string[];
  nextQuestion?: string;
  selfConfidence: number;
};

export type IntakeSessionState = {
  enabled: boolean;
  budgetRemaining: number;
  accumulatedUserText: string;
  lastDecision?: IntakeDecision;
};

export type IntakeTraceRecord = {
  id: string;
  output: string;
  rawText?: string;
  meta?: Record<string, unknown>;
};
