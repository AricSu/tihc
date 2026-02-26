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
import type { ComponentPropsWithoutRef, FC, MouseEvent } from "react";
import { useEffect, useMemo, useState } from "react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";

export const Thread: FC = () => {
  return (
    <ThreadPrimitive.Root className="flex h-full min-h-0 flex-col overflow-hidden bg-white">
      <ThreadPrimitive.Viewport
        turnAnchor="bottom"
        autoScroll
        scrollToBottomOnInitialize
        scrollToBottomOnRunStart
        className="tihc-scrollbar relative flex flex-1 flex-col overflow-x-hidden overflow-y-auto px-4 pt-4"
      >
        <AssistantIf condition={({ thread }) => thread.isEmpty}>
          <ThreadWelcome />
        </AssistantIf>

        <ThreadPrimitive.Messages
          components={{
            UserMessage,
            AssistantMessage,
          }}
        />

        <ThreadPrimitive.ViewportFooter className="sticky bottom-0 z-40 mt-auto flex flex-col gap-2 bg-white pb-2">
          <ThreadScrollToBottom />
          <Composer />
        </ThreadPrimitive.ViewportFooter>
      </ThreadPrimitive.Viewport>
    </ThreadPrimitive.Root>
  );
};

const ThreadWelcome: FC = () => {
  return (
    <div className="mx-auto my-auto w-full max-w-3xl py-10">
      <h2 className="text-xl font-semibold text-slate-900">开始聊天</h2>
      <p className="mt-2 text-sm text-muted-foreground">
        输入你的 TiDB 问题，消息会直接调用后端 API。
      </p>
    </div>
  );
};

const ThreadScrollToBottom: FC = () => {
  return (
    <ThreadPrimitive.ScrollToBottom asChild>
      <TooltipIconButton
        tooltip="回到底部"
        variant="outline"
        className="absolute -top-11 z-10 self-center rounded-full border border-slate-200 bg-white p-4 shadow-sm disabled:invisible"
      >
        <ArrowDownIcon />
      </TooltipIconButton>
    </ThreadPrimitive.ScrollToBottom>
  );
};

const Composer: FC = () => {
  return (
    <ComposerPrimitive.Root className="relative flex w-full flex-col">
      <div className="rounded-2xl border border-slate-200/80 bg-linear-to-br from-white to-slate-50 p-2.5 shadow-[inset_0_1px_0_rgba(255,255,255,0.95),0_12px_28px_-20px_rgba(15,23,42,0.36)] backdrop-blur-sm">
        <ComposerPrimitive.Input
          placeholder="输入问题..."
          className="aui-composer-input min-h-14 max-h-48 w-full resize-none overflow-y-auto bg-transparent px-2.5 py-1.5 pr-4 text-sm leading-6 text-slate-800 outline-none placeholder:text-slate-400 [scrollbar-width:thin] [scrollbar-color:rgba(100,116,139,0.58)_transparent] [&::-webkit-scrollbar]:w-1.5 [&::-webkit-scrollbar-track]:bg-transparent [&::-webkit-scrollbar-thumb]:rounded-full [&::-webkit-scrollbar-thumb]:bg-slate-400/60 [&::-webkit-scrollbar-thumb:hover]:bg-slate-500/72"
          rows={1}
          autoFocus
          aria-label="Message input"
        />
        <ComposerAction />
      </div>
    </ComposerPrimitive.Root>
  );
};

const ComposerAction: FC = () => {
  return (
    <div className="mt-1 flex items-center justify-end px-1 pb-0.5">
      <AssistantIf condition={({ thread }) => !thread.isRunning}>
        <ComposerPrimitive.Send asChild>
          <TooltipIconButton
            tooltip="发送"
            side="top"
            type="submit"
            variant="default"
            size="icon"
            className="size-8 rounded-full bg-slate-900 text-white shadow-sm hover:bg-slate-800"
            aria-label="Send message"
          >
            <ArrowUpIcon className="size-4" />
          </TooltipIconButton>
        </ComposerPrimitive.Send>
      </AssistantIf>

      <AssistantIf condition={({ thread }) => thread.isRunning}>
        <ComposerPrimitive.Cancel asChild>
          <Button
            type="button"
            variant="default"
            size="icon"
            className="size-8 rounded-full"
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
  const progressMarker = "检索过程：";
  const answerMarker = "回答：";
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
            className={cn("underline underline-offset-2 text-sky-700", className)}
            {...props}
          />
        ),
        code: ({ className, ...props }) => (
          <code className={cn("rounded bg-slate-100 px-1 py-0.5 font-medium text-slate-900", className)} {...props} />
        ),
      }}
    >
      {text}
    </ReactMarkdown>
  );
};

const AssistantStructuredBody: FC = () => {
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
        <section className="rounded-xl border border-slate-200/85 bg-slate-50/70 px-3 py-3">
          <div className="mb-2 flex items-center justify-between">
            <div className="inline-flex items-center gap-2 text-sm font-semibold text-slate-700">
              <ListChecksIcon className="size-4" />
              检索过程
            </div>
            {parsed.steps.length > 1 ? (
              <button
                type="button"
                onClick={() => setCollapsed((prev) => !prev)}
                className="inline-flex items-center gap-1 text-xs text-slate-500 hover:text-slate-700"
              >
                {collapsed ? (
                  <>
                    <ChevronDownIcon className="size-3.5" />
                    展开
                  </>
                ) : (
                  <>
                    <ChevronUpIcon className="size-3.5" />
                    收起
                  </>
                )}
              </button>
            ) : null}
          </div>

          <ul className="relative space-y-2 pl-7">
            <span className="absolute top-1 bottom-1 left-2 w-px bg-slate-200" />
            {visibleSteps.map((step, idx) => {
              const realIdx = idx + offset;
              const isCurrent = isRunning && realIdx === parsed.steps.length - 1;
              const { title, duration } = splitStepDuration(step);
              return (
                <li key={`${realIdx}-${step}`} className="relative">
                  <span className="absolute -left-[1.52rem] top-0.5 rounded-full bg-slate-50">
                    {isCurrent ? (
                      <Loader2Icon className="size-4 text-slate-400 animate-spin" />
                    ) : (
                      <CheckCircle2Icon className="size-4 text-emerald-500" />
                    )}
                  </span>
                  <div className="text-[15px] leading-6 text-slate-800">
                    <StepMarkdown text={title} />
                    {duration ? (
                      <span className="ml-2 text-sm text-slate-400">{duration}</span>
                    ) : null}
                  </div>
                </li>
              );
            })}
          </ul>
        </section>
      ) : null}

      {parsed.answer || !isRunning ? (
        <section className="space-y-3">
          <div className="inline-flex items-center gap-2 text-base font-semibold text-slate-900">
            <ListChecksIcon className="size-4.5" />
            Answer
          </div>

          <div className="rounded-xl border border-slate-200/80 bg-white px-4 py-3 text-[15px] leading-7 text-slate-800">
            <ReactMarkdown
              remarkPlugins={[remarkGfm]}
              components={{
                h1: ({ ...props }) => (
                  <h1 className="mt-6 mb-3 font-semibold text-2xl first:mt-0" {...props} />
                ),
                h2: ({ ...props }) => (
                  <h2 className="mt-5 mb-3 font-semibold text-xl first:mt-0" {...props} />
                ),
                h3: ({ ...props }) => (
                  <h3 className="mt-4 mb-2 font-semibold text-lg first:mt-0" {...props} />
                ),
                p: ({ ...props }) => <p className="mt-4 first:mt-0" {...props} />,
                ul: ({ ...props }) => <ul className="mt-3 ml-5 list-disc space-y-1.5" {...props} />,
                ol: ({ ...props }) => (
                  <ol className="mt-3 ml-5 list-decimal space-y-1.5" {...props} />
                ),
                a: ({ className, ...props }) => (
                  <MarkdownAnchor
                    className={cn("underline underline-offset-2 text-sky-700", className)}
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
                          : "rounded bg-slate-100 px-1 py-0.5 font-medium text-slate-900",
                        className,
                      )}
                      {...props}
                    />
                  );
                },
                pre: ({ ...props }) => (
                  <pre
                    className="mt-4 overflow-x-auto rounded-lg border border-slate-200 bg-slate-100 p-3 text-slate-900"
                    {...props}
                  />
                ),
                blockquote: ({ ...props }) => (
                  <blockquote className="mt-4 border-l-2 border-slate-300 pl-4 text-slate-600" {...props} />
                ),
                hr: ({ ...props }) => <hr className="my-5 border-slate-200" {...props} />,
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

const AssistantMessage: FC = () => {
  return (
    <MessagePrimitive.Root
      className="group mx-auto w-full max-w-3xl py-2"
      data-role="assistant"
    >
      <div className="relative select-text rounded-2xl border border-slate-200/80 bg-white px-3.5 py-2.5 text-sm leading-6 [overflow-wrap:anywhere] break-words shadow-[0_6px_18px_-14px_rgba(15,23,42,0.26)]">
        <AssistantStructuredBody />
        <MessageError />
        <MessageCopyAction
          tooltip="复制"
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
      <div className="relative max-w-[90%]">
        <div className="select-text rounded-2xl bg-linear-to-br from-zinc-900 to-black px-3.5 py-2.5 text-sm text-white leading-6 [overflow-wrap:anywhere] break-words shadow-[0_10px_24px_-16px_rgba(15,23,42,0.75)]">
          <MessagePrimitive.Parts />
        </div>
        <MessageCopyAction
          tooltip="复制我的问题"
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
          "size-7 rounded-md border shadow-sm",
          dark
            ? "border-white/20 bg-black/50 text-white hover:bg-black/70"
            : "border-slate-200 bg-white/90 text-slate-600 hover:bg-white hover:text-slate-900",
        )}
      >
        {isCopied ? <CheckIcon className="size-4" /> : <CopyIcon className="size-4" />}
      </TooltipIconButton>
    </div>
  );
};
