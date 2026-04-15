import { ComposerAddAttachment, ComposerAttachments, UserMessageAttachments } from "@/components/assistant-ui/attachment";
import { MarkdownText } from "@/components/assistant-ui/markdown-text";
import { ToolFallback } from "@/components/assistant-ui/tool-fallback";
import { TooltipIconButton } from "@/components/assistant-ui/tooltip-icon-button";
import { Button } from "@/components/ui/button";
import {
  ActionBarPrimitive,
  ComposerPrimitive,
  ErrorPrimitive,
  MessagePrimitive,
  ThreadPrimitive,
  useAssistantState,
  useThreadViewportStore,
} from "@assistant-ui/react";
import {
  ArrowDownIcon,
  ArrowUpIcon,
  CheckIcon,
  CopyIcon,
  PencilIcon,
  RefreshCcwIcon,
  SquareIcon,
} from "lucide-react";
import { useEffect, type FC, type ReactNode } from "react";

type ThreadProps = {
  composerStart?: ReactNode;
  composerEnd?: ReactNode;
};

export function Thread({ composerStart, composerEnd }: ThreadProps) {
  return (
    <ThreadPrimitive.Root className="aui-thread-root bg-background box-border flex h-full flex-col overflow-hidden">
      <ThreadPrimitive.Viewport
        autoScroll
        turnAnchor="bottom"
        scrollToBottomOnInitialize
        scrollToBottomOnRunStart
        scrollToBottomOnThreadSwitch
        className="aui-thread-viewport flex min-h-0 flex-1 flex-col overflow-y-auto px-4"
      >
        <ThreadPrimitive.Empty>
          <ThreadWelcome />
        </ThreadPrimitive.Empty>

        <ThreadAutoScrollOnOpen />

        <ThreadPrimitive.Messages
          components={{
            UserMessage,
            AssistantMessage,
          }}
        />

        <ThreadPrimitive.ViewportFooter className="aui-thread-viewport-footer sticky bottom-0 mt-auto px-4 pb-4">
          <div className="relative mx-auto w-full max-w-3xl">
            <div className="pointer-events-none absolute inset-x-0 -top-12 flex justify-center">
              <div className="pointer-events-auto">
                <ThreadScrollToBottom />
              </div>
            </div>
            <Composer composerStart={composerStart} composerEnd={composerEnd} />
          </div>
        </ThreadPrimitive.ViewportFooter>
      </ThreadPrimitive.Viewport>
    </ThreadPrimitive.Root>
  );
}

export const ThreadWelcome: FC = () => {
  return (
    <div className="aui-thread-welcome-root mx-auto my-auto flex w-full max-w-3xl flex-col items-center justify-center px-4 py-10 text-center">
      <p className="aui-thread-welcome-text text-foreground text-2xl font-semibold">
        Hello there!
      </p>
      <p className="aui-thread-welcome-subtext text-muted-foreground text-sm">
        How can I help you today?
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
        className="aui-thread-scroll-to-bottom rounded-full disabled:invisible"
      >
        <ArrowDownIcon />
      </TooltipIconButton>
    </ThreadPrimitive.ScrollToBottom>
  );
};

const ThreadAutoScrollOnOpen: FC = () => {
  const threadViewportStore = useThreadViewportStore();

  useEffect(() => {
    let frameA = 0;
    let frameB = 0;

    const scrollToBottom = () => {
      cancelAnimationFrame(frameA);
      cancelAnimationFrame(frameB);

      frameA = requestAnimationFrame(() => {
        threadViewportStore.getState().scrollToBottom({ behavior: "instant" });
        frameB = requestAnimationFrame(() => {
          threadViewportStore.getState().scrollToBottom({ behavior: "instant" });
        });
      });
    };

    const handleVisibilityChange = () => {
      if (document.visibilityState !== "visible") return;
      scrollToBottom();
    };

    scrollToBottom();
    window.addEventListener("focus", scrollToBottom);
    document.addEventListener("visibilitychange", handleVisibilityChange);

    return () => {
      cancelAnimationFrame(frameA);
      cancelAnimationFrame(frameB);
      window.removeEventListener("focus", scrollToBottom);
      document.removeEventListener("visibilitychange", handleVisibilityChange);
    };
  }, [threadViewportStore]);

  return null;
};

type ComposerProps = {
  composerStart?: ReactNode;
  composerEnd?: ReactNode;
};

const Composer: FC<ComposerProps> = ({ composerStart, composerEnd }) => {
  return (
    <ComposerPrimitive.Root className="aui-composer-root relative flex w-full flex-col gap-2">
      <ComposerPrimitive.AttachmentDropzone>
        <div className="aui-composer-inner rounded-2xl border bg-background p-2 shadow-sm">
          <ComposerAttachments />
          <ComposerPrimitive.Input
            rows={1}
            autoFocus
            placeholder="Write a message..."
            aria-label="Message input"
            className="aui-composer-input min-h-12 w-full resize-none border-0 bg-transparent px-3 py-2 text-sm outline-none"
          />
          <ComposerActions composerStart={composerStart} composerEnd={composerEnd} />
        </div>
      </ComposerPrimitive.AttachmentDropzone>
    </ComposerPrimitive.Root>
  );
};

type ComposerActionsProps = {
  composerStart?: ReactNode;
  composerEnd?: ReactNode;
};

const ComposerActions: FC<ComposerActionsProps> = ({ composerStart, composerEnd }) => {
  const isRunning = useAssistantState(({ thread }) => thread.isRunning);

  return (
    <div className="aui-composer-actions mt-2 flex items-center justify-between px-1">
      {composerStart ?? <ComposerAddAttachment />}

      <div className="flex items-center gap-2">
        {composerEnd}
        {isRunning ? (
          <ComposerPrimitive.Cancel asChild>
            <Button
              type="button"
              variant="outline"
              size="icon"
              className="rounded-full"
              aria-label="Stop generating"
            >
              <SquareIcon className="size-4 fill-current" />
            </Button>
          </ComposerPrimitive.Cancel>
        ) : null}
        {!isRunning ? (
          <ComposerPrimitive.Send asChild>
            <Button
              type="button"
              size="icon"
              className="rounded-full"
              aria-label="Send message"
            >
              <ArrowUpIcon className="size-4" />
            </Button>
          </ComposerPrimitive.Send>
        ) : null}
      </div>
    </div>
  );
};

const EditComposer: FC = () => {
  return (
    <div className="aui-edit-composer-root mb-4 rounded-2xl border bg-background p-3">
      <ComposerPrimitive.Input
        rows={1}
        autoFocus
        aria-label="Edit message"
        className="aui-edit-composer-input min-h-12 w-full resize-none border-0 bg-transparent px-1 py-2 text-sm outline-none"
      />
      <div className="mt-3 flex items-center justify-end gap-2">
        <ComposerPrimitive.Cancel asChild>
          <Button type="button" variant="outline">
            Cancel
          </Button>
        </ComposerPrimitive.Cancel>
        <ComposerPrimitive.Send asChild>
          <Button type="button">Save and submit</Button>
        </ComposerPrimitive.Send>
      </div>
    </div>
  );
};

const MessageError: FC = () => {
  return (
    <MessagePrimitive.Error>
      <ErrorPrimitive.Root className="mt-3 rounded-md border border-destructive/30 bg-destructive/10 p-2 text-xs text-destructive">
        <ErrorPrimitive.Message />
      </ErrorPrimitive.Root>
    </MessagePrimitive.Error>
  );
};

const AssistantActionBar: FC = () => {
  return (
    <ActionBarPrimitive.Root
      hideWhenRunning
      autohide="not-last"
      className="aui-assistant-action-bar mt-2 flex items-center gap-1"
    >
      <ActionBarPrimitive.Copy asChild>
        <TooltipIconButton tooltip="Copy">
          <CopyIcon />
        </TooltipIconButton>
      </ActionBarPrimitive.Copy>
      <ActionBarPrimitive.Reload asChild>
        <TooltipIconButton tooltip="Retry">
          <RefreshCcwIcon />
        </TooltipIconButton>
      </ActionBarPrimitive.Reload>
    </ActionBarPrimitive.Root>
  );
};

const UserActionBar: FC = () => {
  return (
    <ActionBarPrimitive.Root
      hideWhenRunning
      autohide="not-last"
      className="aui-user-action-bar mt-2 flex items-center justify-end gap-1"
    >
      <ActionBarPrimitive.Edit asChild>
        <TooltipIconButton tooltip="Edit">
          <PencilIcon />
        </TooltipIconButton>
      </ActionBarPrimitive.Edit>
      <ActionBarPrimitive.Copy asChild>
        <TooltipIconButton tooltip="Copy">
          <CopyIcon />
        </TooltipIconButton>
      </ActionBarPrimitive.Copy>
    </ActionBarPrimitive.Root>
  );
};

const AssistantMessage: FC = () => {
  return (
    <MessagePrimitive.Root className="aui-assistant-message-root py-6">
      <div className="aui-assistant-message min-w-0 max-w-full flex flex-col gap-2">
        <MessagePrimitive.Parts
          components={{
            Text: MarkdownText,
            tools: {
              Fallback: ToolFallback,
            },
          }}
        />
        <MessageError />
        <AssistantActionBar />
      </div>
    </MessagePrimitive.Root>
  );
};

const UserMessage: FC = () => {
  const isEditing = useAssistantState(({ message }) => message.composer.isEditing);

  if (isEditing) {
    return <EditComposer />;
  }

  return (
    <MessagePrimitive.Root className="aui-user-message-root py-6">
      <div className="grid grid-cols-[minmax(0,1fr)] gap-2">
        <UserMessageAttachments />
        <div className="justify-self-end min-w-0 max-w-full rounded-2xl bg-primary px-4 py-3 text-sm text-primary-foreground">
          <MessagePrimitive.Parts />
        </div>
      </div>
      <UserActionBar />
    </MessagePrimitive.Root>
  );
};
