//@ts-check
const fs = require("fs")

/**
 * @param {import("connect").Server} app
 */
module.exports = function (app) {
  app.use("/api/influxdb", (req, res) => {
    const stream = fs.createReadStream("influxdb_data.csv")
    stream.on("open", () => {
      res.setHeader("Content-Type", "application/csv")
      stream.pipe(res)
    })
    stream.on("error", (err) => {
      res.writeHead(500, err.message).end()
    })
  })
}
