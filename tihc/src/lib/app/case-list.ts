type CaseListItem = {
  archivedAt: string | null;
  activityState?: string;
  isPlaceholder?: boolean;
  title?: string;
  updatedAt: string;
};

type UpdatedAtItem = {
  updatedAt: string;
};

export function compareByUpdatedAtDesc<T extends UpdatedAtItem>(a: T, b: T): number {
  return Date.parse(b.updatedAt) - Date.parse(a.updatedAt);
}

export function sortCasesByUpdatedAtDesc<T extends UpdatedAtItem>(cases: T[]): T[] {
  return [...cases].sort(compareByUpdatedAtDesc);
}

export function isPlaceholderCase<T extends CaseListItem>(caseItem: T): boolean {
  if (caseItem.isPlaceholder === true) return true;
  return (
    caseItem.activityState === "ready" &&
    typeof caseItem.title === "string" &&
    caseItem.title.trim().toLowerCase() === "default"
  );
}

export function listOpenCases<T extends CaseListItem>(cases: T[]): T[] {
  return cases.filter((item) => item.archivedAt === null).sort(compareByUpdatedAtDesc);
}

export function listVisibleCases<T extends CaseListItem>(cases: T[]): T[] {
  return listOpenCases(cases).filter((item) => !isPlaceholderCase(item));
}
