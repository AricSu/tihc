export const MAX_BODY_BYTES = 4.5 * 1024 * 1024;
export const STREAM_RESPONSE_HEADERS = {
  "Cache-Control": "no-store",
  "Content-Type": "text/event-stream; charset=utf-8",
};
export const JSON_RESPONSE_HEADERS = {
  "Cache-Control": "no-store",
  "Content-Type": "application/json; charset=utf-8",
};

export function unauthorizedResponse(message: string): Response {
  return new Response(message, {
    headers: {
      "Cache-Control": "no-store",
      "Content-Type": "text/plain; charset=utf-8",
    },
    status: 401,
  });
}

export function jsonError(status: number, message: string): Response {
  return new Response(JSON.stringify({ error: { message } }), {
    headers: JSON_RESPONSE_HEADERS,
    status,
  });
}

async function readBoundedText(
  request: Request,
): Promise<{ ok: true; raw: string } | { ok: false; response: Response }> {
  const raw = await request.text();
  if (new TextEncoder().encode(raw).byteLength > MAX_BODY_BYTES) {
    return {
      ok: false,
      response: jsonError(413, "Request body exceeds the 4.5 MB Vercel Functions limit."),
    };
  }
  return {
    ok: true,
    raw,
  };
}

export async function readBoundedJson<T>(
  request: Request,
): Promise<{ ok: true; value: T } | { ok: false; response: Response }> {
  const raw = await readBoundedText(request);
  if (!raw.ok) return raw;

  try {
    return {
      ok: true,
      value: JSON.parse(raw.raw) as T,
    };
  } catch {
    return {
      ok: false,
      response: jsonError(400, "Invalid JSON request body."),
    };
  }
}
