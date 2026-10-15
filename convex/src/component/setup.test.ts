/// <reference types="vite/client" />
import { test } from "vitest";
import schema from "./schema.js";
import { convexTest } from "convex-test";
export const modules = import.meta.glob("./**/*.*s");

export function initConvexTest() {
  return convexTest(schema, modules);
}
test("setup", () => {});
