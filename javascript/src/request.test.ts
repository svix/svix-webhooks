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

test("retries cancel unread response bodies before the next request", async () => {
  const events: string[] = [];
  let calls = 0;
  const fetchImpl: typeof fetch = async () => {
    const attempt = ++calls;
    events.push(`fetch:${attempt}`);
    if (attempt === 3) {
      return new Response('{"data":[],"iterator":null,"done":true}');
    }
    return new Response(
      new ReadableStream<Uint8Array>({
        start(controller) {
          controller.enqueue(new TextEncoder().encode("unread error body"));
        },
        cancel() {
          events.push(`cancel:${attempt}`);
        },
      }),
      { status: 503 }
    );
  };
  const svx = new Svix("token", {
    serverUrl: "https://api.example.com",
    retryScheduleInMs: [0, 0],
    fetch: fetchImpl,
  });

  const result = await svx.application.list();

  assert.deepEqual(result.data, []);
  assert.deepEqual(events, ["fetch:1", "cancel:1", "fetch:2", "cancel:2", "fetch:3"]);
});

test("retry exhaustion keeps the final error body readable", async () => {
  let calls = 0;
  let cancelled = 0;
  const terminalBody = '{"detail":"still unavailable"}';
  const svx = new Svix("token", {
    serverUrl: "https://api.example.com",
    retryScheduleInMs: [0],
    fetch: async () => {
      calls += 1;
      return calls === 1
        ? new Response(
            new ReadableStream({
              cancel() {
                cancelled += 1;
              },
            }),
            {
              status: 503,
            }
          )
        : new Response(terminalBody, { status: 503 });
    },
  });

  await assert.rejects(svx.application.list(), (error: unknown) => {
    assert.ok(error instanceof ApiException);
    assert.equal(error.code, 503);
    assert.equal(error.body, terminalBody);
    return true;
  });
  assert.equal(calls, 2);
  assert.equal(cancelled, 1);
});

test("a failed body cancellation does not prevent the scheduled retry", async () => {
  let calls = 0;
  let cancellations = 0;
  const svx = new Svix("token", {
    serverUrl: "https://api.example.com",
    retryScheduleInMs: [0],
    fetch: async () => {
      calls += 1;
      return calls === 1
        ? new Response(
            new ReadableStream({
              cancel() {
                cancellations += 1;
                throw new Error("body already failed");
              },
            }),
            { status: 500 }
          )
        : new Response('{"data":[],"iterator":null,"done":true}');
    },
  });

  assert.deepEqual((await svx.application.list()).data, []);
  assert.equal(calls, 2);
  assert.equal(cancellations, 1);
});

for (const status of [400, 503]) {
  test(`a non-retried ${status} response retains its error body`, async () => {
    let calls = 0;
    const errorBody = '{"detail":"not retried"}';
    const svx = new Svix("token", {
      serverUrl: "https://api.example.com",
      numRetries: 0,
      fetch: async () => {
        calls += 1;
        return new Response(errorBody, { status });
      },
    });

    await assert.rejects(svx.application.list(), (error: unknown) => {
      assert.ok(error instanceof ApiException);
      assert.equal(error.code, status);
      assert.deepEqual(error.body, status === 400 ? JSON.parse(errorBody) : errorBody);
      return true;
    });
    assert.equal(calls, 1);
  });
}
