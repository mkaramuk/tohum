import { defineConfig } from "tsup";

export default defineConfig({
  // Entry point. Change if your main function is located somewhere else.
  // If you change it, the output file name will be changed
  // as well so update `package.json` scripts to use new file.
  entry: ["src/index.ts"],

  // Everything is going to be bundled so it is fine to use CommonJS
  // to have better compatibility with Node.js environment
  format: ["cjs"],

  // Clear out directory before build
  clean: true,

  // Bundle everything into a single file.
  bundle: true,

  // Include all the dependencies in the bundled file
  // Check for more info: https://github.com/egoist/tsup/issues/619
  noExternal: [/(.*)/],

  outDir: "dist",
  target: "node20",
  platform: "node",
});
