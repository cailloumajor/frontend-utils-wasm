//@ts-check
const fs = require("node:fs")

/**
 * @param {import("connect").Server} app
 */
module.exports = function (app) {
  app.use("/api/influxdb", (req, res) => {
    if (req.method?.toUpperCase() !== "POST") {
      throw new Error("unexpected method")
    }
    const url = new URL(req.url ?? "", "http://example.com")
    if (url.searchParams.get("org") !== "testorg") {
      throw new Error("unexpected `org` query param")
    }
    if (req.headers.authorization !== "Token testtoken") {
      throw new Error("unexpected `Authorization header`")
    }
    let body = ""
    req.setEncoding("utf8")
    req.on("data", (chunk) => {
      body = body.concat(chunk)
    })
    req.on("end", () => {
      if (body !== "testfluxquery") {
        throw new Error("unexpected body")
      }
    })

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
