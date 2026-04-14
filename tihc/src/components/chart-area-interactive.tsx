"use client"

import { Badge } from "@/components/ui/badge"
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import type { DashboardCaseRecord } from "@/lib/app/cases-api"
import { cn } from "@/lib/utils"

const NO_THREAD_ACTIVITY_SUMMARY = "No thread activity yet."

function formatUpdatedAt(value: string): string {
  const parsed = Date.parse(value)
  if (Number.isNaN(parsed)) return value

  return new Intl.DateTimeFormat("en-US", {
    month: "short",
    day: "numeric",
    hour: "numeric",
    minute: "2-digit",
  }).format(new Date(parsed))
}

function normalizeDashboardSummary(summary: string): string {
  return summary === NO_THREAD_ACTIVITY_SUMMARY
    ? "Case activity will appear here once the investigation starts."
    : summary
}

function filterSignals(signals: string[]): string[] {
  return signals.filter((signal) => signal !== NO_THREAD_ACTIVITY_SUMMARY)
}

export function ChartAreaInteractive({ cases }: { cases: DashboardCaseRecord[] }) {
  const recentCases = [...cases]
    .sort((a, b) => Date.parse(b.updatedAt) - Date.parse(a.updatedAt))
    .slice(0, 5)

  return (
    <Card>
      <CardHeader>
        <CardTitle>Recent activity</CardTitle>
        <CardDescription>
          The latest case updates currently available in the dashboard snapshot.
        </CardDescription>
      </CardHeader>
      <CardContent className="flex flex-col gap-3">
        {recentCases.length ? recentCases.map((item) => (
          <div
            key={item.id}
            className="rounded-xl border border-border bg-muted/20 px-4 py-3"
          >
            <div className="flex flex-wrap items-start justify-between gap-3">
              <div className="min-w-0">
                <div className="truncate text-sm font-medium">{item.title}</div>
                <div className="mt-1 text-sm text-muted-foreground">
                  {normalizeDashboardSummary(item.summary)}
                </div>
              </div>
              <div className="flex shrink-0 flex-wrap items-center gap-2">
                <Badge variant="outline">{item.status}</Badge>
                <span className="text-xs text-muted-foreground">{formatUpdatedAt(item.updatedAt)}</span>
              </div>
            </div>
            {filterSignals(item.signals).length ? (
              <div className="mt-3 flex flex-wrap gap-2">
                {filterSignals(item.signals).slice(0, 2).map((signal) => (
                  <div
                    key={`${item.id}:${signal}`}
                    className={cn(
                      "rounded-full border border-border bg-background px-3 py-1 text-xs text-muted-foreground",
                    )}
                  >
                    {signal}
                  </div>
                ))}
              </div>
            ) : null}
          </div>
        )) : (
          <div className="rounded-xl border border-dashed border-border px-4 py-6 text-sm text-muted-foreground">
            No recent case activity yet.
          </div>
        )}
      </CardContent>
    </Card>
  )
}
