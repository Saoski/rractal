import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Canvas from "./components/Canvas.jsx"
import { useState } from "react";
import { useEffect } from "react";

function App() {
  const CANVAS_WIDTH = 1600
  const CANVAS_HEIGHT = 900

  const [fractalPixels, setFractalPixels] = useState([]);
  const [algoOptions, setAlgoOptions] = useState([]);
  const [selectedAlgo, setSelectedAlgo] = useState("");

  const getFractalPixels = async () => {
    return await invoke("get_pixels", { width: CANVAS_WIDTH, height: CANVAS_HEIGHT });
  }

  const getAlgoOptions = async () => {
    return await invoke("get_algos");
  }

  useEffect(() => {
    const fetchPixels = async () => {
      setFractalPixels(await getFractalPixels());
    }
    const fetchAlgos = async () => {
      setAlgoOptions(await getAlgoOptions())
    }
    fetchPixels();
    fetchAlgos();
    setSelectedAlgo(algoOptions[0]);
  }, [])

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

  const handleSelectAlgo = async (chosen_algo) => {
    await invoke("choose_algo", {chosen_algo})
  }

  return (
    <main className="flex justify-center items-center h-screen">
      <div className="flex items-start">
        <div className="flex flex-col h-full gap-3">
          <button onClick={() => {resetZoom()}}>Reset</button>
          <div className="flex gap-1">
            <label htmlFor="algorithms">Select an algorithm:</label>
            <select name="algorithms" id="algorithms">
              {algoOptions.map((algorithm) => (
                <option value="algorithm" onClick={}>{algorithm}</option>
              ))}
            </select>
          </div>
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
