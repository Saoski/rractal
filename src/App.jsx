import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Canvas from "./components/Canvas.jsx"

function App() {
  const getFractalPixels = async (width, height) => {
    return await invoke("get_pixels", { width, height })
  }

  const setZoom = (event, bounding) => {
    const x = event.clientX - bounding.left;
    const y = event.clientY - bounding.top;
    console.log(`Clicked at (${x}, ${y})`);
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
