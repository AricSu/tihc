export function confirmDeleteCase(caseTitle: string): boolean {
  return globalThis.confirm
    ? globalThis.confirm(`Delete "${caseTitle}"? This clears the local case history.`)
    : true;
}
