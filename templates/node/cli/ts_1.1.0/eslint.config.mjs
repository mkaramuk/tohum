import { defineConfig } from "eslint/config";
import eslint from "@eslint/js";
import tseslint from "typescript-eslint";

export default defineConfig({
  ignores: ["./**/*.js", "**/dist/**", "./node_modules/**/*"],
  files: ["./**/*.ts"],

  plugins: {
    "@typescript-eslint": tseslint.plugin,
  },

  languageOptions: {
    parser: tseslint.parser,
    parserOptions: {
      projectService: true,
      tsconfigRootDir: import.meta.dirname,
    },
  },

  extends: [
    eslint.configs.recommended,
    tseslint.configs.recommendedTypeChecked,
  ],

  rules: {
    "@typescript-eslint/no-empty-object-type": "off",

    // We all know at some point we'll have to use `any`
    "@typescript-eslint/no-explicit-any": "off",
  },
});
