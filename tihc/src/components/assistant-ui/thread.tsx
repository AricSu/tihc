import type { AssistantReplyFontSize } from "@/lib/chat/agent-types";
import { MarkdownText } from "@/components/assistant-ui/markdown-text";
import { TooltipIconButton } from "@/components/assistant-ui/tooltip-icon-button";
import {
  isExternalHttpUrl,
  openExternalUrl,
  scrollToHashTarget,
  toTrackedExternalUrl,
} from "@/lib/app/external-link";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import {
  AssistantIf,
  ComposerPrimitive,
  ErrorPrimitive,
  MessagePrimitive,
  ThreadPrimitive,
  useMessage,
} from "@assistant-ui/react";
import {
  ArrowDownIcon,
  ArrowUpIcon,
  CheckCircle2Icon,
  ChevronDownIcon,
  ChevronUpIcon,
  CheckIcon,
  CopyIcon,
  ListChecksIcon,
  Loader2Icon,
  SquareIcon,
} from "lucide-react";
import type { CSSProperties, ComponentPropsWithoutRef, FC, MouseEvent, ReactNode } from "react";
import { useEffect, useMemo, useState } from "react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";

export type ThreadProps = {
  assistantReplyFontSize?: AssistantReplyFontSize;
  title?: string;
  executionTargetName?: string;
  composerToolbar?: ReactNode;
};

export function resolveAssistantReplyFontSizeVars(
  value: AssistantReplyFontSize | undefined,
): CSSProperties {
  const fontSize = value === "small" || value === "large" ? value : "default";

  if (fontSize === "small") {
    return {
      "--tihc-assistant-body-font-size": "14px",
      "--tihc-assistant-body-line-height": "1.85",
      "--tihc-assistant-h1-font-size": "2rem",
      "--tihc-assistant-h2-font-size": "1.6rem",
      "--tihc-assistant-h3-font-size": "1.05rem",
      "--tihc-assistant-code-font-size": "12px",
      "--tihc-assistant-step-duration-font-size": "11px",
    } as CSSProperties;
  }

  if (fontSize === "large") {
    return {
      "--tihc-assistant-body-font-size": "16px",
      "--tihc-assistant-body-line-height": "1.95",
      "--tihc-assistant-h1-font-size": "2.3rem",
      "--tihc-assistant-h2-font-size": "1.8rem",
      "--tihc-assistant-h3-font-size": "1.22rem",
      "--tihc-assistant-code-font-size": "14px",
      "--tihc-assistant-step-duration-font-size": "13px",
    } as CSSProperties;
  }

  return {
    "--tihc-assistant-body-font-size": "15px",
    "--tihc-assistant-body-line-height": "1.9",
    "--tihc-assistant-h1-font-size": "2.15rem",
    "--tihc-assistant-h2-font-size": "1.7rem",
    "--tihc-assistant-h3-font-size": "1.15rem",
    "--tihc-assistant-code-font-size": "13px",
    "--tihc-assistant-step-duration-font-size": "12px",
  } as CSSProperties;
}

export const Thread: FC<ThreadProps> = ({ assistantReplyFontSize, composerToolbar }) => {
  return (
    <ThreadPrimitive.Root className="flex h-full min-h-0 flex-col overflow-hidden bg-transparent">
      <ThreadPrimitive.Viewport
        turnAnchor="bottom"
        autoScroll
        scrollToBottomOnInitialize
        scrollToBottomOnRunStart
        className="tihc-scrollbar relative flex flex-1 flex-col overflow-x-hidden overflow-y-auto px-7 pt-8"
      >
        <AssistantIf condition={({ thread }) => thread.isEmpty}>
          <CaseReadyState />
        </AssistantIf>

        <ThreadPrimitive.Messages
          components={{
            UserMessage,
            AssistantMessage: () => (
              <AssistantMessage assistantReplyFontSize={assistantReplyFontSize} />
            ),
          }}
        />

        <ThreadPrimitive.ViewportFooter className="sticky bottom-0 z-40 mt-auto flex flex-col gap-2 bg-[linear-gradient(180deg,rgba(255,255,255,0),rgba(255,255,255,0.48)_28%,rgba(255,255,255,0.82)_100%)] pb-3 pt-10">
          <ThreadScrollToBottom />
          <Composer composerToolbar={composerToolbar} />
        </ThreadPrimitive.ViewportFooter>
      </ThreadPrimitive.Viewport>
    </ThreadPrimitive.Root>
  );
};

export const CaseReadyState: FC<{ title?: string; executionTargetName?: string }> = () => {
  return (
    <div className="mx-auto my-auto flex w-full max-w-2xl flex-col items-center px-6 py-20 text-center">
      <h2 className="tihc-display text-[2.8rem] font-semibold tracking-[-0.065em] text-slate-950">
        TiHC is ready.
      </h2>
      <p className="mt-4 max-w-xl text-[15px] leading-7 text-slate-500">
        Describe the issue, paste an error, or tell TiHC what to investigate.
      </p>
    </div>
  );
};

const ThreadScrollToBottom: FC = () => {
  return (
    <ThreadPrimitive.ScrollToBottom asChild>
      <TooltipIconButton
        tooltip="Scroll to bottom"
        variant="outline"
        className="absolute -top-11 z-10 self-center rounded-full bg-white/96 p-2 text-slate-400 disabled:invisible"
      >
        <ArrowDownIcon />
      </TooltipIconButton>
    </ThreadPrimitive.ScrollToBottom>
  );
};

const Composer: FC<{ composerToolbar?: ReactNode }> = ({ composerToolbar }) => {
  return (
    <ComposerPrimitive.Root className="relative mx-auto flex w-full max-w-3xl flex-col">
      <div className="rounded-[18px] border border-slate-200/80 bg-white px-3 py-1.5">
        <ComposerPrimitive.Input
          placeholder="Describe the case or paste the first signal..."
          className="aui-composer-input min-h-14 max-h-52 w-full resize-none overflow-y-auto bg-transparent px-3 py-2 pr-4 text-[15px] leading-7 text-slate-800 outline-none placeholder:text-slate-400 [scrollbar-width:thin] [scrollbar-color:rgba(100,116,139,0.58)_transparent] [&::-webkit-scrollbar]:w-1.5 [&::-webkit-scrollbar-track]:bg-transparent [&::-webkit-scrollbar-thumb]:rounded-full [&::-webkit-scrollbar-thumb]:bg-slate-400/60 [&::-webkit-scrollbar-thumb:hover]:bg-slate-500/72"
          rows={1}
          autoFocus
          aria-label="Message input"
        />
        <ComposerAction composerToolbar={composerToolbar} />
      </div>
    </ComposerPrimitive.Root>
  );
};

export const ComposerMeta: FC<{ toolbar?: ReactNode }> = ({ toolbar }) => {
  if (toolbar) {
    return <div className="min-w-0 flex items-center gap-2 overflow-hidden">{toolbar}</div>;
  }

  return (
    <div className="px-2 py-1 text-[10px] font-medium tracking-[0.06em] text-slate-400">
      Shift + Enter for newline
    </div>
  );
};

const ComposerAction: FC<{ composerToolbar?: ReactNode }> = ({ composerToolbar }) => {
  return (
    <div className="mt-2 flex items-center justify-between gap-3 px-1">
      <ComposerMeta toolbar={composerToolbar} />

      <AssistantIf condition={({ thread }) => !thread.isRunning}>
        <ComposerPrimitive.Send asChild>
          <TooltipIconButton
            tooltip="Send"
            side="top"
            type="submit"
            variant="default"
            size="icon"
            className="size-8 rounded-full bg-slate-950 text-white"
            aria-label="Send message"
          >
            <ArrowUpIcon className="size-3.5" />
          </TooltipIconButton>
        </ComposerPrimitive.Send>
      </AssistantIf>

      <AssistantIf condition={({ thread }) => thread.isRunning}>
        <ComposerPrimitive.Cancel asChild>
          <Button
            type="button"
            variant="default"
            size="icon"
            className="size-8 rounded-full border border-slate-200 bg-white text-slate-900"
            aria-label="Stop generating"
          >
            <SquareIcon className="size-3 fill-current" />
          </Button>
        </ComposerPrimitive.Cancel>
      </AssistantIf>
    </div>
  );
};

const MessageError: FC = () => {
  return (
    <MessagePrimitive.Error>
      <ErrorPrimitive.Root className="mt-2 rounded-md border border-destructive/30 bg-destructive/10 p-2 text-destructive text-xs">
        <ErrorPrimitive.Message className="line-clamp-3" />
      </ErrorPrimitive.Root>
    </MessagePrimitive.Error>
  );
};

type ParsedAssistantContent = {
  isStructured: boolean;
  steps: string[];
  answer: string;
};

function parseAssistantContent(raw: string): ParsedAssistantContent {
  const text = raw.replace(/\r/g, "");
  const progressMarker = "Retrieval Process:";
  const answerMarker = "Answer:";
  const progressIdx = text.indexOf(progressMarker);
  const answerIdx = text.indexOf(answerMarker);

  if (progressIdx < 0 && answerIdx < 0) {
    return {
      isStructured: false,
      steps: [],
      answer: text.trim(),
    };
  }

  let progressBlock = "";
  if (progressIdx >= 0) {
    const start = progressIdx + progressMarker.length;
    const end = answerIdx >= 0 ? answerIdx : text.length;
    progressBlock = text.slice(start, end);
  }

  const steps = progressBlock
    .split("\n")
    .map((line) => line.trim())
    .filter(Boolean)
    .map((line) => line.replace(/^[-*]\s*/, ""));

  let answer = "";
  if (answerIdx >= 0) {
    answer = text.slice(answerIdx + answerMarker.length).trim();
  } else if (progressIdx < 0) {
    answer = text.trim();
  }

  return {
    isStructured: true,
    steps,
    answer,
  };
}

function splitStepDuration(step: string): { title: string; duration: string | null } {
  const match = step.match(/^(.*?)(\d+(?:\.\d+)?)s$/);
  if (!match) {
    return { title: step, duration: null };
  }
  return {
    title: match[1].trim(),
    duration: `${match[2]}s`,
  };
}

const MarkdownAnchor: FC<ComponentPropsWithoutRef<"a">> = ({
  href,
  className,
  onClick,
  ...props
}) => {
  const resolvedHref = href
    ? isExternalHttpUrl(href)
      ? toTrackedExternalUrl(href)
      : href
    : href;
  const isExternal = !!resolvedHref && isExternalHttpUrl(resolvedHref);

  const handleClick = (event: MouseEvent<HTMLAnchorElement>) => {
    onClick?.(event);
    if (event.defaultPrevented) return;
    if (!resolvedHref) return;

    if (resolvedHref.startsWith("#")) {
      event.preventDefault();
      if (!scrollToHashTarget(resolvedHref)) {
        window.location.hash = resolvedHref.slice(1);
      }
      return;
    }

    if (!isExternalHttpUrl(resolvedHref)) return;
    event.preventDefault();
    openExternalUrl(resolvedHref);
  };

  return (
    <a
      href={resolvedHref}
      target={isExternal ? "_blank" : undefined}
      rel={isExternal ? "noopener noreferrer" : undefined}
      onClick={handleClick}
      className={className}
      {...props}
    />
  );
};

const StepMarkdown: FC<{ text: string }> = ({ text }) => {
  return (
    <ReactMarkdown
      remarkPlugins={[remarkGfm]}
      components={{
        p: ({ children }) => <>{children}</>,
        a: ({ className, ...props }) => (
          <MarkdownAnchor
            className={cn(
              "font-medium text-slate-900 underline decoration-slate-300 underline-offset-4 transition hover:decoration-slate-900",
              className,
            )}
            {...props}
          />
        ),
        code: ({ className, ...props }) => (
          <code className={cn("rounded-md bg-slate-100 px-1.5 py-0.5 font-medium text-slate-900", className)} {...props} />
        ),
      }}
    >
      {text}
    </ReactMarkdown>
  );
};

const AssistantStructuredBody: FC<{ assistantReplyFontSize?: AssistantReplyFontSize }> = ({
  assistantReplyFontSize,
}) => {
  const text = useMessage((message) =>
    message.content
      .filter((part) => part.type === "text")
      .map((part) => (part as { type: "text"; text: string }).text)
      .join(""),
  );
  const isRunning = useMessage((message) => message.status?.type === "running");
  const [collapsed, setCollapsed] = useState(false);

  const parsed = useMemo(() => parseAssistantContent(text), [text]);
  if (!parsed.isStructured) {
    return <MessagePrimitive.Parts components={{ Text: MarkdownText }} />;
  }

  const hasSteps = parsed.steps.length > 0;
  const offset = collapsed ? parsed.steps.length - 1 : 0;
  const visibleSteps = collapsed ? parsed.steps.slice(-1) : parsed.steps;

  return (
    <div className="space-y-5">
      {hasSteps ? (
        <section className="border-l border-slate-200/80 pl-4">
          <div className="mb-2 flex items-center justify-between">
            <div className="text-[10px] font-medium tracking-[0.08em] text-slate-400">
              Retrieval
            </div>
            {parsed.steps.length > 1 ? (
              <button
                type="button"
                onClick={() => setCollapsed((prev) => !prev)}
                className="inline-flex items-center gap-1 text-[11px] text-slate-400 hover:text-slate-700"
              >
                {collapsed ? (
                  <>
                    <ChevronDownIcon className="size-3.5" />
                    Expand
                  </>
                ) : (
                  <>
                    <ChevronUpIcon className="size-3.5" />
                    Collapse
                  </>
                )}
              </button>
            ) : null}
          </div>

          <ul className="relative space-y-2 pl-7">
            <span className="absolute bottom-1 left-2 top-1 w-px bg-slate-200" />
            {visibleSteps.map((step, idx) => {
              const realIdx = idx + offset;
              const isCurrent = isRunning && realIdx === parsed.steps.length - 1;
              const { title, duration } = splitStepDuration(step);
              return (
                <li key={`${realIdx}-${step}`} className="relative">
                  <span className="absolute -left-[1.52rem] top-0.5 rounded-full bg-white">
                    {isCurrent ? (
                      <Loader2Icon className="size-4 text-slate-400 animate-spin" />
                    ) : (
                      <CheckCircle2Icon className="size-4 text-emerald-500" />
                    )}
                  </span>
                  <div
                    className="text-slate-700"
                    style={{
                      fontSize: "var(--tihc-assistant-body-font-size)",
                      lineHeight: "var(--tihc-assistant-body-line-height)",
                    }}
                  >
                    <StepMarkdown text={title} />
                    {duration ? (
                      <span
                        className="ml-2 tracking-[0.04em] text-slate-400"
                        style={{ fontSize: "var(--tihc-assistant-step-duration-font-size)" }}
                      >
                        {duration}
                      </span>
                    ) : null}
                  </div>
                </li>
              );
            })}
          </ul>
        </section>
      ) : null}

      {parsed.answer || !isRunning ? (
        <section>
          <div
            className="px-1 text-slate-800"
            style={{
              fontSize: "var(--tihc-assistant-body-font-size)",
              lineHeight: "var(--tihc-assistant-body-line-height)",
            }}
          >
            <ReactMarkdown
              remarkPlugins={[remarkGfm]}
              components={{
                h1: ({ ...props }) => (
                  <h1
                    className="tihc-display mt-8 mb-3 font-semibold tracking-[-0.045em] text-slate-950 first:mt-0"
                    style={{
                      fontSize: "var(--tihc-assistant-h1-font-size)",
                      lineHeight: "1.08",
                    }}
                    {...props}
                  />
                ),
                h2: ({ ...props }) => (
                  <h2
                    className="tihc-display mt-8 mb-3 font-semibold tracking-[-0.04em] text-slate-950 first:mt-0"
                    style={{
                      fontSize: "var(--tihc-assistant-h2-font-size)",
                      lineHeight: "1.12",
                    }}
                    {...props}
                  />
                ),
                h3: ({ ...props }) => (
                  <h3
                    className="mt-6 mb-2 font-semibold text-slate-900 first:mt-0"
                    style={{
                      fontSize: "var(--tihc-assistant-h3-font-size)",
                      lineHeight: "1.75",
                    }}
                    {...props}
                  />
                ),
                p: ({ ...props }) => (
                  <p
                    className="mt-5 text-slate-700 first:mt-0"
                    style={{
                      fontSize: "var(--tihc-assistant-body-font-size)",
                      lineHeight: "var(--tihc-assistant-body-line-height)",
                    }}
                    {...props}
                  />
                ),
                ul: ({ ...props }) => (
                  <ul
                    className="mt-4 ml-5 list-disc space-y-2 text-slate-700"
                    style={{
                      fontSize: "var(--tihc-assistant-body-font-size)",
                      lineHeight: "var(--tihc-assistant-body-line-height)",
                    }}
                    {...props}
                  />
                ),
                ol: ({ ...props }) => (
                  <ol
                    className="mt-4 ml-5 list-decimal space-y-2 text-slate-700"
                    style={{
                      fontSize: "var(--tihc-assistant-body-font-size)",
                      lineHeight: "var(--tihc-assistant-body-line-height)",
                    }}
                    {...props}
                  />
                ),
                a: ({ className, ...props }) => (
                  <MarkdownAnchor
                    className={cn(
                      "font-medium text-slate-900 underline decoration-slate-300 underline-offset-4 transition hover:decoration-slate-900",
                      className,
                    )}
                    {...props}
                  />
                ),
                code: ({ className, ...props }) => {
                  const isBlock = className?.includes("language-");
                  return (
                    <code
                      className={cn(
                        isBlock
                          ? "font-mono text-slate-900"
                          : "rounded-md bg-slate-100 px-1.5 py-0.5 font-medium text-slate-900",
                        className,
                      )}
                      {...props}
                    />
                  );
                },
                pre: ({ ...props }) => (
                  <pre
                    className="mt-5 overflow-x-auto rounded-2xl border border-slate-200 bg-slate-50 p-4 text-slate-900"
                    style={{
                      fontSize: "var(--tihc-assistant-code-font-size)",
                      lineHeight: "1.75",
                    }}
                    {...props}
                  />
                ),
                blockquote: ({ ...props }) => (
                  <blockquote
                    className="mt-5 border-l border-slate-300 pl-5 text-slate-500 italic"
                    style={{
                      fontSize: "var(--tihc-assistant-body-font-size)",
                      lineHeight: "var(--tihc-assistant-body-line-height)",
                    }}
                    {...props}
                  />
                ),
                hr: ({ ...props }) => <hr className="my-8 border-slate-200" {...props} />,
              }}
            >
              {parsed.answer}
            </ReactMarkdown>
          </div>
        </section>
      ) : null}
    </div>
  );
};

const AssistantMessage: FC<{ assistantReplyFontSize?: AssistantReplyFontSize }> = ({
  assistantReplyFontSize,
}) => {
  return (
    <MessagePrimitive.Root
      className="group mx-auto w-full max-w-3xl py-2"
      data-role="assistant"
    >
      <div
        className="relative select-text px-1 py-3 text-sm leading-6 [overflow-wrap:anywhere] break-words"
        style={resolveAssistantReplyFontSizeVars(assistantReplyFontSize)}
      >
        <AssistantStructuredBody assistantReplyFontSize={assistantReplyFontSize} />
        <MessageError />
        <MessageCopyAction
          tooltip="Copy"
          side="left"
          positionClass="right-2 top-2"
        />
      </div>
    </MessagePrimitive.Root>
  );
};

const UserMessage: FC = () => {
  return (
    <MessagePrimitive.Root
      className="group mx-auto flex w-full max-w-3xl justify-end py-2"
      data-role="user"
    >
      <div className="relative max-w-[76%]">
        <div className="select-text rounded-[18px] bg-slate-950 px-4 py-3 text-sm leading-6 text-white [overflow-wrap:anywhere] break-words">
          <MessagePrimitive.Parts />
        </div>
        <MessageCopyAction
          tooltip="Copy my message"
          side="left"
          positionClass="right-2 top-2"
          dark
        />
      </div>
    </MessagePrimitive.Root>
  );
};

const MessageCopyAction: FC<{
  tooltip: string;
  side?: "left" | "right" | "top" | "bottom";
  positionClass?: string;
  dark?: boolean;
}> = ({ tooltip, side = "top", positionClass = "right-2 top-2", dark = false }) => {
  const isLast = useMessage((message) => message.isLast);
  const text = useMessage((message) =>
    message.content
      .filter((part) => part.type === "text")
      .map((part) => (part as { type: "text"; text: string }).text)
      .join(""),
  );
  const [isCopied, setIsCopied] = useState(false);

  useEffect(() => {
    if (!isCopied) return;
    const timer = setTimeout(() => setIsCopied(false), 1500);
    return () => clearTimeout(timer);
  }, [isCopied]);

  const onCopy = async () => {
    if (!text) return;
    try {
      await navigator.clipboard.writeText(text);
      setIsCopied(true);
    } catch {
      // ignore copy errors
    }
  };

  return (
    <div
      className={cn(
        "absolute z-0 transition-opacity duration-150",
        positionClass,
        isLast ? "opacity-100" : "opacity-0 group-hover:opacity-100",
      )}
    >
      <TooltipIconButton
        tooltip={tooltip}
        side={side}
        onClick={onCopy}
        className={cn(
          "size-6 rounded-md border",
          dark
            ? "border-white/15 bg-black/32 text-white hover:bg-black/45"
            : "border-slate-200 bg-white/96 text-slate-500 hover:bg-white hover:text-slate-900",
        )}
      >
        {isCopied ? <CheckIcon className="size-3.5" /> : <CopyIcon className="size-3.5" />}
      </TooltipIconButton>
    </div>
  );
};
