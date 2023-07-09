//@ts-check
const fs = require("fs")

/**
 * @param {import("connect").Server} app
 */
module.exports = function (app) {
  app.use("/timeline", (req, res) => {
    const stream = fs.createReadStream("timeline_data.bin")
    stream.on("open", () => {
      res.setHeader("Content-Type", "application/msgpack")
      stream.pipe(res)
    })
    stream.on("error", (err) => {
      res.writeHead(500, err.message).end()
    })
  })
}
