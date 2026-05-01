// @ts-self-types="./{{ types_file }}"

import * as imports from "./{{ glue_file }}"
import { __wbg_set_wasm } from "./{{ glue_file }}"

async function decodeWasmPayload() {
  // biome-ignore format: keep compact formatting
  const b64Decoded = Uint8Array.fromBase64("\
{% for line in base64_lines -%}
{{ line }}\
{% endfor -%}
  ")
  const compressed = new Response(b64Decoded)
  const compressedBlob = await compressed.blob()
  const decompress = new DecompressionStream("gzip")
  const pipeline = compressedBlob.stream().pipeThrough(decompress)
  const wasm = new Response(pipeline)
  const wasmBytes = await wasm.arrayBuffer()

  return wasmBytes
}

const wasmModule = new WebAssembly.Module(await decodeWasmPayload())
const wasmInstance = new WebAssembly.Instance(wasmModule, {
  "./{{ glue_file }}": imports
})

__wbg_set_wasm(wasmInstance.exports)
wasmInstance.exports.__wbindgen_start()

export * from "./{{ glue_file }}"
