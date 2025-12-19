import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Canvas from "./components/Canvas.jsx"

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }

  const getFractalPixels = async (width, height) => {
    return await invoke("get_pixels", { width, height })
  }

  return (
    <main className="container">
      <Canvas 
        getFractalPixels={getFractalPixels}
      />

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Bruh a name..."
        />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>
    </main>
  );
}

export default App;
