import type {
  CaseHistoryMessage,
  CaseHistoryRepository,
  CreateStoredCaseInput,
  StoredCaseRecord,
  StoredCaseMessage,
} from "./case-store";

export const EMPTY_CASE_SUMMARY = "No thread activity yet.";
const EMPTY_MESSAGES_PREVIEW: StoredCaseMessage[] = [
  {
    role: "tihc",
    text: EMPTY_CASE_SUMMARY,
  },
];

function extractTextFromContent(content: unknown): string {
  if (typeof content === "string") return content.trim();
  if (!Array.isArray(content)) return "";

  return content
    .map((part) => {
      if (!part || typeof part !== "object") return "";
      const text = (part as { text?: unknown }).text;
      return typeof text === "string" ? text.trim() : "";
    })
    .filter(Boolean)
    .join("\n")
    .trim();
}

function toStoredCaseMessage(item: CaseHistoryMessage): StoredCaseMessage | null {
  const text = extractTextFromContent(item.message.content);
  if (!text) return null;

  return {
    role: item.message.role === "assistant" ? "tihc" : "operator",
    text,
  };
}

export function deriveCasePresentation(repository: CaseHistoryRepository): {
  summary: string;
  signals: string[];
  messagesPreview: StoredCaseMessage[];
} {
  const textMessages = repository.messages
    .map(toStoredCaseMessage)
    .filter((item): item is StoredCaseMessage => Boolean(item));
  const summary =
    [...textMessages].reverse().find((item) => item.role === "tihc")?.text ??
    textMessages[textMessages.length - 1]?.text ??
    EMPTY_CASE_SUMMARY;
  const signals = [...textMessages]
    .reverse()
    .map((item) => item.text)
    .filter((value, index, values) => values.indexOf(value) === index)
    .slice(0, 3);
  const messagesPreview = textMessages.slice(-2);

  return {
    summary,
    signals: signals.length ? signals : [EMPTY_CASE_SUMMARY],
    messagesPreview: messagesPreview.length ? messagesPreview : EMPTY_MESSAGES_PREVIEW,
  };
}

export function buildStoredCaseRecord(
  input: CreateStoredCaseInput,
  repository?: CaseHistoryRepository | null,
): StoredCaseRecord {
  return {
    ...input,
    ...(repository
      ? deriveCasePresentation(repository)
      : {
          summary: EMPTY_CASE_SUMMARY,
          signals: [EMPTY_CASE_SUMMARY],
          messagesPreview: EMPTY_MESSAGES_PREVIEW,
        }),
  };
}
