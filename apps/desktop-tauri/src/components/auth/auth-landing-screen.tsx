import { AppBrand } from "./app-brand";
import { AuthShell } from "./auth-shell";
import { Button } from "../ui/button";

type AuthLandingScreenProps = {
  onLogin: () => Promise<void>;
  onRegister: () => Promise<void>;
};

export function AuthLandingScreen({
  onLogin,
  onRegister,
}: AuthLandingScreenProps) {
  return (
    <AuthShell>
      <div className="space-y-6">
        <AppBrand />

        <div className="grid gap-3 sm:grid-cols-2">
          <Button size="lg" onClick={() => void onLogin()}>
            Login
          </Button>
          <Button variant="secondary" size="lg" onClick={() => void onRegister()}>
            Register
          </Button>
        </div>
      </div>
    </AuthShell>
  );
}
