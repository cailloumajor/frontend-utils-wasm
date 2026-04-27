// Quick polyfill for `Uint8Array.fromBase64`
if (typeof Uint8Array.fromBase64 !== "function") {
  Uint8Array.fromBase64 = function (encoded) {
    const binary = atob(encoded)
    const size = binary.length
    const bytes = new Uint8Array(size)
    for (let i = 0; i < size; i++) {
      bytes[i] = binary.charCodeAt(i)
    }
    return bytes
  }
}
