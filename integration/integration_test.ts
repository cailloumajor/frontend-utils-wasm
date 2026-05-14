import { assert, assertEquals, assertStringIncludes } from "@std/assert"
import * as path from "@std/path"
import { Image } from "imagescript"
import pixelmatch from "pixelmatch"

import { handler } from "./backend.ts"
import { startBrowser, stopBrowser, testId, withBackendAndBrowser } from "./test_utils.ts"

const updateSnapshot = Deno.args.includes("--update-snapshot")
const ignoreNonSnapshot = updateSnapshot ? { ignore: true } : {}

Deno.test.beforeAll(startBrowser)

Deno.test.afterAll(stopBrowser)

Deno.test({
  name: "timeline fails with an error (without throwing) if canvas was deleted",
  sanitizeOps: false,
  sanitizeResources: false,
  ...ignoreNonSnapshot,
  async fn(t) {
    await withBackendAndBrowser(t, handler, async (page, addr) => {
      await page.goto(addr)

      await page.waitForNetworkIdle({ idleTime: 100 })

      await page.locator("#set-data-button").click()

      await page.$eval("#target-canvas", (el) => {
        el.remove()
      })

      await page.locator("#draw-button.ready").click()

      const errorText = await page
        .locator("div#error-out:not(:empty)")
        .map((el) => el.innerText)
        .wait()
      assertStringIncludes(errorText, "error parsing canvas style property `color`")
    })
  },
})

Deno.test({
  name: "timeline fails if MessagePack deserialization errors",
  sanitizeOps: false,
  sanitizeResources: false,
  ...ignoreNonSnapshot,
  async fn(t) {
    await withBackendAndBrowser(t, handler, async (page, addr) => {
      await page.goto(addr)

      await page.waitForNetworkIdle({ idleTime: 100 })

      const body = new Uint8Array([0xc1])

      await page.setRequestInterception(true)
      page.on("request", (interceptedRequest) => {
        if (interceptedRequest.isInterceptResolutionHandled()) {
          return
        }

        if (interceptedRequest.url().endsWith("/timeline_data.bin")) {
          interceptedRequest.respond({ body })
        } else {
          interceptedRequest.continue()
        }
      })

      await page.locator("#set-data-button").click()

      const errorText = await page
        .locator("div#error-out:not(:empty)")
        .map((el) => el.innerText)
        .wait()
      assertStringIncludes(errorText, "MessagePack")
      assertStringIncludes(errorText, "Reserved")
    })
  },
})

Deno.test({
  name: "timeline fails if draw is requested before having slots data",
  sanitizeOps: false,
  sanitizeResources: false,
  ...ignoreNonSnapshot,
  async fn(t) {
    await withBackendAndBrowser(t, handler, async (page, addr) => {
      await page.goto(addr)

      await page.waitForNetworkIdle({ idleTime: 100 })

      await page.locator("#draw-button").click()

      const errorText = await page
        .locator("div#error-out:not(:empty)")
        .map((el) => el.innerText)
        .wait()
      assertStringIncludes(errorText, "empty")
    })
  },
})

Deno.test({
  name: "timeline fails if fetched data has no slot",
  sanitizeOps: false,
  sanitizeResources: false,
  ...ignoreNonSnapshot,
  async fn(t) {
    await withBackendAndBrowser(t, handler, async (page, addr) => {
      await page.goto(addr)

      await page.waitForNetworkIdle({ idleTime: 100 })

      const body = new Uint8Array([0x90])

      await page.setRequestInterception(true)
      page.on("request", (interceptedRequest) => {
        if (interceptedRequest.isInterceptResolutionHandled()) {
          return
        }

        if (interceptedRequest.url().endsWith("/timeline_data.bin")) {
          interceptedRequest.respond({ body })
        } else {
          interceptedRequest.continue()
        }
      })

      await page.locator("#set-data-button").click()

      await page.locator("#draw-button.ready").click()

      const errorText = await page
        .locator("div#error-out:not(:empty)")
        .map((el) => el.innerText)
        .wait()
      assertStringIncludes(errorText, "empty")
    })
  },
})

Deno.test({
  name: "timeline fails if color index is not in the palette",
  sanitizeOps: false,
  sanitizeResources: false,
  ...ignoreNonSnapshot,
  async fn(t) {
    await withBackendAndBrowser(t, handler, async (page, addr) => {
      await page.goto(addr)

      await page.waitForNetworkIdle({ idleTime: 100 })

      // deno-fmt-ignore
      const body = new Uint8Array([
        0x92, 0x92, 0xce, 0x64, 0xa7, 0x37, 0xbc, 0x0f, 0x92, 0xce, 0x64, 0xa7,
        0xe0, 0x9c, 0x01,
      ])

      await page.setRequestInterception(true)
      page.on("request", (interceptedRequest) => {
        if (interceptedRequest.isInterceptResolutionHandled()) {
          return
        }

        if (interceptedRequest.url().endsWith("/timeline_data.bin")) {
          interceptedRequest.respond({ body })
        } else {
          interceptedRequest.continue()
        }
      })

      await page.locator("#set-data-button").click()

      await page.locator("#draw-button.ready").click()

      const errorText = await page
        .locator("div#error-out:not(:empty)")
        .map((el) => el.innerText)
        .wait()
      assertStringIncludes(errorText, "index")
      assertStringIncludes(errorText, "15")
    })
  },
})

Deno.test({
  name: "timeline renders according to snapshot",
  sanitizeOps: false,
  sanitizeResources: false,
  async fn(t) {
    await withBackendAndBrowser(t, handler, async (page, addr) => {
      const snapshotDir = path.join(import.meta.dirname!, "__image_snapshots__")
      const snapshotFile = path.join(snapshotDir, `${testId(t)}.png`)

      await page.goto(addr)

      await page.waitForNetworkIdle({ idleTime: 100 })

      await page.locator("#target-canvas:not(.drawn)").wait()

      await page.locator("#set-data-button").click()

      await page.locator("#draw-button.ready").click()

      const canvasDataURL = await page
        .locator("canvas#target-canvas.drawn")
        .map((el) => el.toDataURL())
        .wait()
      assert(canvasDataURL.startsWith("data:image/png;base64,"), "bad canvas data URL format")
      const canvasPng = Uint8Array.fromBase64(canvasDataURL.substring(22))

      if (Deno.args.includes("--update-snapshot")) {
        await Deno.writeFile(snapshotFile, canvasPng)
        return
      }

      const canvasImage = await Image.decode(canvasPng)

      const expected = await Deno.readFile(snapshotFile)
      const expectedImage = await Image.decode(expected)

      const diffImage = new Image(canvasImage.width, canvasImage.height)

      const numDiffPixels = pixelmatch(
        canvasImage.bitmap,
        expectedImage.bitmap,
        diffImage.bitmap,
        canvasImage.width,
        canvasImage.height,
        {
          threshold: 0.01,
        },
      )

      if (numDiffPixels > 0) {
        const diffFile = await diffImage.encode(0)
        const diffPath = path.join(snapshotDir, `${testId(t)}.diff.png`)
        await Deno.writeFile(diffPath, diffFile)
      }

      assertEquals(numDiffPixels, 0, "rendered canvas is too different than expected")
    })
  },
})
