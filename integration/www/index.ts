import { Timeline } from "../../pkg/frontend_utils_wasm.js"

const canvas = document.getElementById("target-canvas")!
const setDataButton = document.getElementById("set-data-button")!
const drawButton = document.getElementById("draw-button")!
const errorOut = document.getElementById("error-out")!

const timeline = new Timeline(canvas as HTMLCanvasElement, {
  palette: ["#00ff00", "#ffff00", "#ff0000", "#ff00ff"],
  fontFamily: "Roboto",
  opacity: 0.7,
  xIntervalMinutes: 60,
  xOffsetMinutes: 53,
  emphasisLabels: ["07:53", "15:53", "23:53"],
})

const drawReadyClass = "ready"

setDataButton.addEventListener("click", () => {
  drawButton.classList.remove(drawReadyClass)
  fetch("/timeline_data.bin")
    .then((response) => response.arrayBuffer())
    .then((buffer) => {
      const data = new Uint8Array(buffer)
      console.time("setData function")
      timeline.setData(data)
      console.timeEnd("setData function")
      drawButton.classList.add(drawReadyClass)
    })
    .catch((err) => {
      errorOut.textContent = String(err)
    })
})

const drawedClass = "drawed"

drawButton.addEventListener("click", () => {
  canvas.classList.remove(drawedClass)
  try {
    console.time("draw function")
    timeline.draw()
    console.timeEnd("draw function")
  } catch (err) {
    errorOut.textContent = String(err)
  }
  canvas.classList.add(drawedClass)
})
