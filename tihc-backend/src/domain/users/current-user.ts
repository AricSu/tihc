import type { PrincipalRecord } from "../cases/case-store";

export type CurrentUserRecord = {
  id: string | null;
  authState: "anonymous" | "authenticated";
  displayName: string;
  email: string;
  hostedDomain: string;
};

export const ANONYMOUS_DISPLAY_NAME = "匿名";

export function deriveDisplayNameFromEmail(email: string): string {
  const trimmed = email.trim();
  if (!trimmed) return "User";

  const [localPart] = trimmed.split("@");
  const normalized = (localPart ?? "")
    .replace(/[._-]+/g, " ")
    .replace(/\s+/g, " ")
    .trim();

  if (!normalized) {
    return trimmed;
  }

  return normalized
    .split(" ")
    .map((segment) =>
      segment ? `${segment.slice(0, 1).toUpperCase()}${segment.slice(1)}` : "",
    )
    .join(" ");
}

export function buildAnonymousCurrentUser(): CurrentUserRecord {
  return {
    id: null,
    authState: "anonymous",
    displayName: ANONYMOUS_DISPLAY_NAME,
    email: "",
    hostedDomain: "",
  };
}

export function buildAuthenticatedCurrentUser(principal: PrincipalRecord): CurrentUserRecord {
  return {
    id: principal.id,
    authState: "authenticated",
    displayName: principal.displayName,
    email: principal.email,
    hostedDomain: principal.hostedDomain,
  };
}
