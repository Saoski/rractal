import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Canvas from "./components/Canvas.jsx"
import { useState } from "react";
import { useEffect } from "react";
import { info } from "@tauri-apps/plugin-log";
import { listen } from "@tauri-apps/api/event";
import { useRef } from "react";
import { debug } from "@tauri-apps/plugin-log";

const toPascalCase = (str) => {
  return str
    .trim()
    .split(' ')
    .map(word => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
    .join('');
};

function App() {
  const CANVAS_WIDTH = 1600
  const CANVAS_HEIGHT = 900

  const [fractalPixels, setFractalPixels] = useState([]);
  const [algoOptions, setAlgoOptions] = useState([]);
  const [selectedAlgo, setSelectedAlgo] = useState("");
  const [fractalProgressPercent, setFractalProgressPercent] = useState(null);

  const latestRenderId = useRef(0);

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

    // Setup progress listener
    let unlisten = null;

    async function setupProgressListener() {
      unlisten = await listen("fractal-progress", (event) => {
        setFractalProgressPercent(event.payload)
      })
    }

    setupProgressListener()

    // Cleanup listener on unmount
    return () => {
      if (unlisten !== null) {
        unlisten.then(f => f())
      }
    }
  }, [])

  const setZoom = async (event, bounding, width, height) => {
    setFractalProgressPercent(0)

    const devicePixelRatio = window.devicePixelRatio || 1;
    const x = Math.round((event.clientX - bounding.left) * devicePixelRatio);
    const y = Math.round((event.clientY - bounding.top) * devicePixelRatio);

    console.log(`Clicked at (${x}, ${y})`);
    await invoke("zoom", { width, height, x, y, zoomMult: 2 })
    setFractalPixels(await getFractalPixels())
  }

  const resetZoom = async () => {
    await invoke("reset_zoom", {})
    setFractalPixels(await getFractalPixels())
  }

  const handleSelectAlgo = async (e) => {
    let chosen_algo = e.target.value
    info(`Selecting ${chosen_algo} as the chosen algorithm`)
    await invoke("choose_algo", { algo: toPascalCase(chosen_algo) })
    setSelectedAlgo(chosen_algo)
  }

  return (
    <main className="flex justify-center items-center h-screen">
      <div className="flex items-start">
        <div className="flex flex-col h-full gap-3">
          <button onClick={() => { resetZoom() }}>Reset</button>
          <div className="flex gap-1">
            <label htmlFor="algorithms">Select an algorithm:</label>
            <select name="algorithms" id="algorithms" onChange={handleSelectAlgo}>
              {algoOptions.map((algorithm) => (
                <option value={algorithm}>{algorithm}</option>
              ))}
            </select>
          </div>
        </div>
        <div className="flex flex-col gap2">
          <progress value={fractalProgressPercent} max={100} className="w-full" />
          <Canvas
            fractalPixels={fractalPixels}
            setZoom={setZoom}
            width={CANVAS_WIDTH}
            height={CANVAS_HEIGHT}
          />
        </div>
      </div>
    </main>
  );
}

export default App;
