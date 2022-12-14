import init, { Timeline } from "../pkg"

const canvas = document.getElementById("target-canvas") as HTMLCanvasElement
const drawButton = document.getElementById("draw-button") as HTMLElement
const errorOut = document.getElementById("error-out") as HTMLElement

async function draw() {
  console.time("draw function")
  await Timeline.draw(canvas, {
    influxdbUrl: "/api/influxdb",
    influxdbOrg: "testorg",
    influxdbToken: "testtoken",
    fluxQuery: "testfluxquery",
  })
  console.timeEnd("draw function")
}

init().then(() => {
  const drawedClass = "drawed"
  canvas.addEventListener("drawed", () => {
    canvas.classList.add(drawedClass)
  })
  drawButton.addEventListener("click", () => {
    canvas.classList.remove(drawedClass)
    draw().catch((err) => {
      errorOut.textContent = String(err)
    })
  })
  drawButton.classList.add("ready")
})
