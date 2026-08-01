import { test } from "node:test";
import { strict as assert } from "node:assert/strict";

import { Svix } from "./index";

/** Exposes the request context the real `Svix` constructor built. */
class SvixUnderTest extends Svix {
  public get ctx() {
    return this.requestCtx;
  }
}

function baseUrl(token: string, serverUrl?: string): string {
  return new SvixUnderTest(token, serverUrl === undefined ? {} : { serverUrl }).ctx
    .baseUrl;
}

test("serverUrl trailing slashes are stripped", () => {
  assert.equal(baseUrl("token", "https://api.example.com/"), "https://api.example.com");
  assert.equal(baseUrl("token", "https://api.example.com///"), "https://api.example.com");
  assert.equal(
    baseUrl("token", "https://api.example.com/prefix/"),
    "https://api.example.com/prefix"
  );
});

test("serverUrl without a trailing slash is unchanged", () => {
  assert.equal(baseUrl("token", "https://api.example.com"), "https://api.example.com");
});

test("default and regional server URLs are used as-is", () => {
  assert.equal(baseUrl("token"), "https://api.svix.com");
  assert.equal(baseUrl("testsk.eu.abc"), "https://api.eu.svix.com");
});
