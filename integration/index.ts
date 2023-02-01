import "@fontsource/roboto"

import init, { Timeline } from "../pkg"

const canvas = document.getElementById("target-canvas") as HTMLCanvasElement
const drawButton = document.getElementById("draw-button") as HTMLElement
const errorOut = document.getElementById("error-out") as HTMLElement

init().then(() => {
  const timeline = new Timeline(canvas, {
    fontFamily: "Roboto",
    opacity: 0.7,
    xIntervalMinutes: 120,
    xOffsetMinutes: 1,
    influxdbUrl: "/api/influxdb",
    influxdbOrg: "testorg",
    influxdbToken: "testtoken",
    fluxQuery: "testfluxquery",
  })
  const drawedClass = "drawed"
  canvas.addEventListener("drawed", () => {
    canvas.classList.add(drawedClass)
  })
  drawButton.addEventListener("click", () => {
    canvas.classList.remove(drawedClass)
    console.time("draw function")
    timeline
      .draw()
      .then(() => {
        console.timeEnd("draw function")
      })
      .catch((err) => {
        errorOut.textContent = String(err)
      })
  })
  drawButton.classList.add("ready")
})
