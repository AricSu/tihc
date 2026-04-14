"use client";

import "@assistant-ui/react-markdown/styles/dot.css";

import {
  type CodeHeaderProps,
  MarkdownTextPrimitive,
  unstable_memoizeMarkdownComponents as memoizeMarkdownComponents,
  useIsMarkdownCodeBlock,
} from "@assistant-ui/react-markdown";
import remarkGfm from "remark-gfm";
import { type ComponentPropsWithoutRef, type FC, type MouseEvent, memo, useState } from "react";
import { CheckIcon, CopyIcon } from "lucide-react";

import {
  isExternalHttpUrl,
  openExternalUrl,
  scrollToHashTarget,
  toTrackedExternalUrl,
} from "@/lib/app/external-link";
import { TooltipIconButton } from "@/components/assistant-ui/tooltip-icon-button";
import { cn } from "@/lib/utils";

const MarkdownAnchor: FC<ComponentPropsWithoutRef<"a">> = ({
  href,
  onClick,
  className,
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

const MarkdownTextImpl = () => {
  return (
    <MarkdownTextPrimitive
      remarkPlugins={[remarkGfm]}
      className="aui-md"
      components={defaultComponents}
    />
  );
};

export const MarkdownText = memo(MarkdownTextImpl);

const CodeHeader: FC<CodeHeaderProps> = ({ language, code }) => {
  const { isCopied, copyToClipboard } = useCopyToClipboard();
  const onCopy = () => {
    if (!code || isCopied) return;
    copyToClipboard(code);
  };

  return (
    <div className="aui-code-header-root mt-5 flex items-center justify-between gap-4 rounded-t-2xl border border-slate-200 border-b-0 bg-slate-50 px-4 py-2 text-[11px] font-medium tracking-[0.08em] text-slate-500 uppercase">
      <span className="aui-code-header-language lowercase tracking-[0.04em] [&>span]:text-[10px]">
        {language}
      </span>
      <TooltipIconButton tooltip="Copy" onClick={onCopy}>
        {!isCopied && <CopyIcon />}
        {isCopied && <CheckIcon />}
      </TooltipIconButton>
    </div>
  );
};

const useCopyToClipboard = ({
  copiedDuration = 3000,
}: {
  copiedDuration?: number;
} = {}) => {
  const [isCopied, setIsCopied] = useState<boolean>(false);

  const copyToClipboard = (value: string) => {
    if (!value) return;

    navigator.clipboard.writeText(value).then(() => {
      setIsCopied(true);
      setTimeout(() => setIsCopied(false), copiedDuration);
    });
  };

  return { isCopied, copyToClipboard };
};

const defaultComponents = memoizeMarkdownComponents({
  h1: ({ className, ...props }) => (
    <h1
      className={cn(
        "tihc-display aui-md-h1 mb-8 scroll-m-20 text-[2.15rem] leading-[1.05] font-semibold tracking-[-0.05em] text-slate-950 last:mb-0",
        className,
      )}
      style={{
        fontSize: "var(--tihc-assistant-h1-font-size)",
        lineHeight: "1.05",
      }}
      {...props}
    />
  ),
  h2: ({ className, ...props }) => (
    <h2
      className={cn(
        "tihc-display aui-md-h2 mt-10 mb-4 scroll-m-20 text-[1.7rem] leading-[1.1] font-semibold tracking-[-0.045em] text-slate-950 first:mt-0 last:mb-0",
        className,
      )}
      style={{
        fontSize: "var(--tihc-assistant-h2-font-size)",
        lineHeight: "1.1",
      }}
      {...props}
    />
  ),
  h3: ({ className, ...props }) => (
    <h3
      className={cn(
        "aui-md-h3 mt-8 mb-3 scroll-m-20 text-[1.15rem] leading-7 font-semibold tracking-[-0.02em] text-slate-900 first:mt-0 last:mb-0",
        className,
      )}
      style={{
        fontSize: "var(--tihc-assistant-h3-font-size)",
        lineHeight: "1.75",
      }}
      {...props}
    />
  ),
  h4: ({ className, ...props }) => (
    <h4
      className={cn(
        "aui-md-h4 mt-7 mb-3 scroll-m-20 text-[1rem] leading-7 font-semibold text-slate-900 first:mt-0 last:mb-0",
        className,
      )}
      {...props}
    />
  ),
  h5: ({ className, ...props }) => (
    <h5
      className={cn(
        "aui-md-h5 my-4 text-[0.95rem] font-semibold text-slate-900 first:mt-0 last:mb-0",
        className,
      )}
      {...props}
    />
  ),
  h6: ({ className, ...props }) => (
    <h6
      className={cn(
        "aui-md-h6 my-4 text-[0.9rem] font-semibold text-slate-900 first:mt-0 last:mb-0",
        className,
      )}
      {...props}
    />
  ),
  p: ({ className, ...props }) => (
    <p
      className={cn(
        "aui-md-p mt-5 mb-5 text-[15px] leading-8 text-slate-700 first:mt-0 last:mb-0",
        className,
      )}
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
        "aui-md-a font-medium text-slate-900 underline decoration-slate-300 underline-offset-4 transition hover:decoration-slate-900",
        className,
      )}
      {...props}
    />
  ),
  blockquote: ({ className, ...props }) => (
    <blockquote
      className={cn("aui-md-blockquote mt-5 border-l border-slate-300 pl-5 text-slate-500 italic", className)}
      style={{
        fontSize: "var(--tihc-assistant-body-font-size)",
        lineHeight: "var(--tihc-assistant-body-line-height)",
      }}
      {...props}
    />
  ),
  ul: ({ className, ...props }) => (
    <ul
      className={cn("aui-md-ul my-5 ml-5 list-disc space-y-2 text-[15px] leading-8 text-slate-700", className)}
      style={{
        fontSize: "var(--tihc-assistant-body-font-size)",
        lineHeight: "var(--tihc-assistant-body-line-height)",
      }}
      {...props}
    />
  ),
  ol: ({ className, ...props }) => (
    <ol
      className={cn("aui-md-ol my-5 ml-5 list-decimal space-y-2 text-[15px] leading-8 text-slate-700", className)}
      style={{
        fontSize: "var(--tihc-assistant-body-font-size)",
        lineHeight: "var(--tihc-assistant-body-line-height)",
      }}
      {...props}
    />
  ),
  hr: ({ className, ...props }) => (
    <hr className={cn("aui-md-hr my-8 border-b border-slate-200", className)} {...props} />
  ),
  table: ({ className, ...props }) => (
    <table
      className={cn(
        "aui-md-table my-6 w-full border-separate border-spacing-0 overflow-y-auto text-[14px] leading-7",
        className,
      )}
      style={{
        fontSize: "var(--tihc-assistant-body-font-size)",
        lineHeight: "var(--tihc-assistant-body-line-height)",
      }}
      {...props}
    />
  ),
  th: ({ className, ...props }) => (
    <th
      className={cn(
        "aui-md-th bg-slate-50 px-4 py-2 text-left text-[11px] font-semibold tracking-[0.08em] text-slate-500 uppercase first:rounded-tl-2xl last:rounded-tr-2xl [[align=center]]:text-center [[align=right]]:text-right",
        className,
      )}
      {...props}
    />
  ),
  td: ({ className, ...props }) => (
    <td
      className={cn(
        "aui-md-td border-b border-l border-slate-200 px-4 py-2.5 text-left text-slate-700 last:border-r [[align=center]]:text-center [[align=right]]:text-right",
        className,
      )}
      style={{
        fontSize: "var(--tihc-assistant-body-font-size)",
        lineHeight: "var(--tihc-assistant-body-line-height)",
      }}
      {...props}
    />
  ),
  tr: ({ className, ...props }) => (
    <tr
      className={cn(
        "aui-md-tr m-0 border-b p-0 first:border-t [&:last-child>td:first-child]:rounded-bl-2xl [&:last-child>td:last-child]:rounded-br-2xl",
        className,
      )}
      {...props}
    />
  ),
  sup: ({ className, ...props }) => (
    <sup
      className={cn("aui-md-sup [&>a]:text-xs [&>a]:no-underline", className)}
      {...props}
    />
  ),
  pre: ({ className, ...props }) => (
    <pre
      className={cn(
        "aui-md-pre overflow-x-auto rounded-t-none! rounded-b-2xl border border-slate-200 bg-slate-50 p-4 text-[13px] leading-7 text-slate-900",
        className,
      )}
      style={{
        fontSize: "var(--tihc-assistant-code-font-size)",
        lineHeight: "1.75",
      }}
      {...props}
    />
  ),
  code: function Code({ className, ...props }) {
    const isCodeBlock = useIsMarkdownCodeBlock();
    return (
      <code
        className={cn(
          !isCodeBlock &&
            "aui-md-inline-code rounded-md bg-slate-100 px-1.5 py-0.5 font-medium text-slate-900",
          isCodeBlock && "text-slate-900",
          className,
        )}
        {...props}
      />
    );
  },
  CodeHeader,
});
