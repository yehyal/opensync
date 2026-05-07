import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const checkAuthState = async () => {
    const isAuth = await invoke<boolean>("is_authenticated");

    setIsAuthenticated(isAuth);
  }

  useEffect(() => {
    checkAuthState();
  }, [])

  if (!isAuthenticated) {
    return (
      <main className="container">
        <h1>Login to continue</h1>

        <div className="row">
          <button>Login</button>
        </div>
      </main>
    )
  }

  return (
    <main className="container">
      <h1>Welcome to Tauri + React</h1>

      <div className="row">
        <a href="https://vite.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
        }}
      >
        <input
          id="greet-input"
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>
    </main>
  );
}

export default App;
