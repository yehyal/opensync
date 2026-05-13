import { AuthShell } from "./auth-shell";

export function AuthenticatedScreen() {
  return (
    <AuthShell>
      <div className="space-y-4 text-center">
        <div className="mx-auto flex h-16 w-16 items-center justify-center rounded-full bg-emerald-400/15 text-2xl text-emerald-300">
          ✓
        </div>
        <div className="space-y-2">
          <h1 className="text-2xl font-semibold text-white">Logged in</h1>
          <p className="text-sm leading-6 text-stone-300">Your desktop app is authenticated.</p>
        </div>
      </div>
    </AuthShell>
  );
}
