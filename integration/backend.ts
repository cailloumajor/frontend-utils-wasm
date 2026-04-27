import { serveDir } from "@std/http"
import * as path from "@std/path"

const wwwDir = path.join(import.meta.dirname!, "www")

const bundleResult = await Deno.bundle({
  entrypoints: [path.join(wwwDir, "index.ts")],
  platform: "browser",
  write: false,
})

for (const warn of bundleResult.warnings) {
  console.warn(warn)
}

for (const error of bundleResult.errors) {
  console.error(error)
}

if (bundleResult.errors.length > 0) {
  Deno.exit(1)
}

if (!bundleResult.outputFiles) {
  console.error("Missing bundle output")
  Deno.exit(1)
}

export function handler(req: Request) {
  const reqURL = new URL(req.url)

  if (reqURL.pathname === "/index.js") {
    return new Response(bundleResult.outputFiles![0].contents, {
      headers: {
        "Content-Type": "text/javascript",
      },
    })
  }

  return serveDir(req, {
    fsRoot: wwwDir,
  })
}

if (import.meta.main) {
  Deno.serve(handler)
}
