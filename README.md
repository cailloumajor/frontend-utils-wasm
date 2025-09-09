# WASM utilities for using in frontend

[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org)

## Exposed components

### Timeline

Draws a timeline on an existing `<canvas>` element, given slots data in [MessagePack][msgpack] format. Slots data is expected to be an array of arrays, inner arrays having the following components:

1. Slot start date and time in seconds since epoch (integer);
2. Index of the color in an abstract palette (integer).

[msgpack]: https://msgpack.org/

#### Example

```Javascript
const canvas = document.getElementById("target-canvas");

const timeline = new Timeline(canvas, {
  palette: ["#00ff00", "#ffff00", "#ff0000", "#ff00ff"],
  fontFamily: "Roboto",
  opacity: 0.7,
  xIntervalMinutes: 60,
  xOffsetMinutes: 53,
  emphasisLabels: ["07:53", "15:53", "23:53"],
});

```
