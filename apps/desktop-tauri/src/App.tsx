import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { openUrl } from "@tauri-apps/plugin-opener";
import { AuthenticatedScreen } from "./components/auth/authenticated-screen";
import { AuthLandingScreen } from "./components/auth/auth-landing-screen";
import { AuthWaitingScreen } from "./components/auth/auth-waiting-screen";

type AuthState = "checking" | "guest" | "redirecting" | "authenticated";

function App() {
  const [authState, setAuthState] = useState<AuthState>("checking");

  const checkAuthState = async () => {
    try {
      const isAuthenticated = await invoke<boolean>("is_authenticated");

      setAuthState(isAuthenticated ? "authenticated" : "guest");
    } catch (error) {
      console.error("Failed to check authentication state", error);
      setAuthState("guest");
    }
  };

  const handleLogin = async () => {
    setAuthState("redirecting");
    await openUrl("http://localhost:4321/login");
  };

  const handleRegister = async () => {
    setAuthState("redirecting");
    await openUrl("http://localhost:4321/register");
  };

  useEffect(() => {
    void checkAuthState();
  }, []);

  useEffect(() => {
    if (authState !== "redirecting") {
      return;
    }

    const intervalId = window.setInterval(async () => {
      try {
        const isAuthenticated = await invoke<boolean>("is_authenticated");

        if (isAuthenticated) {
          setAuthState("authenticated");
        }
      } catch (error) {
        console.error("Failed to refresh authentication state", error);
      }
    }, 2000);

    return () => {
      window.clearInterval(intervalId);
    };
  }, [authState]);

  if (authState === "checking" || authState === "redirecting") {
    return <AuthWaitingScreen isRedirecting={authState === "redirecting"} />;
  }

  if (authState === "guest") {
    return (
      <AuthLandingScreen
        onLogin={handleLogin}
        onRegister={handleRegister}
      />
    );
  }

  return <AuthenticatedScreen />;
}

export default App;
