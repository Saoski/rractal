import { useEffect } from "react";
import { useRef } from "react";
import { invoke } from "@tauri-apps/api/core";

const Canvas = ({ getFractalPixels, setZoom, width, height, ...props }) => {
  const canvasRef = useRef(null)
  const devicePixelRatio = window.devicePixelRatio || 1;

  const draw = async () => {
    const canvas = canvasRef.current
    const context = canvas.getContext('2d')

    const newPixelData = await getFractalPixels(canvas.width, canvas.height)

    const imageData = context.createImageData(canvas.width, canvas.height)

    imageData.data.set(newPixelData);

    context.putImageData(imageData, 0, 0)
  }

  useEffect(() => {draw()}, [])

  return (<canvas
    ref={canvasRef}
    onClick={(event) => { 
      setZoom(event, canvasRef.current.getBoundingClientRect(), width, height)
      draw()
    }}
    width={width * devicePixelRatio}
    height={height * devicePixelRatio}
    style={{
      width: `${width}px`,
      height: `${height}px`,
      border: '1px solid #ccc',
      imageRendering: 'pixelated',
    }}
    {...props}
  />)
}

export default Canvas
