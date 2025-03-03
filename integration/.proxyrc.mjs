//@ts-check

import { createReadStream } from "fs"

/**
 * @param {import("connect").Server} app
 */
export default function (app) {
  app.use("/timeline", (req, res) => {
    const stream = createReadStream("timeline_data.bin")
    stream.on("open", () => {
      res.setHeader("Content-Type", "application/msgpack")
      stream.pipe(res)
    })
    stream.on("error", (err) => {
      res.writeHead(500, err.message).end()
    })
  })
}
