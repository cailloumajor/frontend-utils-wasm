import { defineConfig } from "cypress"
import { initPlugin } from "@frsource/cypress-plugin-visual-regression-diff/plugins"

export default defineConfig({
  env: {
    pluginVisualRegressionCleanupUnusedImages: true,
  },
  e2e: {
    baseUrl: "http://localhost:8000",
    setupNodeEvents(on, config) {
      initPlugin(on, config)
    },
  },
})
