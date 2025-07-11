import { assert, assertEquals, assertStringIncludes } from "@std/assert"
import { decodeBase64, encodeBase64 } from "@std/encoding/base64"
import * as path from "@std/path"
import { Image } from "imagescript"
import pixelmatch from "pixelmatch"

import { handler } from "./backend.ts"
import { interceptResponse, testId, withBackendAndBrowser } from "./test_utils.ts"

const updateSnapshot = Deno.args.includes("--update-snapshot")
const ignoreNonSnapshot = updateSnapshot ? { ignore: true } : {}

Deno.test({
  name: "timeline fails with an error (without throwing) if canvas was deleted",
  ...ignoreNonSnapshot,
  fn: async (t) => {
    await withBackendAndBrowser(t, handler, async (page, addr) => {
      await page.goto(addr, { waitUntil: "load" })

      await page
        .locator<HTMLCanvasElement>("#target-canvas")
        .evaluate((el) => {
          el.remove()
        })

      await page.locator("#draw-button").click()

      const errorText = await page
        .locator<HTMLDivElement>("#error-out:not(:empty)")
        .evaluate((el) => el.innerText)
      assertStringIncludes(errorText, "error parsing canvas style property `color`")
    })
  },
})

Deno.test({
  name: "timeline fails if MessagePack deserialization errors",
  ...ignoreNonSnapshot,
  fn: async (t) => {
    await withBackendAndBrowser(t, handler, async (page, addr) => {
      await page.goto(addr, { waitUntil: "load" })

      const body = encodeBase64(new Uint8Array([0xc1]))

      await interceptResponse(page, "*/timeline_data.bin", body)

      await page.locator("#draw-button").click()

      const errorText = await page
        .locator<HTMLDivElement>("#error-out:not(:empty)")
        .evaluate((el) => el.innerText)
      assertStringIncludes(errorText, "MessagePack")
      assertStringIncludes(errorText, "Reserved")
    })
  },
})

Deno.test({
  name: "timeline fails if there is no data",
  ...ignoreNonSnapshot,
  fn: async (t) => {
    await withBackendAndBrowser(t, handler, async (page, addr) => {
      await page.goto(addr, { waitUntil: "load" })

      const body = encodeBase64(new Uint8Array([0x90]))

      await interceptResponse(page, "*/timeline_data.bin", body)

      await page.locator("#draw-button").click()

      const errorText = await page
        .locator<HTMLDivElement>("#error-out:not(:empty)")
        .evaluate((el) => el.innerText)
      assertStringIncludes(errorText, "empty")
    })
  },
})

Deno.test({
  name: "timeline fails if color index is not in the palette",
  ...ignoreNonSnapshot,
  fn: async (t) => {
    await withBackendAndBrowser(t, handler, async (page, addr) => {
      await page.goto(addr, { waitUntil: "load" })

      // deno-fmt-ignore
      const body = encodeBase64(new Uint8Array([
        0x92, 0x92, 0xce, 0x64, 0xa7, 0x37, 0xbc, 0x0f, 0x92, 0xce, 0x64, 0xa7, 0xe0, 0x9c, 0x01
      ]))

      await interceptResponse(page, "*/timeline_data.bin", body)

      await page.locator("#draw-button").click()

      const errorText = await page
        .locator<HTMLDivElement>("#error-out:not(:empty)")
        .evaluate((el) => el.innerText)
      assertStringIncludes(errorText, "index")
      assertStringIncludes(errorText, "15")
    })
  },
})

Deno.test({
  name: "timeline renders according to snapshot",
  fn: async (t) => {
    await withBackendAndBrowser(t, handler, async (page, addr) => {
      const snapshotDir = path.join(import.meta.dirname!, "__image_snapshots__")
      const snapshotFile = path.join(snapshotDir, `${testId(t)}.png`)

      await page.goto(addr, { waitUntil: "load" })

      await page.locator("#target-canvas:not(.drawed)").wait()

      await page.locator("#draw-button").click()

      const canvasDataURL = await page
        .locator<HTMLCanvasElement>("#target-canvas.drawed")
        .evaluate((el) => el.toDataURL())
      assert(canvasDataURL.startsWith("data:image/png;base64,"), "bad canvas data URL format")
      const canvasPng = decodeBase64(canvasDataURL.substring(22))

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
