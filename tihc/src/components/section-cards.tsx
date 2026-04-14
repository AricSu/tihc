import { Badge } from "@/components/ui/badge"
import {
  Card,
  CardAction,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import type { DashboardCaseRecord } from "@/lib/app/cases-api"

const NO_THREAD_ACTIVITY_SUMMARY = "No thread activity yet."

function countCases(cases: DashboardCaseRecord[], status: DashboardCaseRecord["status"]): number {
  return cases.filter((item) => item.status === status).length
}

function formatUpdatedAt(value: string): string {
  const parsed = Date.parse(value)
  if (Number.isNaN(parsed)) return "Unknown"

  return new Intl.DateTimeFormat("en-US", {
    month: "short",
    day: "numeric",
    hour: "numeric",
    minute: "2-digit",
  }).format(new Date(parsed))
}

function normalizeDashboardSummary(summary: string | null | undefined): string {
  if (!summary || summary === NO_THREAD_ACTIVITY_SUMMARY) {
    return "Case activity will appear here once the investigation starts."
  }
  return summary
}

export function SectionCards({ cases }: { cases: DashboardCaseRecord[] }) {
  const totalCases = cases.length
  const investigatingCases = countCases(cases, "Investigating")
  const resolvedCases = countCases(cases, "Resolved")
  const latestCase = [...cases].sort((a, b) => Date.parse(b.updatedAt) - Date.parse(a.updatedAt))[0] ?? null
  const targets = Array.from(new Set(cases.map((item) => item.executionTarget)))

  return (
    <div className="grid grid-cols-1 gap-4 px-4 lg:px-6 @xl/main:grid-cols-2 @5xl/main:grid-cols-4">
      <Card>
        <CardHeader>
          <CardDescription>Total cases</CardDescription>
          <CardTitle className="text-3xl font-semibold tabular-nums">{totalCases}</CardTitle>
          <CardAction>
            <Badge variant="outline">{totalCases ? "Live" : "Empty"}</Badge>
          </CardAction>
        </CardHeader>
        <CardFooter className="text-sm text-muted-foreground">
          Visible workspaces currently loaded into the dashboard.
        </CardFooter>
      </Card>

      <Card>
        <CardHeader>
          <CardDescription>Investigations in progress</CardDescription>
          <CardTitle className="text-3xl font-semibold tabular-nums">{investigatingCases}</CardTitle>
          <CardAction>
            <Badge variant="outline">{resolvedCases} resolved</Badge>
          </CardAction>
        </CardHeader>
        <CardFooter className="text-sm text-muted-foreground">
          {latestCase
            ? `${latestCase.title} was the most recently updated case.`
            : "No case activity has been recorded yet."}
        </CardFooter>
      </Card>

      <Card>
        <CardHeader>
          <CardDescription>Execution targets</CardDescription>
          <CardTitle className="text-3xl font-semibold tabular-nums">{targets.length}</CardTitle>
          <CardAction>
            <Badge variant="outline">{targets[0] ?? "None"}</Badge>
          </CardAction>
        </CardHeader>
        <CardFooter className="text-sm text-muted-foreground">
          {targets.length
            ? `Active across ${targets.join(", ")}.`
            : "Connect a plugin target to begin tracking cases."}
        </CardFooter>
      </Card>

      <Card>
        <CardHeader>
          <CardDescription>Latest update</CardDescription>
          <CardTitle className="text-xl font-semibold">
            {latestCase ? formatUpdatedAt(latestCase.updatedAt) : "No updates yet"}
          </CardTitle>
          <CardAction>
            <Badge variant="outline">{latestCase?.status ?? "Idle"}</Badge>
          </CardAction>
        </CardHeader>
        <CardFooter className="text-sm text-muted-foreground">
          {normalizeDashboardSummary(latestCase?.summary)}
        </CardFooter>
      </Card>
    </div>
  )
}
