/// <reference types="vite/client" />
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";
import { api, internal } from "./_generated/api.js";
import {
  DEFAULT_RETENTION_DAYS,
  MAX_ATTEMPTS,
  MAX_LIST_LIMIT,
  parseRetentionDays,
} from "./messages.js";
import type { WorkId } from "@convex-dev/workpool";
import type { Id } from "./_generated/dataModel.js";
import { initConvexTest } from "./setup.test.js";

type Handler = (req: Request) => Response | Promise<Response>;

function mockSvix(handler: Handler) {
  const requests: Request[] = [];
  vi.stubGlobal(
    "fetch",
    vi.fn(async (input: RequestInfo | URL, init?: RequestInit) => {
      const req = new Request(input, init);
      requests.push(req.clone());
      return await handler(req);
    }),
  );
  return requests;
}

function messageOut(id = "msg_1") {
  return Response.json({
    id,
    eventType: "order.created",
    payload: "{}",
    timestamp: new Date().toISOString(),
  });
}

describe("messages", () => {
  beforeEach(() => {
    vi.useFakeTimers();
    process.env.SVIX_API_KEY = "sk_test.us";
    process.env.SVIX_SERVER_URL = "https://svix.test";
  });
  afterEach(() => {
    vi.useRealTimers();
    vi.unstubAllGlobals();
    delete process.env.SVIX_MESSAGE_RETENTION_DAYS;
    vi.restoreAllMocks();
  });

  test("sends a message to Svix", async () => {
    const requests = mockSvix(() => messageOut("msg_abc"));
    const t = initConvexTest();
    const messageId = await t.mutation(api.messages.send, {
      appId: "app_1",
      eventType: "order.created",
      payload: JSON.stringify({ orderId: 1 }),
      channels: ["orders"],
      application: { name: "Customer 1", uid: "app_1" },
    });
    expect((await t.query(api.messages.get, { messageId }))?.status).toBe(
      "pending",
    );

    await t.finishAllScheduledFunctions(vi.runAllTimers);

    const message = await t.query(api.messages.get, { messageId });
    expect(message).toMatchObject({
      status: "sent",
      attempts: 1,
      svixMessageId: "msg_abc",
    });
    expect(requests).toHaveLength(1);
    const req = requests[0];
    expect(new URL(req.url).pathname).toBe("/api/v1/app/app_1/msg");
    expect(req.headers.get("authorization")).toBe("Bearer sk_test.us");
    expect(req.headers.get("idempotency-key")).toBe(`convex-${messageId}`);
    const body = await req.json();
    expect(body).toMatchObject({
      eventType: "order.created",
      payload: { orderId: 1 },
      channels: ["orders"],
      application: { name: "Customer 1", uid: "app_1" },
    });
    expect(body.eventId).toBeUndefined();
  });

  test("retries transient errors with the same idempotency key", async () => {
    let calls = 0;
    const requests = mockSvix(() =>
      ++calls <= 2
        ? Response.json({ code: "unavailable" }, { status: 503 })
        : messageOut(),
    );
    const t = initConvexTest();
    const messageId = await t.mutation(api.messages.send, {
      appId: "app_1",
      eventType: "order.created",
      payload: "{}",
    });

    await t.finishAllScheduledFunctions(vi.runAllTimers);

    // The SDK's own retries are disabled, so each request is an attempt.
    const message = await t.query(api.messages.get, { messageId });
    expect(message).toMatchObject({ status: "sent", attempts: 3 });
    expect(requests).toHaveLength(3);
    const keys = new Set(requests.map((r) => r.headers.get("idempotency-key")));
    expect(keys).toEqual(new Set([`convex-${messageId}`]));
  });

  test("doesn't retry client errors", async () => {
    const requests = mockSvix(() =>
      Response.json(
        { code: "not_found", detail: "Application not found" },
        { status: 404 },
      ),
    );
    const t = initConvexTest();
    const messageId = await t.mutation(api.messages.send, {
      appId: "app_missing",
      eventType: "order.created",
      payload: "{}",
    });

    await t.finishAllScheduledFunctions(vi.runAllTimers);

    const message = await t.query(api.messages.get, { messageId });
    expect(message?.status).toBe("failed");
    expect(message?.attempts).toBe(1);
    expect(message?.lastError).toContain("404");
    expect(requests).toHaveLength(1);

    // Manual retry after fixing the problem.
    mockSvix(() => messageOut());
    expect(await t.mutation(api.messages.retry, { messageId })).toBe(true);
    await t.finishAllScheduledFunctions(vi.runAllTimers);
    expect((await t.query(api.messages.get, { messageId }))?.status).toBe(
      "sent",
    );
    // Only failed messages can be retried.
    expect(await t.mutation(api.messages.retry, { messageId })).toBe(false);
  });

  test("treats a duplicate eventId as sent", async () => {
    const requests = mockSvix(() =>
      Response.json(
        { code: "conflict", detail: "Event ID already exists" },
        { status: 409 },
      ),
    );
    const t = initConvexTest();
    const messageId = await t.mutation(api.messages.send, {
      appId: "app_1",
      eventType: "order.created",
      payload: "{}",
      eventId: "evt_1",
    });

    await t.finishAllScheduledFunctions(vi.runAllTimers);

    const message = await t.query(api.messages.get, { messageId });
    expect(message).toMatchObject({ status: "sent", attempts: 1 });
    expect(message?.svixMessageId).toBeUndefined();
    expect(requests).toHaveLength(1);
    expect(await requests[0].json()).toMatchObject({ eventId: "evt_1" });
  });

  test("rejects payloads that aren't JSON objects", async () => {
    const t = initConvexTest();
    for (const payload of ["{not json", "[]", "null", "5", '"hi"']) {
      await expect(
        t.mutation(api.messages.send, {
          appId: "app_1",
          eventType: "order.created",
          payload,
        }),
      ).rejects.toThrow("JSON-encoded object");
    }
  });

  test("drops empty channels and tags", async () => {
    const requests = mockSvix(() => messageOut());
    const t = initConvexTest();
    await t.mutation(api.messages.send, {
      appId: "app_1",
      eventType: "order.created",
      payload: "{}",
      channels: [],
      tags: [],
    });
    await t.finishAllScheduledFunctions(vi.runAllTimers);
    const body = await requests[0].json();
    expect(body).not.toHaveProperty("channels");
    expect(body).not.toHaveProperty("tags");
  });

  test("counts attempts that crash", async () => {
    const t = initConvexTest();
    const messageId = await t.mutation(api.messages.send, {
      appId: "app_1",
      eventType: "order.created",
      payload: "{}",
    });
    const message = await t.mutation(internal.messages.startAttempt, {
      messageId: messageId as Id<"messages">,
    });
    expect(message?.attempts).toBe(1);
    expect((await t.query(api.messages.get, { messageId }))?.attempts).toBe(1);
  });

  test("handles malformed message IDs", async () => {
    const t = initConvexTest();
    expect(await t.query(api.messages.get, { messageId: "nope" })).toBeNull();
    await expect(
      t.mutation(api.messages.retry, { messageId: "nope" }),
    ).rejects.toThrow("Message nope not found");
  });

  test("retries failed messages in bulk", async () => {
    // e.g. an invalid API key.
    mockSvix(() => Response.json({ code: "unauthorized" }, { status: 401 }));
    const t = initConvexTest();
    const ids: string[] = [];
    for (const appId of ["app_1", "app_1", "app_1", "app_2"]) {
      ids.push(
        await t.mutation(api.messages.send, {
          appId,
          eventType: "order.created",
          payload: "{}",
        }),
      );
    }
    await t.finishAllScheduledFunctions(vi.runAllTimers);
    const statuses = async () =>
      await Promise.all(
        ids.map(
          async (messageId) =>
            (await t.query(api.messages.get, { messageId }))?.status,
        ),
      );
    expect(await statuses()).toEqual(["failed", "failed", "failed", "failed"]);

    mockSvix(() => messageOut());
    await t.mutation(api.messages.retryFailed, { appId: "app_1" });
    await t.finishAllScheduledFunctions(vi.runAllTimers);
    expect(await statuses()).toEqual(["sent", "sent", "sent", "failed"]);

    await t.mutation(api.messages.retryFailed, {});
    await t.finishAllScheduledFunctions(vi.runAllTimers);
    expect(await statuses()).toEqual(["sent", "sent", "sent", "sent"]);
  });

  test("retries large backlogs in batches, once each", async () => {
    // Still failing, so requeued messages fail again while later batches run.
    const requests = mockSvix(() =>
      Response.json({ code: "unauthorized" }, { status: 401 }),
    );
    const t = initConvexTest();
    for (let i = 0; i < 250; i++) {
      await t.mutation(api.messages.send, {
        appId: "app_1",
        eventType: "order.created",
        payload: "{}",
      });
    }
    await t.finishAllScheduledFunctions(vi.runAllTimers);
    expect(requests).toHaveLength(250);

    await t.mutation(api.messages.retryFailed, {});
    await t.finishAllScheduledFunctions(vi.runAllTimers);
    expect(requests).toHaveLength(500);
  });

  test("gives up after MAX_ATTEMPTS", async () => {
    mockSvix(() => Response.json({}, { status: 500 }));
    const t = initConvexTest();
    const messageId = await t.mutation(api.messages.send, {
      appId: "app_1",
      eventType: "order.created",
      payload: "{}",
    });

    await t.finishAllScheduledFunctions(vi.runAllTimers);

    const message = await t.query(api.messages.get, { messageId });
    expect(message).toMatchObject({
      status: "failed",
      attempts: MAX_ATTEMPTS,
    });
  });

  test("lists messages", async () => {
    mockSvix(() => messageOut());
    const t = initConvexTest();
    for (const appId of ["app_1", "app_1", "app_2"]) {
      await t.mutation(api.messages.send, {
        appId,
        eventType: "order.created",
        payload: "{}",
      });
    }
    await t.finishAllScheduledFunctions(vi.runAllTimers);
    expect(await t.query(api.messages.list, { appId: "app_1" })).toHaveLength(
      2,
    );
    expect(
      await t.query(api.messages.list, { appId: "app_1", limit: 1 }),
    ).toHaveLength(1);
    for (const message of await t.query(api.messages.list, {
      appId: "app_1",
    })) {
      expect(message).not.toHaveProperty("payload");
      expect(message).not.toHaveProperty("payloadId");
    }

    expect(
      await t.query(api.messages.list, { appId: "app_1", limit: 1.5 }),
    ).toHaveLength(1);
  });

  test("caps the number of listed messages", async () => {
    const t = initConvexTest();
    for (let i = 0; i <= MAX_LIST_LIMIT; i++) {
      await t.mutation(api.messages.send, {
        appId: "app_1",
        eventType: "order.created",
        payload: "{}",
      });
    }
    expect(
      await t.query(api.messages.list, { appId: "app_1", limit: 1000 }),
    ).toHaveLength(MAX_LIST_LIMIT);
  });

  describe("onDeliveryFailed", () => {
    // Called by the workpool when an attempt crashes or times out, without
    // `deliver` recording its outcome.
    test("marks the message as failed", async () => {
      const t = initConvexTest();
      const messageId = await t.run(async (ctx) => {
        const payloadId = await ctx.db.insert("payloads", { payload: "{}" });
        return await ctx.db.insert("messages", {
          appId: "app_1",
          eventType: "order.created",
          payloadId,
          status: "pending",
          attempts: 0,
        });
      });

      await t.mutation(internal.messages.onDeliveryFailed, {
        workId: "work_1" as WorkId,
        context: { messageId },
        result: { kind: "failed", error: "Function timed out" },
      });

      expect(await t.query(api.messages.get, { messageId })).toMatchObject({
        status: "failed",
        lastError: "Function timed out",
      });
    });

    test("leaves sent messages alone", async () => {
      mockSvix(() => messageOut());
      const t = initConvexTest();
      const messageId = await t.mutation(api.messages.send, {
        appId: "app_1",
        eventType: "order.created",
        payload: "{}",
      });
      await t.finishAllScheduledFunctions(vi.runAllTimers);

      await t.mutation(internal.messages.onDeliveryFailed, {
        workId: "work_1" as WorkId,
        context: { messageId: messageId as Id<"messages"> },
        result: { kind: "canceled" },
      });

      expect((await t.query(api.messages.get, { messageId }))?.status).toBe(
        "sent",
      );
    });
  });

  describe("deleteExpired", () => {
    const DAY_MS = 24 * 60 * 60 * 1000;

    async function sendAll(
      t: ReturnType<typeof initConvexTest>,
      count: number,
    ): Promise<string[]> {
      const ids = [];
      for (let i = 0; i < count; i++) {
        ids.push(
          await t.mutation(api.messages.send, {
            appId: "app_1",
            eventType: "order.created",
            payload: "{}",
          }),
        );
      }
      await t.finishAllScheduledFunctions(vi.runAllTimers);
      return ids;
    }

    async function remaining(t: ReturnType<typeof initConvexTest>) {
      return await t.run(async (ctx) => ({
        messages: (await ctx.db.query("messages").collect()).length,
        payloads: (await ctx.db.query("payloads").collect()).length,
      }));
    }

    test("deletes messages older than the retention period", async () => {
      mockSvix(() => messageOut());
      const t = initConvexTest();
      await sendAll(t, 2);
      vi.setSystemTime(Date.now() + (DEFAULT_RETENTION_DAYS - 1) * DAY_MS);
      const [recent] = await sendAll(t, 1);

      vi.setSystemTime(Date.now() + 2 * DAY_MS);
      await t.mutation(internal.messages.deleteExpired, {});

      expect(await remaining(t)).toEqual({ messages: 1, payloads: 1 });
      expect(
        await t.query(api.messages.get, { messageId: recent }),
      ).not.toBeNull();
    });

    test("never deletes pending messages", async () => {
      mockSvix(() => Response.json({}, { status: 500 }));
      const t = initConvexTest();
      await t.mutation(api.messages.send, {
        appId: "app_1",
        eventType: "order.created",
        payload: "{}",
      });

      vi.setSystemTime(Date.now() + (DEFAULT_RETENTION_DAYS + 1) * DAY_MS);
      await t.mutation(internal.messages.deleteExpired, {});

      expect(await remaining(t)).toEqual({ messages: 1, payloads: 1 });
    });

    test("uses SVIX_MESSAGE_RETENTION_DAYS", async () => {
      mockSvix(() => messageOut());
      const t = initConvexTest();
      await sendAll(t, 1);
      vi.setSystemTime(Date.now() + 2 * DAY_MS);

      process.env.SVIX_MESSAGE_RETENTION_DAYS = "0";
      await t.mutation(internal.messages.deleteExpired, {});
      expect(await remaining(t)).toEqual({ messages: 1, payloads: 1 });

      process.env.SVIX_MESSAGE_RETENTION_DAYS = "1";
      await t.mutation(internal.messages.deleteExpired, {});
      expect(await remaining(t)).toEqual({ messages: 0, payloads: 0 });
    });

    test("keeps going until all expired messages are deleted", async () => {
      mockSvix(() => messageOut());
      const t = initConvexTest();
      await sendAll(t, 101);

      vi.setSystemTime(Date.now() + (DEFAULT_RETENTION_DAYS + 1) * DAY_MS);
      await t.mutation(internal.messages.deleteExpired, {});
      await t.finishAllScheduledFunctions(vi.runAllTimers);

      expect(await remaining(t)).toEqual({ messages: 0, payloads: 0 });
    });

    test("parses the retention period", () => {
      expect(parseRetentionDays(undefined)).toBe(DEFAULT_RETENTION_DAYS);
      expect(parseRetentionDays("")).toBe(DEFAULT_RETENTION_DAYS);
      expect(parseRetentionDays("7")).toBe(7);
      expect(parseRetentionDays("0.5")).toBe(0.5);
      for (const value of ["-1", "abc", "Infinity"]) {
        expect(() => parseRetentionDays(value)).toThrow(
          "Invalid SVIX_MESSAGE_RETENTION_DAYS",
        );
      }
    });
  });
});
