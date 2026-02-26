const UTM_PARAMS = {
  utm_source: "tihc_extension",
  utm_medium: "assistant_ui",
  utm_campaign: "tihc_referral",
  utm_content: "chat_link",
} as const;

export function isExternalHttpUrl(rawUrl: string): boolean {
  return /^https?:\/\//i.test(rawUrl.trim());
}

export function toTrackedExternalUrl(rawUrl: string): string {
  if (!rawUrl) return rawUrl;
  const normalizedRawUrl = rawUrl.trim();
  if (!isExternalHttpUrl(normalizedRawUrl)) return normalizedRawUrl;

  let url: URL;
  try {
    url = new URL(normalizedRawUrl);
  } catch {
    return normalizedRawUrl;
  }

  for (const [key, value] of Object.entries(UTM_PARAMS)) {
    url.searchParams.set(key, value);
  }

  return url.toString();
}

export function scrollToHashTarget(hashHref: string): boolean {
  if (!hashHref.startsWith("#")) return false;
  const rawId = hashHref.slice(1);
  if (!rawId) return false;

  let targetId = rawId;
  try {
    targetId = decodeURIComponent(rawId);
  } catch {
    targetId = rawId;
  }

  const target = document.getElementById(targetId);
  if (!target) return false;
  target.scrollIntoView({ behavior: "smooth", block: "start" });
  return true;
}

export function openExternalUrl(rawUrl: string): void {
  const trackedUrl = toTrackedExternalUrl(rawUrl);
  const chromeApi = (
    globalThis as unknown as {
      chrome?: {
        tabs?: {
          create?: (options: { url: string }) => void;
        };
      };
    }
  ).chrome;

  try {
    if (chromeApi?.tabs?.create) {
      chromeApi.tabs.create({ url: trackedUrl });
      return;
    }
  } catch {
    // ignore and fallback to window.open
  }

  window.open(trackedUrl, "_blank", "noopener,noreferrer");
}
