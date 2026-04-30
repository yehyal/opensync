export const API_BASE_URL = import.meta.env.PUBLIC_API_BASE_URL ?? "http://localhost:3000";

export const DEFAULT_REDIRECT_URI =
  import.meta.env.PUBLIC_DESKTOP_REDIRECT_URI ?? "opensync://auth/callback";

export function resolveRedirectUri(currentUrl: URL): string {
  return currentUrl.searchParams.get("redirect_uri") ?? DEFAULT_REDIRECT_URI;
}

export function redirectWithTokens(
  redirectUri: string,
  tokens: { accessToken: string; refreshToken?: string },
): string {
  const url = new URL(redirectUri);
  url.searchParams.set("access_token", tokens.accessToken);
  if (tokens.refreshToken) {
    url.searchParams.set("refresh_token", tokens.refreshToken);
  }
  return url.toString();
}
