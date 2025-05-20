import { launch, type Page } from "@astral/astral"

export async function withBackendAndBrowser(
  handler: Deno.ServeHandler,
  fn: (page: Page, addr: string) => void | Promise<void>,
) {
  const aborter = new AbortController()

  await using server = Deno.serve({
    hostname: "127.0.0.1",
    port: 0,
    signal: aborter.signal,
    onListen: () => {}, // Don't spam terminal with "Listening on..."
  }, handler)

  await using browser = await launch({
    args: [
      "--no-sandbox",
    ],
    headless: !Deno.args.includes("--no-headless"),
  })

  await using page = await browser.newPage()

  await fn(page, `http://${server.addr.hostname}:${server.addr.port}`)
}

export async function interceptResponse(page: Page, urlPattern: string, body: string) {
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
