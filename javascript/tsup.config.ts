import { defineConfig } from "tsup";

export default defineConfig({
  entry: ["src/index.ts"],
  dts: true,
  clean: true,
  treeshake: true,
  format: ["esm", "cjs"],
});
