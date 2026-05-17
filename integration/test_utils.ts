import * as path from "@std/path"
import puppeteer, { type Browser, type Page } from "puppeteer"

const size = { width: 920, height: 400 }

const isInCI = ["CI", "CONTINUOUS_INTEGRATION"].some((key) => Deno.env.has(key))

let browser: Browser

export async function startBrowser() {
  try {
    browser = await puppeteer.launch({
      args: ["--no-sandbox"],
      headless: isInCI,
    })
  } catch (err) {
    console.error(err)
  }
}

export async function stopBrowser() {
  await browser.close()
}

export function testId(t: Deno.TestContext): string {
  return t.name.replaceAll(" ", "_")
}

export async function withBackendAndBrowser(
  t: Deno.TestContext,
  handler: Deno.ServeHandler,
  fn: (page: Page, addr: string) => void | Promise<void>,
): Promise<void> {
  await using server = Deno.serve({
    hostname: "127.0.0.1",
    port: 0,
    onListen: () => {}, // Don't spam terminal with "Listening on..."
  }, handler)

  const page = await browser.newPage()

  await page.setViewport(size)

  const addr = `http://${server.addr.hostname}:${server.addr.port}`

  if (!isInCI) {
    await fn(page, addr)
  } else {
    const videoFile = path.join(import.meta.dirname!, "videos", testId(t))
    const recorder = await page.screencast({
      path: `${videoFile}.webm`,
    })
    await fn(page, addr)
    await recorder.stop()
  }

  await page.close()
}
