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
        width={1600}
        height={900}
      />
    </main>
  );
}

export default App;
