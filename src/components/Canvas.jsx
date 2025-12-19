import { useEffect } from "react";
import { useRef } from "react";
import { invoke } from "@tauri-apps/api/core";

const Canvas = ({ getFractalPixels, setZoom, ...props }) => {
  const canvasRef = useRef(null)

  useEffect(() => {
    const draw = async () => {
      const canvas = canvasRef.current
      const context = canvas.getContext('2d')

      const newPixelData = await getFractalPixels(canvas.width, canvas.height)

      const imageData = context.createImageData(canvas.width, canvas.height)

      imageData.data.set(newPixelData);

      context.putImageData(imageData, 0, 0)
    }

    draw();
  }, [])

  return <canvas ref={canvasRef} onClick={(event) => {setZoom(event, canvasRef.current.getBoundingClientRect())}} {...props} />
}

export default Canvas
