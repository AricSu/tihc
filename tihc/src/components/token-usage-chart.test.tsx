import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, test, vi } from "vitest";

vi.mock("recharts", async (importOriginal) => {
  const actual = await importOriginal<typeof import("recharts")>();
  return {
    ...actual,
    Area: () => <div data-series-kind="area" />,
    AreaChart: ({ children }: { children?: React.ReactNode }) => (
      <div data-chart-kind="area">{children}</div>
    ),
    Bar: () => <div data-series-kind="bar" />,
    BarChart: ({ children }: { children?: React.ReactNode }) => (
      <div data-chart-kind="bar">{children}</div>
    ),
  };
});

import { TokenUsageChart } from "./token-usage-chart";

describe("token usage chart", () => {
  test("renders token activity as a bar chart", () => {
    const html = renderToStaticMarkup(<TokenUsageChart />);

    expect(html).toContain("Token activity");
    expect(html).toContain('data-chart-kind="bar"');
    expect(html).toContain('data-series-kind="bar"');
    expect(html).not.toContain('data-chart-kind="area"');
    expect(html).not.toContain('data-series-kind="area"');
  });
});
