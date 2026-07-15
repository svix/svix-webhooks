import { defineConfig } from "tsdown";

export default defineConfig({
  entry: ["src/index.ts"],
  outDir: "dist",
  format: ["esm"],
  target: "es2022",
  dts: true,
  sourcemap: true,
  clean: true,
});
