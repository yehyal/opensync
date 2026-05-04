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

  async function handleSubmit(event: React.FormEvent<HTMLFormElement>) {
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
    <section className="w-full max-w-xl p-8 sm:p-10 bg-[#fffdf8]/95 border border-[#d9d0c0] shadow-[0_24px_64px_rgba(22,18,9,0.08)]">
      <p className="m-0 mb-3 text-[0.72rem] tracking-[0.18em] uppercase text-[#6c665d]">
        {mode === "login" ? "Desktop Login" : "Create Account"}
      </p>
      <h1 className="m-0 mb-3 text-[clamp(2rem,6vw,3.2rem)] leading-[0.95] font-semibold">
        {title}
      </h1>
      <p className="m-0 mb-6 text-base leading-relaxed text-[#6c665d]">{description}</p>

      <form className="grid gap-4" onSubmit={handleSubmit}>
        {mode === "signup" && (
          <label className="grid gap-1.5">
            <span className="text-sm text-[#6c665d]">Name</span>
            <input
              name="name"
              type="text"
              autoComplete="name"
              required
              className="w-full border border-[#d9d0c0] bg-white px-4 py-3 text-[#1c1a17]"
            />
          </label>
        )}

        <label className="grid gap-1.5">
          <span className="text-sm text-[#6c665d]">Email</span>
          <input
            name="email"
            type="email"
            autoComplete="email"
            required
            className="w-full border border-[#d9d0c0] bg-white px-4 py-3 text-[#1c1a17]"
          />
        </label>

        <label className="grid gap-1.5">
          <span className="text-sm text-[#6c665d]">Password</span>
          <input
            name="password"
            type="password"
            autoComplete={mode === "login" ? "current-password" : "new-password"}
            required
            className="w-full border border-[#d9d0c0] bg-white px-4 py-3 text-[#1c1a17]"
          />
        </label>

        <button
          type="submit"
          disabled={loading}
          className="inline-flex items-center justify-center bg-[#1f5eff] text-white px-5 py-3 disabled:opacity-60"
        >
          {loading ? "Working..." : submitLabel}
        </button>
      </form>

      <p className="min-h-6 mt-4 text-[0.95rem] text-[#6c665d]" aria-live="polite">
        {status}
      </p>
      <p className="mt-4 text-[0.95rem]">
        <a
          className="text-[#6c665d] underline underline-offset-4 decoration-[#6c665d]/40"
          href={alternateHref}
        >
          {alternateLabel}
        </a>
      </p>
    </section>
  );
}

export default AuthForm;
