import { afterEach, beforeEach, describe, expect, test, vi } from "vitest";
import { defineTestApp } from "convex-test";
import {
  defineSchema,
  defineTable,
  httpRouter,
  makeFunctionReference,
} from "convex/server";
import { v } from "convex/values";
import { Webhook } from "svix";
import { Svix } from "./index.js";
import componentTest from "../test.js";

const SECRET = "whsec_MfKQ9r8GKYqrTwjUPD8ILPZIo2LaLaSw";

const app = defineTestApp({
  schema: defineSchema({
    events: defineTable({ svixId: v.string(), payload: v.any() }),
  }),
  components: { svix: componentTest },
});
const svix = new Svix(app.components.svix, { webhookSecret: SECRET });

// Receiving webhooks: an HTTP route like the one in the example's http.ts.
const http = httpRouter();
http.route({
  path: "/webhooks/svix",
  method: "POST",
  handler: svix.webhookHandler(async (ctx, event) => {
    await ctx.runMutation(makeFunctionReference<"mutation">("example:record"), {
      svixId: event.id,
      payload: event.payload,
    });
  }),
});

const { api, createTest } = app.defineModules({
  example: {
    send: app.mutation({
      args: { fail: v.boolean() },
      handler: async (ctx, args) => {
        const messageId = await svix.sendMessage(ctx, {
          appId: "app_1",
          eventType: "user.signup",
          // `$`-prefixed keys aren't valid in Convex values.
          payload: { email: "a@example.com", $schema: "user.signup.v1" },
        });
        if (args.fail) {
          throw new Error("rolled back");
        }
        return messageId;
      },
    }),
    status: app.query({
      args: { messageId: v.string() },
      handler: async (ctx, args) =>
        (await svix.getMessage(ctx, args.messageId))?.status ?? null,
    }),
    payload: app.query({
      args: { messageId: v.string() },
      handler: async (ctx, args) =>
        JSON.stringify((await svix.getMessage(ctx, args.messageId))?.payload),
    }),
    list: app.query({
      args: {},
      handler: async (ctx) => await svix.listMessages(ctx, { appId: "app_1" }),
    }),
    portal: app.action({
      args: {},
      handler: async (ctx) =>
        await svix.appPortalAccess(ctx, {
          appId: "app_1",
          capabilities: ["ViewBase"],
        }),
    }),
    record: app.mutation({
      args: { svixId: v.string(), payload: v.any() },
      handler: async (ctx, args) => {
        await ctx.db.insert("events", args);
      },
    }),
    events: app.query({
      args: {},
      handler: async (ctx) => await ctx.db.query("events").collect(),
    }),
  },
  http: { default: http },
});

describe("Svix client", () => {
  beforeEach(() => {
    vi.useFakeTimers();
    process.env.SVIX_API_KEY = "sk_test.us";
    process.env.SVIX_SERVER_URL = "https://svix.test";
  });
  afterEach(() => {
    vi.useRealTimers();
    vi.unstubAllGlobals();
  });

  test("sends messages transactionally", async () => {
    const fetchMock = vi.fn(async () =>
      Response.json({
        id: "msg_1",
        eventType: "user.signup",
        payload: {},
        timestamp: new Date().toISOString(),
      }),
    );
    vi.stubGlobal("fetch", fetchMock);
    const t = createTest();

    await expect(t.mutation(api.example.send, { fail: true })).rejects.toThrow(
      "rolled back",
    );
    expect(await t.query(api.example.list, {})).toEqual([]);

    const messageId = await t.mutation(api.example.send, { fail: false });
    await t.finishAllScheduledFunctions(vi.runAllTimers);
    expect(await t.query(api.example.status, { messageId })).toBe("sent");
    expect(fetchMock).toHaveBeenCalledTimes(1);
    const sentPayload = (await new Request(
      ...(fetchMock.mock.calls[0] as unknown as [RequestInfo, RequestInit]),
    ).json()) as { payload: unknown };
    const payload = { email: "a@example.com", $schema: "user.signup.v1" };
    expect(sentPayload.payload).toEqual(payload);
    expect(
      JSON.parse(await t.query(api.example.payload, { messageId })),
    ).toEqual(payload);
  });

  test("gets an app portal link", async () => {
    vi.stubGlobal(
      "fetch",
      vi.fn(async (input: RequestInfo | URL, init?: RequestInit) => {
        const req = new Request(input, init);
        expect(req.url).toBe(
          "https://svix.test/api/v1/auth/app-portal-access/app_1",
        );
        expect(await req.json()).toEqual({ capabilities: ["ViewBase"] });
        return Response.json({
          url: "https://app.svix.com/login#key=x",
          token: "x",
        });
      }),
    );
    const t = createTest();
    expect(await t.action(api.example.portal, {})).toEqual({
      url: "https://app.svix.com/login#key=x",
      token: "x",
    });
  });

  test("verifies incoming webhooks", async () => {
    vi.useRealTimers();
    const t = createTest();
    const body = JSON.stringify({ type: "invoice.paid", data: { id: 1 } });
    const headers = (secret: string) => {
      const timestamp = new Date();
      return {
        "content-type": "application/json",
        "svix-id": "msg_in_1",
        "svix-timestamp": Math.floor(timestamp.getTime() / 1000).toString(),
        "svix-signature": new Webhook(secret).sign("msg_in_1", timestamp, body),
      };
    };

    const bad = await t.fetch("/webhooks/svix", {
      method: "POST",
      body,
      headers: headers("whsec_" + btoa("not the right secret")),
    });
    expect(bad.status).toBe(400);
    expect(await t.query(api.example.events, {})).toEqual([]);

    const ok = await t.fetch("/webhooks/svix", {
      method: "POST",
      body,
      headers: headers(SECRET),
    });
    expect(ok.status).toBe(204);
    const events = await t.query(api.example.events, {});
    expect(events).toMatchObject([
      {
        svixId: "msg_in_1",
        payload: { type: "invoice.paid", data: { id: 1 } },
      },
    ]);
  });
});
