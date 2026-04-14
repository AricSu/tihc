import { DashboardShell } from "./dashboard-shell";

function resolveSection(
  search: string | undefined,
): "dashboard" | "usage" | "plugin" | "skills" | "llm" {
  if (!search) return "dashboard";
  const section = new URLSearchParams(search).get("section");
  if (section === "usage" || section === "token-usage") return "usage";
  if (section === "skills") return "skills";
  if (section === "llm") return "llm";
  return section === "plugin" ? "plugin" : "dashboard";
}

export default function Options() {
  const section = resolveSection(globalThis.location?.search);
  return <DashboardShell initialSection={section} />;
}
