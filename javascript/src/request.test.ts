import { test } from "node:test";
import { strict as assert } from "node:assert/strict";

import { Svix } from "./index";
import { ApiException } from "./util";

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

function countingServerErrorFetch(counter: { calls: number }): typeof fetch {
  return (async () => {
    counter.calls += 1;
    return new Response(`{"code":"500","detail":"asd"}`, { status: 500 });
  }) as typeof fetch;
}

test("numRetries: 0 disables retries", async () => {
  const counter = { calls: 0 };
  const svx = new Svix("token", {
    serverUrl: "https://api.example.com",
    numRetries: 0,
    fetch: countingServerErrorFetch(counter),
  });

  await assert.rejects(svx.application.list(), ApiException);
  assert.equal(counter.calls, 1);
});

test("numRetries: NaN is rejected", () => {
  assert.throws(() => new Svix("token", { numRetries: NaN }), {
    name: "TypeError",
    message: "numRetries must not be NaN",
  });
});

test("default retries twice after the initial request", async () => {
  const counter = { calls: 0 };
  const svx = new Svix("token", {
    serverUrl: "https://api.example.com",
    fetch: countingServerErrorFetch(counter),
  });

  await assert.rejects(svx.application.list(), ApiException);
  assert.equal(counter.calls, 3);
});
