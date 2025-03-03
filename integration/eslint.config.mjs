// @ts-check

import globals from "globals"
import pluginCypress from "eslint-plugin-cypress/flat"
import pluginJs from "@eslint/js"
import pluginPrettier from "eslint-plugin-prettier/recommended"
import pluginTypescript from "typescript-eslint"

export default pluginTypescript.config(
  {
    ignores: ["dist/*"],
  },
  pluginJs.configs.recommended,
  ...pluginTypescript.configs.recommended,
  pluginCypress.configs.recommended,
  pluginPrettier,
  {
    languageOptions: {
      ecmaVersion: "latest",
      sourceType: "module",
      globals: {
        ...globals.browser,
        ...globals.es2021,
        ...globals.node,
      },
    },

    rules: {
      indent: ["error", 2],
      "linebreak-style": ["error", "unix"],
      quotes: ["error", "double", { avoidEscape: true }],
      semi: ["error", "never"],
    },
  },
)
