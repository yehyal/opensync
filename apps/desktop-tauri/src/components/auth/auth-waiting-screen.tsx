import { AppBrand } from "./app-brand";
import { AuthShell } from "./auth-shell";

type AuthWaitingScreenProps = {
  isRedirecting: boolean;
};

export function AuthWaitingScreen({ isRedirecting }: AuthWaitingScreenProps) {
  return (
    <AuthShell>
      <div className="space-y-6 text-center">
        <AppBrand />

        <div className="space-y-4">
          <div className="mx-auto h-12 w-12 animate-spin rounded-full border-4 border-white/15 border-t-emerald-300" />
          <div className="space-y-2">
            <h2 className="text-xl font-semibold text-white">
              {isRedirecting ? "Waiting for login" : "Checking session"}
            </h2>
            <p className="text-sm leading-6 text-stone-300">
              {isRedirecting
                ? "Finish the browser flow to continue. This screen will update automatically once authentication completes."
                : "Looking for an existing authenticated session for this device."}
            </p>
          </div>
        </div>
      </div>
    </AuthShell>
  );
}
