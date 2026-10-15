/// <reference types="vite/client" />
import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";
import { api, internal } from "./_generated/api.js";
import {
  DEFAULT_RETENTION_DAYS,
  DELIVERY_DEADLINE_MS,
  MAX_ATTEMPTS,
  MAX_BACKOFF_MS,
  MAX_LIST_LIMIT,
  parseRetentionDays,
} from "./messages.js";
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

/** Moves the clock past the deadline of any scheduled delivery attempt. */
function passDeliveryDeadline() {
  vi.setSystemTime(Date.now() + MAX_BACKOFF_MS * 2 + DELIVERY_DEADLINE_MS);
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
    expect(await req.json()).toMatchObject({
      eventType: "order.created",
      eventId: `convex-${messageId}`,
      payload: { orderId: 1 },
      channels: ["orders"],
      application: { name: "Customer 1", uid: "app_1" },
    });
  });

  test("retries transient errors with the same idempotency key", async () => {
    let calls = 0;
    const requests = mockSvix(() =>
      ++calls <= 4
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

    // The SDK itself retries twice, so the second delivery attempt succeeds.
    const message = await t.query(api.messages.get, { messageId });
    expect(message).toMatchObject({ status: "sent", attempts: 2 });
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
    await t.mutation(api.messages.retry, { messageId });
    await t.finishAllScheduledFunctions(vi.runAllTimers);
    expect((await t.query(api.messages.get, { messageId }))?.status).toBe(
      "sent",
    );
  });

  test("treats a duplicate eventId as sent", async () => {
    const requests = mockSvix(() =>
      Response.json(
        { code: "conflict", detail: "Event ID already exists" },
        { status: 409 },
      ),
    );
    const t = initConvexTest();
    const withEventId = await t.mutation(api.messages.send, {
      appId: "app_1",
      eventType: "order.created",
      payload: "{}",
      eventId: "evt_1",
    });
    const withoutEventId = await t.mutation(api.messages.send, {
      appId: "app_1",
      eventType: "order.created",
      payload: "{}",
    });

    await t.finishAllScheduledFunctions(vi.runAllTimers);

    const sent = await t.query(api.messages.get, { messageId: withEventId });
    expect(sent).toMatchObject({ status: "sent", attempts: 1 });
    expect(sent?.svixMessageId).toBeUndefined();
    // Without an eventId, the generated one conflicts if an earlier attempt
    // already reached Svix.
    const resent = await t.query(api.messages.get, {
      messageId: withoutEventId,
    });
    expect(resent).toMatchObject({ status: "sent", attempts: 1 });
    expect(requests).toHaveLength(2);
    expect(await requests[0].json()).toMatchObject({ eventId: "evt_1" });
    expect(await requests[1].json()).toMatchObject({
      eventId: `convex-${withoutEventId}`,
    });
  });

  test("rejects payloads that aren't JSON", async () => {
    const t = initConvexTest();
    await expect(
      t.mutation(api.messages.send, {
        appId: "app_1",
        eventType: "order.created",
        payload: "{not json",
      }),
    ).rejects.toThrow("JSON-encoded");
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
    expect(
      await t.mutation(api.messages.retryFailed, {
        appId: "app_1",
        batchSize: 2,
      }),
    ).toEqual({ retried: 2, hasMore: true });
    expect(
      await t.mutation(api.messages.retryFailed, { appId: "app_1" }),
    ).toEqual({ retried: 1, hasMore: false });
    await t.finishAllScheduledFunctions(vi.runAllTimers);
    expect(await statuses()).toEqual(["sent", "sent", "sent", "failed"]);

    expect(await t.mutation(api.messages.retryFailed, {})).toEqual({
      retried: 1,
      hasMore: false,
    });
    await t.finishAllScheduledFunctions(vi.runAllTimers);
    expect(await statuses()).toEqual(["sent", "sent", "sent", "sent"]);
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

  test("lists and cleans up messages", async () => {
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

    const result = await t.mutation(api.messages.cleanup, {
      before: Date.now() + 1,
      batchSize: 2,
    });
    expect(result).toEqual({ deleted: 2, hasMore: true });
    const rest = await t.mutation(api.messages.cleanup, {
      before: Date.now() + 1,
    });
    expect(rest).toEqual({ deleted: 1, hasMore: false });
    await t.run(async (ctx) => {
      expect(await ctx.db.query("payloads").collect()).toEqual([]);
    });
  });

  test("clamps the cleanup batch size", async () => {
    mockSvix(() => messageOut());
    const t = initConvexTest();
    for (let i = 0; i < 3; i++) {
      await t.mutation(api.messages.send, {
        appId: "app_1",
        eventType: "order.created",
        payload: "{}",
      });
    }
    await t.finishAllScheduledFunctions(vi.runAllTimers);
    for (const batchSize of [0, -5, 0.5]) {
      expect(
        await t.mutation(api.messages.cleanup, {
          before: Date.now() + 1,
          batchSize,
        }),
      ).toMatchObject({ deleted: 1 });
    }
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

  describe("requeueStuck", () => {
    test("re-queues messages whose delivery job ended early", async () => {
      mockSvix(() => messageOut());
      const t = initConvexTest();
      const messageId = await t.mutation(api.messages.send, {
        appId: "app_1",
        eventType: "order.created",
        payload: "{}",
      });
      // Simulates a job that ended without recording an outcome.
      await t.run(async (ctx) => {
        const message = await ctx.db.get("messages", messageId);
        await ctx.scheduler.cancel(message!.deliveryJobId!);
      });

      passDeliveryDeadline();
      await t.mutation(internal.messages.requeueStuck, {});

      const message = await t.query(api.messages.get, { messageId });
      expect(message).toMatchObject({
        status: "pending",
        attempts: 1,
        lastError: "Delivery attempt didn't complete",
      });
      const job = await t.run(
        async (ctx) =>
          await ctx.db.system.get(
            "_scheduled_functions",
            message!.deliveryJobId!,
          ),
      );
      expect(job?.state.kind).toBe("pending");

      await t.finishAllScheduledFunctions(vi.runAllTimers);
      expect((await t.query(api.messages.get, { messageId }))?.status).toBe(
        "sent",
      );
    });

    test("records the error of a failed delivery job", async () => {
      mockSvix(() => messageOut());
      const t = initConvexTest();
      const messageId = await t.mutation(api.messages.send, {
        appId: "app_1",
        eventType: "order.created",
        payload: "{}",
      });
      await t.run(async (ctx) => {
        const message = await ctx.db.get("messages", messageId);
        // System tables are read-only in real deployments, but not in
        // convex-test, which lets us fake a job that timed out.
        const db = ctx.db as unknown as {
          patch: (table: string, id: string, value: object) => Promise<void>;
        };
        await db.patch("_scheduled_functions", message!.deliveryJobId!, {
          state: { kind: "failed", error: "Function timed out" },
        });
      });

      passDeliveryDeadline();
      await t.mutation(internal.messages.requeueStuck, {});

      const message = await t.query(api.messages.get, { messageId });
      expect(message?.lastError).toBe(
        "Delivery attempt failed: Function timed out",
      );
    });

    test("leaves messages that aren't overdue alone", async () => {
      mockSvix(() => messageOut());
      const t = initConvexTest();
      const messageId = await t.mutation(api.messages.send, {
        appId: "app_1",
        eventType: "order.created",
        payload: "{}",
      });
      // Even without a delivery job, it isn't looked at before its deadline.
      await t.run(async (ctx) => {
        const message = await ctx.db.get("messages", messageId);
        await ctx.scheduler.cancel(message!.deliveryJobId!);
      });
      const before = await t.query(api.messages.get, { messageId });

      await t.mutation(internal.messages.requeueStuck, {});

      expect(await t.query(api.messages.get, { messageId })).toEqual(before);
    });

    test("extends the deadline of overdue messages still being delivered", async () => {
      mockSvix(() => messageOut());
      const t = initConvexTest();
      const messageId = await t.mutation(api.messages.send, {
        appId: "app_1",
        eventType: "order.created",
        payload: "{}",
      });
      const before = await t.query(api.messages.get, { messageId });

      passDeliveryDeadline();
      await t.mutation(internal.messages.requeueStuck, {});

      expect(await t.query(api.messages.get, { messageId })).toEqual(before);
      const deliverBy = await t.run(
        async (ctx) => (await ctx.db.get("messages", messageId))!.deliverBy,
      );
      expect(deliverBy).toBe(Date.now() + DELIVERY_DEADLINE_MS);
    });

    test("gives up after MAX_ATTEMPTS", async () => {
      mockSvix(() => messageOut());
      const t = initConvexTest();
      const messageId = await t.mutation(api.messages.send, {
        appId: "app_1",
        eventType: "order.created",
        payload: "{}",
      });
      for (let i = 0; i < MAX_ATTEMPTS; i++) {
        await t.run(async (ctx) => {
          const message = await ctx.db.get("messages", messageId);
          await ctx.scheduler.cancel(message!.deliveryJobId!);
        });
        passDeliveryDeadline();
        await t.mutation(internal.messages.requeueStuck, {});
      }
      const message = await t.query(api.messages.get, { messageId });
      expect(message).toMatchObject({
        status: "failed",
        attempts: MAX_ATTEMPTS,
      });
      expect(message?.deliveryJobId).toBeUndefined();
    });

    test("pages through more overdue messages than fit in a batch", async () => {
      mockSvix(() => messageOut());
      const t = initConvexTest();
      const ids = [];
      for (let i = 0; i < 101; i++) {
        ids.push(
          await t.mutation(api.messages.send, {
            appId: "app_1",
            eventType: "order.created",
            payload: "{}",
          }),
        );
      }
      await t.run(async (ctx) => {
        for (const message of await ctx.db.query("messages").collect()) {
          await ctx.scheduler.cancel(message.deliveryJobId!);
        }
      });

      passDeliveryDeadline();
      await t.mutation(internal.messages.requeueStuck, {});
      await t.finishAllScheduledFunctions(vi.runAllTimers);

      for (const messageId of ids) {
        expect(await t.query(api.messages.get, { messageId })).toMatchObject({
          status: "sent",
          attempts: 2,
        });
      }
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
