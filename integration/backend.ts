import { serveDir } from "@std/http"
import * as path from "@std/path"
import * as esbuild from "esbuild"

const wwwDir = path.join(import.meta.dirname!, "www")

const jsEntryPoint = await esbuild.build({
  bundle: false,
  entryPoints: [path.join(wwwDir, "index.ts")],
  write: false,
})

for (const warn of jsEntryPoint.warnings) {
  console.warn(warn)
}

for (const error of jsEntryPoint.errors) {
  console.error(error)
}

if (jsEntryPoint.errors.length > 0) {
  Deno.exit(1)
}

export function handler(req: Request) {
  const reqURL = new URL(req.url)

  if (reqURL.pathname === "/index.js") {
    return new Response(jsEntryPoint.outputFiles[0].contents, {
      headers: {
        "Content-Type": "text/javascript",
      },
    })
  }

  if (reqURL.pathname.startsWith("/pkg/")) {
    return serveDir(req, {
      fsRoot: "pkg",
      urlRoot: "pkg",
      showIndex: false,
    })
  }

  return serveDir(req, {
    fsRoot: wwwDir,
  })
}
