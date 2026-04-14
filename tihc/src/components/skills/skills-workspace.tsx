import {
  AlertCircleIcon,
  ArrowLeftIcon,
  CheckCircle2Icon,
  Columns2Icon,
  EyeIcon,
  FilePlus2Icon,
  PencilLineIcon,
} from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Textarea } from "@/components/ui/textarea";
import { cn } from "@/lib/utils";

type SkillEditorMode = "write" | "preview" | "split";

type SkillDocument = {
  id: string;
  name: string;
  description: string;
  body: string;
};

type SkillsQueryState = {
  editor: string | null;
  savedSkill: boolean;
};

type DraftParseResult = SkillDocument & {
  error: string | null;
};

export function SkillsWorkspace() {
  const [skills, setSkills] = useState<SkillDocument[]>([]);
  const [queryState, setQueryState] = useState<SkillsQueryState>(() =>
    readSkillsQuery(globalThis.location?.search),
  );

  useEffect(() => {
    if (typeof window === "undefined") return;

    const handlePopState = () => {
      setQueryState(readSkillsQuery(window.location.search));
    };

    window.addEventListener("popstate", handlePopState);
    return () => window.removeEventListener("popstate", handlePopState);
  }, []);

  const visibleSkills = skills;

  const editorState = useMemo(() => {
    if (!queryState.editor) return null;

    if (queryState.editor === "new") {
      return {
        mode: "create" as const,
        skill: createEmptySkillDocument(),
      };
    }

    const existingSkill = skills.find((skill) => skill.id === queryState.editor);
    if (!existingSkill) return null;

    return {
      mode: "edit" as const,
      skill: existingSkill,
    };
  }, [queryState.editor, skills]);

  const [panelMode, setPanelMode] = useState<SkillEditorMode>("write");
  const initialDraft = useMemo(
    () => createSkillEditorDraft(editorState?.skill ?? createEmptySkillDocument()),
    [editorState],
  );
  const [markdownDraft, setMarkdownDraft] = useState(initialDraft);

  useEffect(() => {
    setPanelMode("write");
    setMarkdownDraft(initialDraft);
  }, [initialDraft]);

  const parsedDraft = readSkillDraftForDisplay(
    markdownDraft,
    editorState?.mode === "edit" ? editorState.skill.id : undefined,
  );

  const navigateSkills = (nextState: SkillsQueryState, replace = false) => {
    setQueryState(nextState);
    if (typeof window === "undefined") return;

    const url = new URL(window.location.href);
    url.searchParams.set("section", "skills");

    if (nextState.editor) {
      url.searchParams.set("editor", nextState.editor);
    } else {
      url.searchParams.delete("editor");
    }

    if (nextState.savedSkill) {
      url.searchParams.set("savedSkill", "1");
    } else {
      url.searchParams.delete("savedSkill");
    }

    const nextSearch = url.searchParams.toString();
    const nextUrl = `${url.pathname}${nextSearch ? `?${nextSearch}` : ""}${url.hash}`;

    if (replace) {
      window.history.replaceState({}, "", nextUrl);
    } else {
      window.history.pushState({}, "", nextUrl);
    }
  };

  const resetDraft = () => {
    setPanelMode("write");
    setMarkdownDraft(initialDraft);
  };

  const saveDraft = () => {
    if (!editorState || parsedDraft.error) {
      setPanelMode("preview");
      return;
    }

    const nextSkill: SkillDocument = {
      id: parsedDraft.id,
      name: parsedDraft.name,
      description: parsedDraft.description,
      body: parsedDraft.body,
    };

    setSkills((currentSkills) => {
      const existingIndex = currentSkills.findIndex((skill) => skill.id === nextSkill.id);
      if (existingIndex === -1) {
        return [nextSkill, ...currentSkills];
      }

      return currentSkills.map((skill) => (skill.id === nextSkill.id ? nextSkill : skill));
    });

    navigateSkills({ editor: nextSkill.id, savedSkill: true });
  };

  if (!editorState) {
    return (
      <div className="mx-auto flex w-full max-w-6xl flex-col gap-6">
        {visibleSkills.length ? (
          <>
            <div className="flex justify-end">
              <Button
                type="button"
                onClick={() => navigateSkills({ editor: "new", savedSkill: false })}
              >
                <FilePlus2Icon data-icon="inline-start" />
                Create skill
              </Button>
            </div>
            <div className="grid gap-4 lg:grid-cols-2">
              {visibleSkills.map((skill) => (
                <div
                  key={skill.id}
                  className="flex flex-col gap-4 rounded-3xl border bg-background/90 p-5 shadow-sm"
                >
                  <div className="flex items-start justify-between gap-3">
                    <div className="space-y-2">
                      <div className="flex flex-wrap items-center gap-2">
                        <h3 className="text-base font-semibold">{skill.name}</h3>
                        <Badge variant="secondary">Ready</Badge>
                      </div>
                      <p className="text-sm leading-6 text-muted-foreground">
                        {skill.description}
                      </p>
                    </div>
                    <Button
                      type="button"
                      variant="ghost"
                      size="sm"
                      onClick={() => navigateSkills({ editor: skill.id, savedSkill: false })}
                    >
                      <PencilLineIcon data-icon="inline-start" />
                      Edit
                    </Button>
                  </div>
                </div>
              ))}
            </div>
          </>
        ) : (
          <div className="rounded-3xl border border-dashed bg-muted/20 p-10 text-center">
            <p className="text-base font-medium">No skills yet</p>
            <p className="mt-2 text-sm leading-6 text-muted-foreground">
              Create your first reusable instruction to shape how the agent works with you in future
              turns.
            </p>
            <Button
              type="button"
              className="mt-4"
              onClick={() => navigateSkills({ editor: "new", savedSkill: false })}
            >
              <FilePlus2Icon data-icon="inline-start" />
              Create skill
            </Button>
          </div>
        )}
      </div>
    );
  }

  const isCreate = editorState.mode === "create";
  const title = isCreate ? "Create skill" : "Edit skill";
  const submitLabel = isCreate ? "Create skill" : "Save changes";
  const visibleSkillCountLabel = formatSkillCountLabel(visibleSkills.length);
  const wordCount = countWords(markdownDraft);

  return (
    <div className="mx-auto flex w-full max-w-[120rem] flex-col gap-6">
      <div className="flex flex-wrap items-center justify-between gap-3">
        <Button
          type="button"
          variant="ghost"
          size="sm"
          onClick={() => navigateSkills({ editor: null, savedSkill: false })}
        >
          <ArrowLeftIcon data-icon="inline-start" />
          Back to Knowledge
        </Button>
        <div className="flex flex-wrap items-center gap-2">
          <Badge variant={isCreate ? "outline" : "secondary"}>
            {isCreate ? "New draft" : "Existing skill"}
          </Badge>
          <Badge variant="outline">{visibleSkillCountLabel} saved</Badge>
        </div>
      </div>

      {queryState.savedSkill ? (
        <Alert>
          <CheckCircle2Icon />
          <AlertTitle>Skill saved</AlertTitle>
          <AlertDescription>Your skill is ready to use in future turns.</AlertDescription>
        </Alert>
      ) : null}

      <div className="flex flex-col gap-6">
        <Tabs
          value={panelMode}
          onValueChange={(value) => {
            if (value === "write" || value === "preview" || value === "split") {
              setPanelMode(value);
            }
          }}
          className="grid gap-6 xl:grid-cols-[minmax(0,1fr)_18rem]"
        >
          <div className="overflow-hidden rounded-3xl border bg-background shadow-sm">
            <div className="flex flex-col gap-4 border-b px-6 py-5 lg:flex-row lg:items-start lg:justify-between">
              <div className="flex flex-col gap-2">
                <p className="text-xs font-medium uppercase tracking-[0.24em] text-muted-foreground">
                  Playground
                </p>
                <div className="flex flex-col gap-1">
                  <h1 className="text-2xl font-semibold">{title}</h1>
                  <p className="max-w-2xl text-sm leading-6 text-muted-foreground">
                    {isCreate
                      ? "Write a reusable markdown instruction the agent can apply in future turns when it fits the task."
                      : "Refine this reusable markdown instruction so the agent can keep applying it in future turns."}
                  </p>
                </div>
              </div>
              <div className="flex flex-wrap items-center gap-2">
                <Badge variant="outline">Markdown only</Badge>
                <Badge variant="outline">{getEditorModeLabel(panelMode)}</Badge>
              </div>
            </div>

            <TabsContent value="write" className="m-0">
              <EditorCanvas value={markdownDraft} onChange={setMarkdownDraft} />
            </TabsContent>

            <TabsContent value="preview" className="m-0">
              <PreviewCanvas value={markdownDraft} className="min-h-[68vh] bg-muted/20" />
            </TabsContent>

            <TabsContent value="split" className="m-0">
              <div className="grid min-h-[68vh] lg:grid-cols-2">
                <div className="border-b lg:border-r lg:border-b-0">
                  <EditorCanvas value={markdownDraft} onChange={setMarkdownDraft} borderless />
                </div>
                <PreviewCanvas value={markdownDraft} className="bg-muted/20" />
              </div>
            </TabsContent>

            <div className="flex flex-wrap items-center justify-between gap-3 border-t bg-muted/20 px-6 py-4">
              <div className="flex flex-wrap items-center gap-2 text-sm text-muted-foreground">
                <span>{wordCount ? `${wordCount} words` : "Empty draft"}</span>
                <span aria-hidden="true">·</span>
                <span>{markdownDraft.length} chars</span>
              </div>
              <div className="flex flex-wrap items-center gap-2">
                <Button type="button" variant="ghost" onClick={resetDraft}>
                  Reset draft
                </Button>
                <Button type="button" onClick={saveDraft}>
                  {submitLabel}
                </Button>
              </div>
            </div>
          </div>

          <div className="flex h-fit flex-col gap-4 rounded-3xl border bg-background p-4 shadow-sm">
            <section className="flex flex-col gap-3">
              <div className="flex flex-col gap-1">
                <p className="text-sm font-medium">Mode</p>
                <p className="text-sm text-muted-foreground">
                  Switch between writing, rendering, or the split view.
                </p>
              </div>
              <TabsList className="grid h-11 w-full grid-cols-3">
                <TabsTrigger value="write" className="text-xs">
                  <PencilLineIcon data-icon="inline-start" />
                  Write
                </TabsTrigger>
                <TabsTrigger value="preview" className="text-xs">
                  <EyeIcon data-icon="inline-start" />
                  Preview
                </TabsTrigger>
                <TabsTrigger value="split" className="text-xs">
                  <Columns2Icon data-icon="inline-start" />
                  Split view
                </TabsTrigger>
              </TabsList>
            </section>

            <Separator />

            <section className="flex flex-col gap-3">
              <div className="flex flex-col gap-1">
                <p className="text-sm font-medium">Skill Summary</p>
                <p className="text-sm text-muted-foreground">
                  The markdown header defines the skill name and short description.
                </p>
              </div>
              <div className="flex flex-col gap-3">
                <SidebarStat
                  label="Name"
                  value={parsedDraft.name || "Add `name:` in the markdown header."}
                />
                <SidebarStat
                  label="Description"
                  value={parsedDraft.description || "Add `description:` in the markdown header."}
                />
                <SidebarStat
                  label="Save status"
                  value={
                    parsedDraft.error
                      ? "Add the required markdown header fields before saving."
                      : "Ready to save and reuse in future turns."
                  }
                />
                {parsedDraft.error ? (
                  <Alert variant="destructive">
                    <AlertCircleIcon />
                    <AlertTitle>Markdown header required</AlertTitle>
                    <AlertDescription>{parsedDraft.error}</AlertDescription>
                  </Alert>
                ) : null}
              </div>
            </section>

          </div>
        </Tabs>
      </div>
    </div>
  );
}

function readSkillsQuery(search: string | undefined): SkillsQueryState {
  const params = new URLSearchParams(search ?? "");

  return {
    editor: params.get("editor"),
    savedSkill: params.get("savedSkill") === "1",
  };
}

function createEmptySkillDocument(): SkillDocument {
  return {
    id: "",
    name: "",
    description: "",
    body: "# New skill\nDescribe the situation, outcome, and working style you want the agent to follow.\n",
  };
}

function formatSkillCountLabel(skillCount: number) {
  return `${skillCount} ${skillCount === 1 ? "skill" : "skills"}`;
}

function SidebarStat({
  label,
  value,
  monospace = false,
}: {
  label: string;
  value: string;
  monospace?: boolean;
}) {
  return (
    <div className="rounded-2xl border bg-muted/20 p-3">
      <p className="text-xs font-medium uppercase tracking-[0.24em] text-muted-foreground">
        {label}
      </p>
      <p className={cn("mt-2 text-sm text-muted-foreground", monospace && "break-all font-mono text-xs")}>
        {value}
      </p>
    </div>
  );
}

function EditorCanvas({
  value,
  onChange,
  borderless = false,
}: {
  value: string;
  onChange: (value: string) => void;
  borderless?: boolean;
}) {
  return (
    <div className="flex min-h-[68vh] flex-col">
      <div className="border-b px-6 py-4">
        <p className="text-sm font-medium">Markdown canvas</p>
        <p className="mt-1 text-sm text-muted-foreground">
          Edit the full draft, including the frontmatter at the top.
        </p>
      </div>
      <div className="flex-1">
        <Textarea
          aria-label="Markdown editor"
          value={value}
          onChange={(event) => onChange(event.target.value)}
          className={cn(
            "min-h-[60vh] rounded-none border-0 bg-transparent px-6 py-6 font-mono text-sm leading-6 shadow-none focus-visible:border-transparent focus-visible:ring-0",
            !borderless && "min-h-[68vh]",
          )}
        />
      </div>
    </div>
  );
}

function PreviewCanvas({
  value,
  className,
}: {
  value: string;
  className?: string;
}) {
  const parsedDraft = readSkillDraftForDisplay(value);

  return (
    <div className={cn("flex min-h-[68vh] flex-col", className)}>
      <div className="border-b px-6 py-4">
        <p className="text-sm font-medium">Preview</p>
        <p className="mt-1 text-sm text-muted-foreground">Rendered with GitHub-flavored markdown.</p>
      </div>
      <div className="flex-1 px-6 py-6">
        {parsedDraft.name ? (
          <div className="mb-6 flex flex-col gap-2 border-b pb-4">
            <p className="text-xs font-medium uppercase tracking-[0.24em] text-muted-foreground">
              Frontmatter
            </p>
            <h2 className="text-xl font-semibold">{parsedDraft.name}</h2>
            {parsedDraft.description ? (
              <p className="text-sm leading-6 text-muted-foreground">{parsedDraft.description}</p>
            ) : null}
          </div>
        ) : null}
        {parsedDraft.error ? (
          <Alert variant="destructive">
            <AlertCircleIcon />
            <AlertTitle>Preview unavailable</AlertTitle>
            <AlertDescription>{parsedDraft.error}</AlertDescription>
          </Alert>
        ) : (
          <MarkdownPreview value={parsedDraft.body} />
        )}
      </div>
    </div>
  );
}

function MarkdownPreview({ value }: { value: string }) {
  if (!value.trim()) {
    return (
      <p className="text-sm text-muted-foreground">
        Start typing markdown in the editor to preview it here.
      </p>
    );
  }

  return (
    <div className="max-h-[42rem] overflow-y-auto">
      <ReactMarkdown
        remarkPlugins={[remarkGfm]}
        components={{
          h1: ({ children, ...props }) => (
            <h1 className="mb-3 text-lg font-semibold" {...props}>
              {children}
            </h1>
          ),
          h2: ({ children, ...props }) => (
            <h2 className="mb-2 mt-5 text-sm font-semibold" {...props}>
              {children}
            </h2>
          ),
          h3: ({ children, ...props }) => (
            <h3 className="mb-2 mt-4 text-sm font-medium" {...props}>
              {children}
            </h3>
          ),
          p: ({ children, ...props }) => (
            <p className="my-2 text-sm leading-6 text-foreground/90" {...props}>
              {children}
            </p>
          ),
          ul: ({ children, ...props }) => (
            <ul className="my-2 ml-5 list-disc text-sm text-foreground/90 [&>li]:mt-1" {...props}>
              {children}
            </ul>
          ),
          ol: ({ children, ...props }) => (
            <ol className="my-2 ml-5 list-decimal text-sm text-foreground/90 [&>li]:mt-1" {...props}>
              {children}
            </ol>
          ),
          li: ({ children, ...props }) => (
            <li className="leading-6" {...props}>
              {children}
            </li>
          ),
          code: ({ children, className, ...props }) => (
            <code
              className={cn("rounded bg-muted px-1.5 py-0.5 font-mono text-[0.8rem]", className)}
              {...props}
            >
              {children}
            </code>
          ),
          pre: ({ children, ...props }) => (
            <pre
              className="my-3 overflow-x-auto rounded-lg border bg-muted/50 p-3 text-sm"
              {...props}
            >
              {children}
            </pre>
          ),
          blockquote: ({ children, ...props }) => (
            <blockquote
              className="my-3 border-l-2 border-border pl-3 text-sm text-muted-foreground italic"
              {...props}
            >
              {children}
            </blockquote>
          ),
          table: ({ children, ...props }) => (
            <div className="my-3 overflow-x-auto rounded-lg border">
              <table className="w-full text-sm" {...props}>
                {children}
              </table>
            </div>
          ),
          th: ({ children, ...props }) => (
            <th className="border-b bg-muted/50 px-3 py-2 text-left font-medium" {...props}>
              {children}
            </th>
          ),
          td: ({ children, ...props }) => (
            <td className="border-b px-3 py-2 align-top text-muted-foreground" {...props}>
              {children}
            </td>
          ),
          a: ({ children, href, ...props }) => (
            <a
              className="text-primary underline underline-offset-2"
              href={href}
              {...props}
            >
              {children}
            </a>
          ),
        }}
      >
        {value}
      </ReactMarkdown>
    </div>
  );
}

function getEditorModeLabel(mode: SkillEditorMode) {
  if (mode === "write") return "Write";
  if (mode === "preview") return "Preview";
  return "Split view";
}

function countWords(value: string) {
  const trimmedValue = value.trim();
  if (!trimmedValue) return 0;
  return trimmedValue.split(/\s+/).length;
}

function createSkillEditorDraft(skill: SkillDocument) {
  const normalizedBody = skill.body.endsWith("\n") ? skill.body : `${skill.body}\n`;

  return ["---", `id: ${skill.id}`, `name: ${skill.name}`, `description: ${skill.description}`, "---", normalizedBody].join("\n");
}

function readSkillDraftForDisplay(value: string, existingSkillId?: string): DraftParseResult {
  try {
    const parsedDraft = parseSubmittedSkillDraft(value, existingSkillId);

    return {
      ...parsedDraft,
      error: null,
    };
  } catch (error) {
    return {
      id: existingSkillId?.trim() ?? "",
      name: "",
      description: "",
      body: "",
      error: error instanceof Error ? error.message : "Invalid frontmatter.",
    };
  }
}

function parseSubmittedSkillDraft(value: string, existingSkillId?: string): SkillDocument {
  const parsedDraft = parseSkillDraft(value);
  const name = parsedDraft.metadata.name?.trim();
  const description = parsedDraft.metadata.description?.trim();

  if (!name) {
    throw new Error("The draft must include a `name` in the frontmatter.");
  }

  if (!description) {
    throw new Error("The draft must include a `description` in the frontmatter.");
  }

  return {
    id: existingSkillId?.trim() || parsedDraft.metadata.id?.trim() || slugifySkillId(name),
    name,
    description,
    body: parsedDraft.body,
  };
}

function parseSkillDraft(value: string) {
  const lines = value.split(/\r?\n/);

  if (!lines.length || lines[0]?.trim() !== "---") {
    throw new Error("The draft must start with a frontmatter block.");
  }

  let endIndex = -1;

  for (let index = 1; index < lines.length; index += 1) {
    if (lines[index]?.trim() === "---") {
      endIndex = index;
      break;
    }
  }

  if (endIndex === -1) {
    throw new Error("The frontmatter block must be closed with `---`.");
  }

  const metadata: Record<string, string> = {};

  for (const line of lines.slice(1, endIndex)) {
    const trimmedLine = line.trim();

    if (!trimmedLine || trimmedLine.startsWith("#")) {
      continue;
    }

    const separatorIndex = line.indexOf(":");
    if (separatorIndex === -1) {
      throw new Error("Frontmatter lines must use `key: value`.");
    }

    metadata[line.slice(0, separatorIndex).trim()] = line.slice(separatorIndex + 1).trim();
  }

  const body = lines.slice(endIndex + 1).join("\n");

  return {
    metadata,
    body: body.endsWith("\n") ? body : `${body}\n`,
  };
}

function slugifySkillId(value: string) {
  const normalizedValue = value.trim().toLowerCase();

  return normalizedValue
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "")
    .slice(0, 64);
}
