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
      className="aui-md min-w-0 max-w-full [overflow-wrap:anywhere]"
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
    <div className="aui-code-header-root mt-5 flex items-center justify-between gap-4 rounded-t-lg border border-b-0 bg-muted px-4 py-2 text-xs text-muted-foreground">
      <span className="aui-code-header-language font-mono lowercase">
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
        "aui-md-h1 mt-6 scroll-m-20 text-3xl font-semibold tracking-tight first:mt-0",
        className,
      )}
      {...props}
    />
  ),
  h2: ({ className, ...props }) => (
    <h2
      className={cn(
        "aui-md-h2 mt-8 scroll-m-20 border-b pb-2 text-2xl font-semibold tracking-tight first:mt-0",
        className,
      )}
      {...props}
    />
  ),
  h3: ({ className, ...props }) => (
    <h3
      className={cn(
        "aui-md-h3 mt-8 scroll-m-20 text-xl font-semibold tracking-tight first:mt-0",
        className,
      )}
      {...props}
    />
  ),
  h4: ({ className, ...props }) => (
    <h4
      className={cn(
        "aui-md-h4 mt-8 scroll-m-20 text-lg font-semibold tracking-tight first:mt-0",
        className,
      )}
      {...props}
    />
  ),
  h5: ({ className, ...props }) => (
    <h5
      className={cn(
        "aui-md-h5 mt-6 text-base font-semibold tracking-tight first:mt-0",
        className,
      )}
      {...props}
    />
  ),
  h6: ({ className, ...props }) => (
    <h6
      className={cn(
        "aui-md-h6 mt-6 text-sm font-semibold tracking-tight first:mt-0",
        className,
      )}
      {...props}
    />
  ),
  p: ({ className, ...props }) => (
    <p
      className={cn(
        "aui-md-p mt-4 leading-7 [overflow-wrap:anywhere] first:mt-0",
        className,
      )}
      {...props}
    />
  ),
  a: ({ className, ...props }) => (
    <MarkdownAnchor
      className={cn(
        "aui-md-a font-medium text-primary underline underline-offset-4 [overflow-wrap:anywhere]",
        className,
      )}
      {...props}
    />
  ),
  blockquote: ({ className, ...props }) => (
    <blockquote
      className={cn("aui-md-blockquote mt-6 border-l-2 pl-6 italic text-muted-foreground", className)}
      {...props}
    />
  ),
  ul: ({ className, ...props }) => (
    <ul
      className={cn("aui-md-ul my-4 ml-6 list-disc [&>li]:mt-2", className)}
      {...props}
    />
  ),
  ol: ({ className, ...props }) => (
    <ol
      className={cn("aui-md-ol my-4 ml-6 list-decimal [&>li]:mt-2", className)}
      {...props}
    />
  ),
  hr: ({ className, ...props }) => (
    <hr className={cn("aui-md-hr my-6 border-border", className)} {...props} />
  ),
  table: ({ className, ...props }) => (
    <table
      className={cn(
        "aui-md-table my-6 w-full table-fixed rounded-lg border text-sm",
        className,
      )}
      {...props}
    />
  ),
  th: ({ className, ...props }) => (
    <th
      className={cn(
        "aui-md-th border-b bg-muted/50 px-4 py-2 text-left font-medium text-muted-foreground whitespace-normal break-words [[align=center]]:text-center [[align=right]]:text-right",
        className,
      )}
      {...props}
    />
  ),
  td: ({ className, ...props }) => (
    <td
      className={cn(
        "aui-md-td border-b px-4 py-2 text-left align-top whitespace-normal break-words [[align=center]]:text-center [[align=right]]:text-right",
        className,
      )}
      {...props}
    />
  ),
  tr: ({ className, ...props }) => (
    <tr className={cn("aui-md-tr m-0 p-0", className)} {...props} />
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
        "aui-md-pre rounded-b-lg border border-t-0 bg-muted px-4 py-3 text-sm whitespace-pre-wrap [overflow-wrap:anywhere]",
        className,
      )}
      {...props}
    />
  ),
  code: function Code({ className, ...props }) {
    const isCodeBlock = useIsMarkdownCodeBlock();
    return (
      <code
        className={cn(
          !isCodeBlock &&
            "aui-md-inline-code rounded bg-muted px-1.5 py-0.5 font-mono text-[0.9em] [overflow-wrap:anywhere]",
          isCodeBlock && "text-foreground",
          className,
        )}
        {...props}
      />
    );
  },
  CodeHeader,
});
