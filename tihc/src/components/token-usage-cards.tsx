import { IconTrendingDown, IconTrendingUp } from "@tabler/icons-react";
import type { StoredUsageSummaryRecord, UsagePeriodTotals } from "@/lib/chat/agent-types";
import { Badge } from "@/components/ui/badge";
import {
  Card,
  CardAction,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

function emptyTotals(): UsagePeriodTotals {
  return {
    cachedInputTokens: 0,
    costUsd: 0,
    inputTokens: 0,
    outputTokens: 0,
    reasoningTokens: 0,
    requestCount: 0,
    totalTokens: 0,
  };
}

function formatCompactNumber(value: number): string {
  if (!Number.isFinite(value)) return "0";
  return new Intl.NumberFormat("en-US", {
    maximumFractionDigits: value >= 1000 ? 1 : 0,
    notation: value >= 1000 ? "compact" : "standard",
  }).format(value);
}

function formatCurrency(value: number): string {
  return new Intl.NumberFormat("en-US", {
    currency: "USD",
    maximumFractionDigits: 2,
    minimumFractionDigits: 2,
    style: "currency",
  }).format(value);
}

function formatTrend(current: number, previous: number): string {
  if (previous <= 0) {
    return current > 0 ? "+100.0%" : "0.0%";
  }

  const percent = ((current - previous) / previous) * 100;
  return `${percent >= 0 ? "+" : ""}${percent.toFixed(1)}%`;
}

function renderTrendIcon(isUp: boolean) {
  return isUp ? <IconTrendingUp /> : <IconTrendingDown />;
}

export function TokenUsageCards({
  summary = null,
}: {
  summary?: StoredUsageSummaryRecord | null;
}) {
  const current = summary?.current ?? emptyTotals();
  const previous = summary?.previous ?? emptyTotals();
  const totalTrendUp = current.totalTokens >= previous.totalTokens;
  const promptTrendUp = current.inputTokens >= previous.inputTokens;
  const completionTrendUp = current.outputTokens >= previous.outputTokens;
  const spendTrendUp = current.costUsd >= previous.costUsd;

  return (
    <div className="grid grid-cols-1 gap-4 px-4 *:data-[slot=card]:bg-gradient-to-t *:data-[slot=card]:from-primary/5 *:data-[slot=card]:to-card *:data-[slot=card]:shadow-xs lg:px-6 @xl/main:grid-cols-2 @5xl/main:grid-cols-4 dark:*:data-[slot=card]:bg-card">
      <Card className="@container/card">
        <CardHeader>
          <CardDescription>Total Tokens</CardDescription>
          <CardTitle className="text-2xl font-semibold tabular-nums @[250px]/card:text-3xl">
            {formatCompactNumber(current.totalTokens)}
          </CardTitle>
          <CardAction>
            <Badge variant="outline">
              {renderTrendIcon(totalTrendUp)}
              {formatTrend(current.totalTokens, previous.totalTokens)}
            </Badge>
          </CardAction>
        </CardHeader>
        <CardFooter className="flex-col items-start gap-1.5 text-sm">
          <div className="line-clamp-1 flex gap-2 font-medium">
            {current.requestCount} tracked requests in the current window
          </div>
          <div className="text-muted-foreground">Across authenticated usage records</div>
        </CardFooter>
      </Card>
      <Card className="@container/card">
        <CardHeader>
          <CardDescription>Prompt Tokens</CardDescription>
          <CardTitle className="text-2xl font-semibold tabular-nums @[250px]/card:text-3xl">
            {formatCompactNumber(current.inputTokens)}
          </CardTitle>
          <CardAction>
            <Badge variant="outline">
              {renderTrendIcon(promptTrendUp)}
              {formatTrend(current.inputTokens, previous.inputTokens)}
            </Badge>
          </CardAction>
        </CardHeader>
        <CardFooter className="flex-col items-start gap-1.5 text-sm">
          <div className="line-clamp-1 flex gap-2 font-medium">
            Cache reads: {formatCompactNumber(current.cachedInputTokens)}
          </div>
          <div className="text-muted-foreground">Provider-reported input token volume</div>
        </CardFooter>
      </Card>
      <Card className="@container/card">
        <CardHeader>
          <CardDescription>Completion Tokens</CardDescription>
          <CardTitle className="text-2xl font-semibold tabular-nums @[250px]/card:text-3xl">
            {formatCompactNumber(current.outputTokens)}
          </CardTitle>
          <CardAction>
            <Badge variant="outline">
              {renderTrendIcon(completionTrendUp)}
              {formatTrend(current.outputTokens, previous.outputTokens)}
            </Badge>
          </CardAction>
        </CardHeader>
        <CardFooter className="flex-col items-start gap-1.5 text-sm">
          <div className="line-clamp-1 flex gap-2 font-medium">
            Reasoning tokens: {formatCompactNumber(current.reasoningTokens)}
          </div>
          <div className="text-muted-foreground">Provider-reported output token volume</div>
        </CardFooter>
      </Card>
      <Card className="@container/card">
        <CardHeader>
          <CardDescription>Estimated Spend</CardDescription>
          <CardTitle className="text-2xl font-semibold tabular-nums @[250px]/card:text-3xl">
            {formatCurrency(current.costUsd)}
          </CardTitle>
          <CardAction>
            <Badge variant="outline">
              {renderTrendIcon(spendTrendUp)}
              {formatTrend(current.costUsd, previous.costUsd)}
            </Badge>
          </CardAction>
        </CardHeader>
        <CardFooter className="flex-col items-start gap-1.5 text-sm">
          <div className="line-clamp-1 flex gap-2 font-medium">
            Billing stays zero until pricing rules are attached
          </div>
          <div className="text-muted-foreground">Recorded spend when pricing metadata is available</div>
        </CardFooter>
      </Card>
    </div>
  );
}
