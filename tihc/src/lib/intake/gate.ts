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
  return /\bexplain\b/.test(s) || s.includes("执行计划");
}

function looksLikeErrorSnippet(text: string): boolean {
  const s = lower(text);
  return (
    /\berror\b/.test(s) ||
    s.includes("报错") ||
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
    /(今天|昨天|前天|刚刚|最近|本周|上周|本月|上月|开始|持续)/.test(text)
  );
}

function looksLikeImpactScope(text: string): boolean {
  return /(影响|波及|所有|部分|业务|库|表|租户|region|集群)/i.test(text);
}

function looksLikeChangeClue(text: string): boolean {
  return /(变更|升级|发布|上线|回滚|迁移|扩容|缩容|参数|配置|重启)/.test(text);
}

function classifyTaskType(text: string): TaskType {
  const s = lower(text);

  if (/(怎么采集|如何采集|怎么获取|如何获取|dump|profile|pprof|诊断)/.test(s)) {
    return "HOW_TO_COLLECT";
  }
  if (/(升级|迁移|切换|变更窗口|批量变更|扩容|缩容|region|容灾|演练)/.test(s)) {
    return "OPERATION_CHANGE";
  }
  if (looksLikeSql(text)) {
    return "SQL_TUNING";
  }
  if (/(tidb_|tikv_|pd_|tiflash_|配置|参数|开关)/.test(s)) {
    return "CONFIG_GUIDANCE";
  }
  if (/(最佳实践|best practice|规范|建议)/.test(s)) {
    return "BEST_PRACTICE";
  }
  if (/(账号|权限|工单|流程|开通|创建用户|role)/.test(s)) {
    return "NON_TECH_ADMIN";
  }

  const hasError = looksLikeErrorSnippet(text);
  const hasSymptom = /(慢|超时|异常|失败|卡住|抖动|延迟|内存|cpu|io|oom|重启|不可用)/i.test(text);
  if (hasError && !hasSymptom) {
    return "EXPLAIN_ERROR";
  }
  return "TROUBLESHOOT_SYMPTOM";
}

function decideOutOfScope(text: string): boolean {
  const s = lower(text);
  const hasTiDbHint = containsAny(s, TIDB_KEYWORDS) || /(mysql|sql|数据库|db)/.test(s);
  const clearlyOther = /(爬虫|crawl|selenium|scrapy|前端|react|javascript)/.test(s);
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
    "错误原文/日志片段": "请把完整报错/日志片段粘贴出来（最好包含前后 20 行）。",
    "时间窗口": "这个问题大概从什么时候开始出现？（具体时间/时间段）",
    "影响范围": "影响范围是哪些？（哪些业务/库表/region/节点，是否全量）",
    "变更线索": "最近是否有变更？（升级/发布/参数调整/扩缩容/重启）",
    "SQL": "请贴一下完整 SQL（建议包含绑定变量的实际值或示例）。",
    "EXPLAIN/执行计划": "请提供 EXPLAIN/执行计划（或执行计划截图/文本）。",
    "期望 vs 实际": "你期望的表现是什么？实际表现是什么？（例如耗时/错误/影响）",
    "表结构/索引信息": "请提供相关表结构与索引信息（SHOW CREATE TABLE/索引列表）。",
    "具体参数/配置项": "你具体想咨询/调整哪个参数或配置项？（参数名 + 当前值）",
    "目标行为/使用场景": "你的使用场景/目标是什么？（例如写入为主、跨 region、延迟目标等）",
    "当前状态": "当前集群状态是什么？（版本/拓扑/组件数量/关键配置）",
    "目标状态": "目标状态是什么？（目标版本/目标拓扑/目标 region/目标窗口）",
    "操作方式/工具": "你计划用什么方式操作？（TiUP/K8s/手工/CDC/BR/DM 等）",
    "应用场景/需求上下文（RPO/RTO/数据量/部署形态）":
      "你的场景是怎样的？（部署形态、数据量、RPO/RTO、是否跨 region）",
    "账号/权限范围": "需要开通什么权限范围？（哪些库/表/操作，只读/读写/管理）",
    "目标用户信息": "目标用户是谁？（账号/邮箱/组织）",
    "组织/项目上下文": "属于哪个组织/项目/环境？（prod/staging/集群名）",
  };

  if (bySlot[first]) return bySlot[first];

  const prefix = taskType === "HOW_TO_COLLECT" ? "为了给出采集方法，我先确认：" : "为了更快定位，我先确认：";
  return `${prefix}${first}？`;
}

function computeMissingMep(taskType: TaskType, text: string): string[] {
  const missing: string[] = [];

  if (taskType === "EXPLAIN_ERROR") {
    if (!looksLikeErrorSnippet(text)) missing.push("错误原文/日志片段");
    return missing;
  }

  if (taskType === "TROUBLESHOOT_SYMPTOM") {
    if (!looksLikeTimeWindow(text)) missing.push("时间窗口");
    if (!looksLikeImpactScope(text)) missing.push("影响范围");
    if (!looksLikeErrorSnippet(text)) missing.push("错误原文/日志片段");
    if (!looksLikeChangeClue(text)) missing.push("变更线索");
    return missing;
  }

  if (taskType === "SQL_TUNING") {
    if (!looksLikeSql(text)) missing.push("SQL");
    if (!looksLikeExplain(text)) missing.push("EXPLAIN/执行计划");
    if (!/(期望|实际|expected|actual|ms|s|秒|分钟)/i.test(text)) missing.push("期望 vs 实际");
    if (!/(show create table|表结构|索引|index)/i.test(text)) missing.push("表结构/索引信息");
    return missing;
  }

  if (taskType === "CONFIG_GUIDANCE") {
    if (!/(tidb_|tikv_|pd_|tiflash_)/i.test(text)) missing.push("具体参数/配置项");
    if (!/(场景|目标|希望|想要|workload|oltp|olap|延迟|吞吐)/i.test(text)) {
      missing.push("目标行为/使用场景");
    }
    return missing;
  }

  if (taskType === "OPERATION_CHANGE") {
    if (!/(当前|现状|版本|v\d|拓扑|tidb|tikv|pd)/i.test(text)) missing.push("当前状态");
    if (!/(目标|to v|升级到|迁移到|切换到|v\d)/i.test(text)) missing.push("目标状态");
    if (!/(tiup|k8s|br|dm|cdc|工具|方式)/i.test(text)) missing.push("操作方式/工具");
    if (!/(窗口|停机|只读|maintenance|回滚|预案)/i.test(text)) missing.push("环境与窗口/回滚预案");
    return missing;
  }

  if (taskType === "HOW_TO_COLLECT") {
    return missing;
  }

  if (taskType === "BEST_PRACTICE") {
    const hasContext = /(rpo|rto|k8s|kubernetes|数据量|部署|region|窗口|业务)/i.test(text);
    if (!hasContext) missing.push("应用场景/需求上下文（RPO/RTO/数据量/部署形态）");
    return missing;
  }

  if (taskType === "NON_TECH_ADMIN") {
    if (!/(只读|读写|admin|权限|role)/i.test(text)) missing.push("账号/权限范围");
    if (!/(账号|邮箱|user|同事)/i.test(text)) missing.push("目标用户信息");
    missing.push("组织/项目上下文");
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

