import { useState } from "react";

type AuthFormProps = {
  mode: "login" | "signup";
  title: string;
  description: string;
  submitLabel: string;
  alternateHref: string;
  alternateLabel: string;
  apiBaseUrl: string;
  redirectUri: string;
};

export function AuthForm({
  mode,
  title,
  description,
  submitLabel,
  alternateHref,
  alternateLabel,
  apiBaseUrl,
  redirectUri,
}: AuthFormProps) {
  const [status, setStatus] = useState("");
  const [loading, setLoading] = useState(false);

  const endpoint = `${apiBaseUrl}/auth/${mode === "login" ? "login" : "register"}`;

  async function handleSubmit(event: React.SubmitEvent<HTMLFormElement>) {
    event.preventDefault();
    setLoading(true);
    setStatus("Authenticating...");

    const formData = new FormData(event.currentTarget);
    const payload = Object.fromEntries(formData.entries());

    try {
      const response = await fetch(endpoint, {
        method: "POST",
        headers: {
          "content-type": "application/json",
        },
        body: JSON.stringify(payload),
      });

      const data = await response.json().catch(() => null);
      if (!response.ok) {
        const message =
          data?.message && Array.isArray(data.message)
            ? data.message.join(", ")
            : data?.message || "Authentication failed";
        throw new Error(message);
      }

      if (!data?.accessToken || !redirectUri) {
        throw new Error("Missing token or redirect target");
      }

      const target = new URL(redirectUri);
      target.searchParams.set("access_token", data.accessToken);
      if (data.refreshToken) {
        target.searchParams.set("refresh_token", data.refreshToken);
      }
      console.log(redirectUri);
      setStatus("Authenticated. Redirecting back to the app...");
      window.location.href = target.toString();
    } catch (error) {
      setStatus(error instanceof Error ? error.message : "Authentication failed");
    } finally {
      setLoading(false);
    }
  }

  return (
    <section className="panel">
      <p className="eyebrow">{mode === "login" ? "Desktop Login" : "Create Account"}</p>
      <h1>{title}</h1>
      <p className="copy">{description}</p>

      <form className="form" onSubmit={handleSubmit}>
        {mode === "signup" && (
          <label>
            <span>Name</span>
            <input name="name" type="text" autoComplete="name" required />
          </label>
        )}

        <label>
          <span>Email</span>
          <input name="email" type="email" autoComplete="email" required />
        </label>

        <label>
          <span>Password</span>
          <input
            name="password"
            type="password"
            autoComplete={mode === "login" ? "current-password" : "new-password"}
            required
          />
        </label>

        <button type="submit" disabled={loading}>
          {loading ? "Working..." : submitLabel}
        </button>
      </form>

      <p className="status" aria-live="polite">
        {status}
      </p>
      <p className="alt">
        <a href={alternateHref}>{alternateLabel}</a>
      </p>
    </section>
  );
}

export default AuthForm;
