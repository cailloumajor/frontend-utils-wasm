import "@fontsource/roboto"

import init, { Timeline } from "../pkg"

const canvas = document.getElementById("target-canvas") as HTMLCanvasElement
const drawButton = document.getElementById("draw-button") as HTMLElement
const errorOut = document.getElementById("error-out") as HTMLElement

init().then(() => {
  const timeline = new Timeline(canvas, {
    palette: ["#00ff00", "#ffff00", "#ff0000", "#ff00ff"],
    fontFamily: "Roboto",
    opacity: 0.7,
    xIntervalMinutes: 60,
    xOffsetMinutes: 53,
    emphasisLabels: ["07:53", "15:53", "23:53"],
  })
  const drawedClass = "drawed"
  canvas.addEventListener("drawed", () => {
    canvas.classList.add(drawedClass)
  })
  drawButton.addEventListener("click", () => {
    canvas.classList.remove(drawedClass)
    fetch("/timeline")
      .then((response) => response.arrayBuffer())
      .then((buffer) => {
        const data = new Uint8Array(buffer)
        console.time("draw function")
        return timeline.draw(data)
      })
      .then(() => {
        console.timeEnd("draw function")
      })
      .catch((err) => {
        errorOut.textContent = String(err)
      })
  })
  drawButton.classList.add("ready")
})
