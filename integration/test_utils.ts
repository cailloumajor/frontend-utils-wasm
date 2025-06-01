import { launch, type Page } from "@astral/astral"
import * as path from "@std/path"

const size = { width: 920, height: 400 }

const fps = 30

const ffmpegArgs = [
  ["-loglevel", "warning"],
  // Read images from standard input.
  ["-f", "image2pipe", "-r", `${fps}`, "-i", "pipe:0"],
  // No audio
  ["-an"],
  // Use libvpx-vp9 video encoder.
  ["-c:v", "libvpx-vp9", "-crf", "32", "-b:v", "0", "-deadline", "realtime"],
  // Sets the output format.
  ["-f", "webm"],
  // Slow down the video.
  ["-vf", "setpts=10.0*PTS"],
]

async function withRecording(
  videoPath: string,
  page: Page,
  fn: () => void | Promise<void>,
): Promise<void> {
  const frameInterval = 1000 / fps

  const ffmpegCommand = new Deno.Command("ffmpeg", {
    // Use `-y` to overwrite output file.
    args: [...ffmpegArgs.flat(), "-y", videoPath],
    stdin: "piped",
  })
  await using ffmpegProcess = ffmpegCommand.spawn()
  const toFFmpeg = ffmpegProcess.stdin.getWriter()

  let errorDuringInterval: Error | null = null

  const timer = setInterval(
    async () => {
      try {
        const png = await page.screenshot({ optimizeForSpeed: true })
        await toFFmpeg.ready
        await toFFmpeg.write(png)
      } catch (err) {
        clearInterval(timer)
        errorDuringInterval = err as Error
      }
    },
    frameInterval,
  )

  await fn()

  // Wait some frames to allow recording the final status.
  await new Promise<void>((resolve) =>
    setTimeout(
      () => {
        clearInterval(timer)
        resolve()
      },
      8 * frameInterval,
    )
  )

  await toFFmpeg.ready
  await toFFmpeg.close()

  if (errorDuringInterval != null) {
    throw errorDuringInterval
  }

  const status = await ffmpegProcess.status
  if (!status.success) {
    throw new Error("FFmpeg process encountered an error")
  }
}

export function testId(t: Deno.TestContext): string {
  return t.name.replaceAll(" ", "_")
}

export async function withBackendAndBrowser(
  t: Deno.TestContext,
  handler: Deno.ServeHandler,
  fn: (page: Page, addr: string) => void | Promise<void>,
): Promise<void> {
  const noHeadless = Deno.args.includes("--no-headless")

  await using server = Deno.serve({
    hostname: "127.0.0.1",
    port: 0,
    onListen: () => {}, // Don't spam terminal with "Listening on..."
  }, handler)

  await using browser = await launch({
    args: [
      "--no-sandbox",
    ],
    headless: !noHeadless,
  })

  await using page = await browser.newPage()

  await page.setViewportSize(size)

  const addr = `http://${server.addr.hostname}:${server.addr.port}`

  if (noHeadless) {
    await fn(page, addr)
  } else {
    const videosDir = path.join(import.meta.dirname!, "videos")
    const videoFile = path.join(videosDir, `${testId(t)}.webm`)
    await withRecording(videoFile, page, () => fn(page, addr))
  }
}

export async function interceptResponse(
  page: Page,
  urlPattern: string,
  body: string,
): Promise<void> {
  const celestial = page.unsafelyGetCelestialBindings()
  await celestial.Fetch.enable({
    patterns: [
      {
        urlPattern,
        requestStage: "Response",
      },
    ],
  })
  celestial.addEventListener("Fetch.requestPaused", async (event) => {
    const { requestId, responseStatusCode, responseHeaders, responseStatusText } = event.detail
    if (responseStatusCode == null || responseHeaders == null || responseStatusText == null) {
      throw new Error("bad Fetch.requestPaused event")
    }
    await celestial.Fetch.fulfillRequest({
      requestId,
      responseCode: responseStatusCode,
      responseHeaders,
      responsePhrase: responseStatusText,
      body,
    })
  })
}
