import { useEffect } from "react";
import { useRef } from "react";
import { invoke } from "@tauri-apps/api/core";

const Canvas = ({ getFractalPixels, ...props }) => {

  const canvasRef = useRef(null)

  useEffect(() => {
    const draw = async () => {
      const canvas = canvasRef.current
      const context = canvas.getContext('2d')

      const newPixelData = await invoke('get_pixels', { height: canvas.height, width: canvas.width })
      console.log("📢[Canvas.jsx:15]: width: ", canvas.width);
      console.log("📢[Canvas.jsx:17]: height: ", canvas.height);

      const imageData = context.createImageData(canvas.width, canvas.height)

      imageData.data.set(newPixelData);

      context.putImageData(imageData, 0, 0)
    }

    draw();
  }, [])

  return <canvas ref={canvasRef} {...props} />
}

export default Canvas
