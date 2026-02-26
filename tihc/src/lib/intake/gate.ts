import type { HandleabilityState, IntakeDecision, TaskType } from "./types";

const TIDB_KEYWORDS = [
  "tidb",
  "tikv",
  "pd",
  "tiflash",
  "cdc",
  "br",
  "dm",
  "pingcap",
  "tiup",
];

function normalize(text: string): string {
  return text.trim();
}

function lower(text: string): string {
  return text.toLowerCase();
}

function containsAny(haystack: string, needles: string[]): boolean {
  const s = lower(haystack);
  return needles.some((needle) => s.includes(lower(needle)));
}

function looksLikeSql(text: string): boolean {
  const s = lower(text);
  return /\b(select|insert|update|delete|replace)\b/.test(s);
}

function looksLikeExplain(text: string): boolean {
  const s = lower(text);
  return /\b(explain|query\s*plan|execution\s*plan)\b/.test(s);
}

function looksLikeErrorSnippet(text: string): boolean {
  const s = lower(text);
  return (
    /\berror\b/.test(s) ||
    s.includes("panic") ||
    s.includes("stack") ||
    s.includes("errno") ||
    s.includes("exception") ||
    /ERROR\s+\d+/i.test(text)
  );
}

function looksLikeTimeWindow(text: string): boolean {
  return (
    /(\d{1,2}:\d{2})/.test(text) ||
    /(\d{4}-\d{2}-\d{2})/.test(text) ||
    /(today|yesterday|recently|this week|last week|this month|last month|start|since|duration)/i.test(text)
  );
}

function looksLikeImpactScope(text: string): boolean {
  return /(impact|affected|all|partial|business|database|table|tenant|region|cluster)/i.test(text);
}

function looksLikeChangeClue(text: string): boolean {
  return /(change|upgrade|release|deploy|rollback|migration|scale|parameter|config|restart)/i.test(text);
}

function classifyTaskType(text: string): TaskType {
  const s = lower(text);

  if (/(how to collect|how to get|collect|dump|profile|pprof|diagnostic)/.test(s)) {
    return "HOW_TO_COLLECT";
  }
  if (/(upgrade|migration|switch|change window|batch change|scale|region|drill|disaster recovery)/.test(s)) {
    return "OPERATION_CHANGE";
  }
  if (looksLikeSql(text)) {
    return "SQL_TUNING";
  }
  if (/(tidb_|tikv_|pd_|tiflash_|config|parameter|setting)/.test(s)) {
    return "CONFIG_GUIDANCE";
  }
  if (/(best practice|guideline|recommendation)/.test(s)) {
    return "BEST_PRACTICE";
  }
  if (/(account|permission|ticket|process|create user|role)/.test(s)) {
    return "NON_TECH_ADMIN";
  }

  const hasError = looksLikeErrorSnippet(text);
  const hasSymptom = /(slow|timeout|abnormal|failed|stuck|jitter|latency|memory|cpu|io|oom|restart|unavailable)/i.test(text);
  if (hasError && !hasSymptom) {
    return "EXPLAIN_ERROR";
  }
  return "TROUBLESHOOT_SYMPTOM";
}

function decideOutOfScope(text: string): boolean {
  const s = lower(text);
  const hasTiDbHint = containsAny(s, TIDB_KEYWORDS) || /(mysql|sql|database|db)/.test(s);
  const clearlyOther = /(crawler|crawl|selenium|scrapy|frontend|react|javascript)/.test(s);
  return !hasTiDbHint && clearlyOther;
}

function clamp01(value: number): number {
  return Math.max(0, Math.min(1, value));
}

function pickFirst(items: string[]): string | undefined {
  return items.length ? items[0] : undefined;
}

function buildQuestion(taskType: TaskType, missing: string[]): string | undefined {
  const first = pickFirst(missing);
  if (!first) return undefined;

  const bySlot: Record<string, string> = {
    "Error snippet/log excerpt": "Please paste the complete error/log snippet (ideally with about 20 lines of context).",
    "Time window": "When did this issue start? Please provide exact time or time range.",
    "Impact scope": "What is the impact scope? (which services/databases/tables/regions/nodes, full or partial)",
    "Change clue": "Any recent changes? (upgrade/release/config change/scale/restart)",
    SQL: "Please share the full SQL (with sample bind values when possible).",
    "EXPLAIN/query plan": "Please provide EXPLAIN/query plan output (text or screenshot).",
    "Expected vs actual": "What was expected vs what actually happened? (latency/errors/impact)",
    "Schema/index info": "Please provide related schema and index information (SHOW CREATE TABLE/index list).",
    "Specific parameter/config": "Which parameter/config are you asking about? (name + current value)",
    "Target behavior/use case": "What is your target behavior/use case? (e.g., write-heavy, cross-region, latency target)",
    "Current state": "What is the current cluster state? (version/topology/components/key config)",
    "Target state": "What is the target state? (target version/topology/region/window)",
    "Execution method/tools": "Which method/tool will you use? (TiUP/K8s/manual/CDC/BR/DM)",
    "Context (RPO/RTO/data size/deployment)": "What is your context? (deployment model, data size, RPO/RTO, cross-region or not)",
    "Account/permission scope": "What permission scope is needed? (which DB/table/actions, read-only/read-write/admin)",
    "Target user info": "Who is the target user? (account/email/team)",
    "Org/project context": "Which org/project/environment is this for? (prod/staging/cluster name)",
    "Environment window/rollback plan": "What is the maintenance window and rollback plan?",
  };

  if (bySlot[first]) return bySlot[first];

  const prefix = taskType === "HOW_TO_COLLECT" ? "To provide collection steps, I need:" : "To troubleshoot faster, I need:";
  return `${prefix} ${first}?`;
}

function computeMissingMep(taskType: TaskType, text: string): string[] {
  const missing: string[] = [];

  if (taskType === "EXPLAIN_ERROR") {
    if (!looksLikeErrorSnippet(text)) missing.push("Error snippet/log excerpt");
    return missing;
  }

  if (taskType === "TROUBLESHOOT_SYMPTOM") {
    if (!looksLikeTimeWindow(text)) missing.push("Time window");
    if (!looksLikeImpactScope(text)) missing.push("Impact scope");
    if (!looksLikeErrorSnippet(text)) missing.push("Error snippet/log excerpt");
    if (!looksLikeChangeClue(text)) missing.push("Change clue");
    return missing;
  }

  if (taskType === "SQL_TUNING") {
    if (!looksLikeSql(text)) missing.push("SQL");
    if (!looksLikeExplain(text)) missing.push("EXPLAIN/query plan");
    if (!/(expected|actual|ms|sec|second|minute|latency|cost)/i.test(text)) missing.push("Expected vs actual");
    if (!/(show create table|schema|index|keys?)/i.test(text)) missing.push("Schema/index info");
    return missing;
  }

  if (taskType === "CONFIG_GUIDANCE") {
    if (!/(tidb_|tikv_|pd_|tiflash_)/i.test(text)) missing.push("Specific parameter/config");
    if (!/(use case|target|goal|want|workload|oltp|olap|latency|throughput)/i.test(text)) {
      missing.push("Target behavior/use case");
    }
    return missing;
  }

  if (taskType === "OPERATION_CHANGE") {
    if (!/(current|as-is|version|v\d|topology|tidb|tikv|pd)/i.test(text)) missing.push("Current state");
    if (!/(target|to v|upgrade to|migrate to|switch to|v\d)/i.test(text)) missing.push("Target state");
    if (!/(tiup|k8s|br|dm|cdc|tool|method|procedure)/i.test(text)) missing.push("Execution method/tools");
    if (!/(window|downtime|readonly|maintenance|rollback|fallback)/i.test(text)) {
      missing.push("Environment window/rollback plan");
    }
    return missing;
  }

  if (taskType === "HOW_TO_COLLECT") {
    return missing;
  }

  if (taskType === "BEST_PRACTICE") {
    const hasContext = /(rpo|rto|k8s|kubernetes|data size|deployment|region|window|business)/i.test(text);
    if (!hasContext) missing.push("Context (RPO/RTO/data size/deployment)");
    return missing;
  }

  if (taskType === "NON_TECH_ADMIN") {
    if (!/(readonly|read[- ]?write|admin|permission|role)/i.test(text)) missing.push("Account/permission scope");
    if (!/(account|email|user|owner|team member)/i.test(text)) missing.push("Target user info");
    missing.push("Org/project context");
    return missing;
  }

  return missing;
}

function computeConfidence(taskType: TaskType, handleability: HandleabilityState, missing: string[], text: string): number {
  let confidence = 0.55;
  if (containsAny(text, TIDB_KEYWORDS)) confidence += 0.1;
  if (taskType === "SQL_TUNING" && looksLikeSql(text)) confidence += 0.15;
  if (taskType === "EXPLAIN_ERROR" && looksLikeErrorSnippet(text)) confidence += 0.15;
  if (taskType === "OPERATION_CHANGE" && looksLikeChangeClue(text)) confidence += 0.1;
  if (handleability === "NEED_MORE_INFO") confidence -= Math.min(0.2, missing.length * 0.05);
  if (handleability === "OUT_OF_SCOPE") confidence = 0.75;
  return clamp01(confidence);
}

export function decideIntake(rawText: string): IntakeDecision {
  const text = normalize(rawText);

  if (decideOutOfScope(text)) {
    const taskType: TaskType = "BEST_PRACTICE";
    const handleability: HandleabilityState = "OUT_OF_SCOPE";
    const missingMep: string[] = [];
    return {
      taskType,
      handleability,
      missingMep,
      selfConfidence: computeConfidence(taskType, handleability, missingMep, text),
    };
  }

  const taskType = classifyTaskType(text);
  const missingMep = computeMissingMep(taskType, text);
  const handleability: HandleabilityState = missingMep.length ? "NEED_MORE_INFO" : "HANDLE_NOW";
  const nextQuestion = handleability === "NEED_MORE_INFO" ? buildQuestion(taskType, missingMep) : undefined;

  return {
    taskType,
    handleability,
    missingMep,
    nextQuestion,
    selfConfidence: computeConfidence(taskType, handleability, missingMep, text),
  };
}

export function renderHiddenIntakeComment(decision: IntakeDecision): string {
  const payload = {
    taskType: decision.taskType,
    handleability: decision.handleability,
    missingMep: decision.missingMep,
    selfConfidence: decision.selfConfidence,
  };
  return `<!--TIHC_INTAKE:${JSON.stringify(payload)}-->`;
}
