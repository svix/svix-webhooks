import { test } from "node:test";
import { strict as assert } from "node:assert/strict";

import { Svix } from "./index";
import { SvixInternal } from "./api_internal";

test("Svix client modules load without circular initialization errors", () => {
  const client = new Svix("test-token", { serverUrl: "https://api.example.com" });
  assert.equal(typeof client.application, "object");

  const internal = new SvixInternal("test-token", { serverUrl: "https://api.example.com" });
  assert.equal(internal.getRequestCtx().baseUrl, "https://api.example.com");
});
