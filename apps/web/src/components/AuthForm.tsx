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
  const [status, setStatus] = useState<{
    tone: "idle" | "info" | "success" | "error";
    message: string;
  }>({
    tone: "idle",
    message:
      mode === "login"
        ? "Sign in here, then OpenSync will hand the session back to your desktop app."
        : "Create a lightweight account now. More sync features can grow from the same identity later.",
  });
  const [loading, setLoading] = useState(false);

  const endpoint = `${apiBaseUrl}/auth/${mode === "login" ? "login" : "register"}`;

  async function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setLoading(true);
    setStatus({ tone: "info", message: "Contacting OpenSync and preparing your handoff..." });

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
      target.searchParams.set("user_id", data.user.id);

      setStatus({
        tone: "success",
        message: "Authenticated. Returning to the desktop app now.",
      });

      window.location.href = target.toString();
    } catch (error) {
      setStatus({
        tone: "error",
        message: error instanceof Error ? error.message : "Authentication failed",
      });
    } finally {
      setLoading(false);
    }
  }

  return (
    <section className="auth-card">
      <header className="auth-card__header">
        <p className="section-kicker">{mode === "login" ? "Desktop login" : "Create account"}</p>
        <h1 className="auth-card__title">{title}</h1>
        <p className="auth-card__description">{description}</p>
      </header>

      <form className="auth-form" onSubmit={handleSubmit} aria-busy={loading}>
        <div className="auth-fieldset">
          {mode === "signup" ? (
            <div className="auth-field">
              <label htmlFor="auth-name">Name</label>
              <input
                id="auth-name"
                name="name"
                type="text"
                autoComplete="name"
                placeholder="How should OpenSync identify you?"
                required
                className="auth-input"
              />
            </div>
          ) : null}

          <div className="auth-field">
            <label htmlFor="auth-email">Email</label>
            <input
              id="auth-email"
              name="email"
              type="email"
              autoComplete="email"
              placeholder="you@workspace.dev"
              required
              className="auth-input"
            />
          </div>

          <div className="auth-field">
            <label htmlFor="auth-password">Password</label>
            <input
              id="auth-password"
              name="password"
              type="password"
              autoComplete={mode === "login" ? "current-password" : "new-password"}
              placeholder={mode === "login" ? "Enter your password" : "Choose a secure password"}
              required
              className="auth-input"
            />
          </div>
        </div>

        <button type="submit" disabled={loading} className="button button--primary auth-submit">
          {loading ? "Working..." : submitLabel}
        </button>
      </form>

      <p className="auth-status" data-tone={status.tone} aria-live="polite">
        {status.message}
      </p>

      <footer className="auth-card__footer">
        <p>{mode === "login" ? "Need an account first?" : "Already set up?"}</p>
        <a className="auth-link" href={alternateHref}>
          {alternateLabel}
        </a>
      </footer>
    </section>
  );
}

export default AuthForm;
