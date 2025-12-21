import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Canvas from "./components/Canvas.jsx"
import { useState } from "react";
import { useEffect } from "react";

function App() {
  const CANVAS_WIDTH = 1600
  const CANVAS_HEIGHT = 900

  const [fractalPixels, setFractalPixels] = useState([])

  useEffect(() => {
    const fetchPixels = async () => {
      setFractalPixels(await getFractalPixels());
    }
    fetchPixels();
  }, [])

  const getFractalPixels = async () => {
    return await invoke("get_pixels", { width: CANVAS_WIDTH, height: CANVAS_HEIGHT })
  }

  const setZoom = async (event, bounding, width, height) => {
    const devicePixelRatio = window.devicePixelRatio || 1;
    const x = Math.round((event.clientX - bounding.left) * devicePixelRatio);
    const y = Math.round((event.clientY - bounding.top) * devicePixelRatio);
    console.log(`Clicked at (${x}, ${y})`);
    await invoke("zoom" ,{width, height, x, y, zoomMult: 2})
    setFractalPixels(await getFractalPixels())
  }

  const resetZoom = async () => {
    await invoke("reset_zoom", {})
    setFractalPixels(await getFractalPixels())
  }

  return (
    <main className="flex justify-center items-center h-screen">
      <div className="flex">
        <div className="flex-col justify-start">
          <button onClick={() => {resetZoom()}}>Reset</button>
        </div>
        <Canvas
          fractalPixels={fractalPixels}
          setZoom={setZoom}
          width={CANVAS_WIDTH}
          height={CANVAS_HEIGHT}
        />
      </div>
    </main>
  );
}

export default App;
