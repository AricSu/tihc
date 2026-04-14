import * as React from "react";
import { z } from "zod";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import { cn } from "@/lib/utils";

const caseMessageSchema = z.object({
  role: z.enum(["operator", "tihc"]),
  text: z.string(),
});

export const schema = z.object({
  id: z.string(),
  title: z.string(),
  status: z.string(),
  priority: z.string(),
  channel: z.string(),
  updatedAt: z.string(),
  executionTarget: z.string(),
  owner: z.string(),
  summary: z.string(),
  signals: z.array(z.string()),
  messages: z.array(caseMessageSchema),
});

type CaseRecord = z.infer<typeof schema>;

function statusVariant(status: string): "default" | "secondary" | "outline" {
  if (status === "Investigating") return "default";
  if (status === "Watching") return "secondary";
  return "outline";
}

function formatUpdatedAt(value: string): string {
  const parsed = Date.parse(value);
  if (Number.isNaN(parsed)) return value;

  return new Intl.DateTimeFormat("en-US", {
    month: "short",
    day: "numeric",
    hour: "numeric",
    minute: "2-digit",
  }).format(new Date(parsed));
}

export function DataTable({ data }: { data: unknown[] }) {
  const caseData = React.useMemo(() => data.map((item) => schema.parse(item)), [data]);
  const [selectedCaseId, setSelectedCaseId] = React.useState<string | null>(
    caseData[0]?.id ?? null,
  );

  const selectedCase =
    caseData.find((item) => item.id === selectedCaseId) ?? caseData[0] ?? null;

  if (!selectedCase) {
    return (
      <div className="px-4 lg:px-6">
        <Card>
          <CardHeader>
            <CardTitle>Cases</CardTitle>
            <CardDescription>No cases are available yet.</CardDescription>
          </CardHeader>
        </Card>
      </div>
    );
  }

  return (
    <div className="px-4 lg:px-6">
      <div className="grid gap-4 xl:grid-cols-[340px_minmax(0,1fr)]">
        <Card>
          <CardHeader>
            <CardTitle>Cases</CardTitle>
            <CardDescription>
              Select a case to inspect the latest snapshot without opening the sidepanel.
            </CardDescription>
          </CardHeader>
          <CardContent className="flex flex-col gap-2">
            {caseData.map((item) => {
              const active = item.id === selectedCase.id;

              return (
                <Button
                  key={item.id}
                  type="button"
                  variant={active ? "secondary" : "ghost"}
                  onClick={() => setSelectedCaseId(item.id)}
                  className="h-auto w-full justify-start rounded-xl px-3 py-3"
                >
                  <div className="flex w-full flex-col gap-3 text-left">
                    <div className="flex items-start justify-between gap-3">
                      <div className="min-w-0">
                        <div className="truncate text-sm font-medium">{item.title}</div>
                        <div className="mt-1 flex flex-wrap gap-2">
                          <Badge variant="outline">{item.channel}</Badge>
                          <Badge variant="outline">{item.priority}</Badge>
                        </div>
                      </div>
                      <Badge variant={statusVariant(item.status)}>{item.status}</Badge>
                    </div>
                    <p className="line-clamp-2 text-sm text-muted-foreground">{item.summary}</p>
                    <div className="flex items-center justify-between gap-3 text-xs text-muted-foreground">
                      <span className="truncate">{item.executionTarget}</span>
                      <span className="shrink-0">{formatUpdatedAt(item.updatedAt)}</span>
                    </div>
                  </div>
                </Button>
              );
            })}
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <div className="flex flex-wrap items-center gap-2">
              <Badge variant={statusVariant(selectedCase.status)}>{selectedCase.status}</Badge>
              <Badge variant="outline">{selectedCase.priority}</Badge>
              <Badge variant="outline">{selectedCase.channel}</Badge>
            </div>
            <CardTitle className="text-2xl">{selectedCase.title}</CardTitle>
            <CardDescription>{selectedCase.summary}</CardDescription>
          </CardHeader>
          <CardContent className="flex flex-col gap-6">
            <div className="grid gap-3 sm:grid-cols-2 xl:grid-cols-4">
              <CaseMeta label="Execution target" value={selectedCase.executionTarget} />
              <CaseMeta label="Owner" value={selectedCase.owner} />
              <CaseMeta label="Last update" value={formatUpdatedAt(selectedCase.updatedAt)} />
              <CaseMeta label="Case ID" value={selectedCase.id} />
            </div>

            <Separator />

            <section className="flex flex-col gap-3">
              <div>
                <h3 className="text-sm font-medium">Recent signals</h3>
                <p className="text-sm text-muted-foreground">
                  The latest facts the sidepanel thread is working from.
                </p>
              </div>
              <div className="flex flex-col gap-2">
                {selectedCase.signals.map((signal) => (
                  <div
                    key={signal}
                    className="rounded-xl border border-border bg-muted/40 px-4 py-3 text-sm"
                  >
                    {signal}
                  </div>
                ))}
              </div>
            </section>

            <Separator />

            <section className="flex flex-col gap-3">
              <div>
                <h3 className="text-sm font-medium">Case preview</h3>
                <p className="text-sm text-muted-foreground">
                  A lightweight, thread-like snapshot for the selected case.
                </p>
              </div>
              <div className="flex flex-col gap-3">
                {selectedCase.messages.map((message, index) => {
                  const isAssistant = message.role === "tihc";

                  return (
                    <div
                      key={`${selectedCase.id}:${index}`}
                      className={cn(
                        "max-w-[92%] rounded-2xl border px-4 py-3 text-sm leading-6 shadow-xs",
                        isAssistant ? "bg-muted/50" : "ml-auto bg-background",
                      )}
                    >
                      <div className="mb-2 text-[11px] font-medium tracking-[0.12em] text-muted-foreground uppercase">
                        {isAssistant ? "TiHC" : "Operator"}
                      </div>
                      <p>{message.text}</p>
                    </div>
                  );
                })}
              </div>
            </section>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}

function CaseMeta({ label, value }: { label: string; value: string }) {
  return (
    <div className="rounded-xl border border-border bg-muted/30 px-4 py-3">
      <div className="text-[11px] font-medium tracking-[0.12em] text-muted-foreground uppercase">
        {label}
      </div>
      <div className="mt-2 text-sm font-medium">{value}</div>
    </div>
  );
}
