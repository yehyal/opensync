export function AppBrand() {
  return (
    <div className="flex flex-col items-center gap-4 text-center">
      <img
        src="/sync.png"
        alt="OpenSync logo"
        className="h-20 w-20 rounded-2xl border border-white/10 bg-white/8 p-3 shadow-[0_18px_40px_rgba(0,0,0,0.35)]"
      />
      <div className="space-y-2">
        <p className="text-xs font-semibold uppercase tracking-[0.3em] text-emerald-300/80">
          Desktop Client
        </p>
        <div>
          <h1 className="text-3xl font-semibold tracking-tight text-white">
            OpenSync
          </h1>
          <p className="mt-2 text-sm leading-6 text-stone-300">
            Sign in from your browser to connect the desktop app to your
            workspace.
          </p>
        </div>
      </div>
    </div>
  );
}
