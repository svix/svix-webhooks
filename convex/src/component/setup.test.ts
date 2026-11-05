/// <reference types="vite/client" />
import { test } from "vitest";
import schema from "./schema.js";
import { convexTest } from "convex-test";
import workpool from "@convex-dev/workpool/test";
export const modules = import.meta.glob("./**/*.*s");

export function initConvexTest() {
  const t = convexTest(schema, modules);
  workpool.register(t, "deliveryPool");
  return t;
}
test("setup", () => {});
