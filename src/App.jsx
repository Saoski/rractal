import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Canvas from "./components/Canvas.jsx"

function App() {
  const getFractalPixels = async (width, height) => {
    return await invoke("get_pixels", { width, height })
  }

  const setZoom = async (event, bounding, width, height) => {
    const devicePixelRatio = window.devicePixelRatio || 1;
    const x = Math.round((event.clientX - bounding.left) * devicePixelRatio);
    const y = Math.round((event.clientY - bounding.top) * devicePixelRatio);
    console.log(`Clicked at (${x}, ${y})`);
    await invoke("zoom" ,{width, height, x, y, zoomMult: 2})
  }

  return (
    <main className="container">
      <Canvas
        getFractalPixels={getFractalPixels}
        setZoom={setZoom}
        width={1600}
        height={900}
      />
    </main>
  );
}

export default App;
