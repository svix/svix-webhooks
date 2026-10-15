# Svix Convex Component

Send and receive webhooks from your [Convex](https://convex.dev) app with
[Svix](https://www.svix.com).

- **Transactional sending.** Call `svix.sendMessage(ctx, ...)` from a mutation.
  The message is only sent if the mutation commits, so you never notify your
  customers about something that was rolled back.
- **Reliable delivery to Svix.** Messages are delivered in the background,
  retried with exponential backoff on transient errors, and sent with an
  `eventId` so retries never create duplicates.
- **Delivery status.** Query a message's status (`pending`, `sent` or `failed`)
  reactively from your app.
- **App Portal.** Generate links to the Svix App Portal, where your customers
  manage their own webhook endpoints.
- **Receiving webhooks.** A ready-made HTTP action that verifies Svix webhook
  signatures before handing the event to your code.

## Installation

```sh
npm install @svix/convex
```

Install the component in `convex/convex.config.ts` and pass it your Svix API
key. Declaring the key as an app env var and passing it by reference means the
component always sees its current value.

```ts
// convex/convex.config.ts
import { defineApp } from "convex/server";
import { v } from "convex/values";
import svix from "@svix/convex/convex.config.js";

const app = defineApp({
  env: {
    SVIX_API_KEY: v.string(),
  },
});
app.use(svix, { env: { SVIX_API_KEY: app.env.SVIX_API_KEY } });

export default app;
```

```sh
npx convex env set SVIX_API_KEY sk_...
```

The component also accepts these optional env vars:

- `SVIX_SERVER_URL`, for self-hosted Svix or a specific region. By default, the
  region is inferred from the API key.
- `SVIX_MESSAGE_RETENTION_DAYS`, how many days to keep sent and failed messages
  for (default `30`). See [Cleaning up](#cleaning-up).

## Sending webhooks

```ts
// convex/orders.ts
import { Svix } from "@svix/convex";
import { v } from "convex/values";
import { components } from "./_generated/api";
import { mutation, query } from "./_generated/server";

export const svix = new Svix(components.svix);

export const createOrder = mutation({
  args: { customerId: v.string(), amount: v.number() },
  handler: async (ctx, args) => {
    const orderId = await ctx.db.insert("orders", args);
    await svix.sendMessage(ctx, {
      // A Svix application ID or UID. Using your own customer ID as the UID
      // means you don't need to store a Svix ID.
      appId: args.customerId,
      eventType: "order.created",
      payload: { orderId, amount: args.amount },
      // Optional: create the application if it doesn't exist yet.
      application: { name: "Acme Inc.", uid: args.customerId },
    });
    return orderId;
  },
});
```

`sendMessage` accepts `eventType`, `payload`, and optionally `eventId`,
`channels`, `tags` and `application`, which map to the
[Create Message](https://api.svix.com/docs#tag/Message/operation/v1.message.create)
API. It can also be called from actions.

The payload can be any JSON-serializable object. It's stored JSON-encoded, so
keys that aren't valid in Convex values (such as `$schema`) are fine, but the
encoded payload must fit in a Convex document (1 MiB).

If you don't set an `eventId`, the component uses `convex-<messageId>`, which
Svix uses to reject duplicates when a message is retried. If Svix has already
seen the `eventId` in your environment, the message is marked as `sent` without
a `svixMessageId`, as the event was already delivered to Svix.

### Tracking delivery

`sendMessage` returns an ID you can use to look up the message:

```ts
export const messageStatus = query({
  args: { messageId: v.string() },
  handler: async (ctx, args) => {
    const message = await svix.getMessage(ctx, args.messageId);
    // message.status: "pending" | "sent" | "failed"
    // message.svixMessageId: the Svix message ID, once sent
    // message.lastError: why the last attempt failed
    return message;
  },
});
```

`svix.listMessages(ctx, { appId, limit })` returns the most recent messages for
an application, up to 100. It leaves out their payloads, which can be large; use
`getMessage` to get one.

Server errors, rate limiting and network errors are retried up to 8 times with
exponential backoff. Other errors (for example, a 4xx because the application
doesn't exist, or an invalid API key) fail the message immediately. Use
`svix.retryMessage(ctx, messageId)` to retry a failed message, or
`svix.retryFailedMessages(ctx, { appId })` to retry failed messages in bulk once
you've fixed the problem (omit `appId` to retry all of them). It retries up to
`batchSize` (default 100) messages per call; call it again while `hasMore` is
true.

If a delivery attempt ends without recording its outcome (for example, because
the action timed out), a cron in the component notices within 20 minutes of the
attempt being due and re-queues the message, counting it as a failed attempt.

This status is about delivering the message _to Svix_. Svix then takes care of
delivering it to your customers' endpoints, with its own retries, which you can
follow in the Svix dashboard or App Portal.

### Cleaning up

Once a day, the component deletes sent and failed messages (and their payloads)
older than 30 days. Pending messages are never deleted. To keep messages for a
different number of days, set `SVIX_MESSAGE_RETENTION_DAYS`, or set it to `0` to
keep them forever:

```ts
// convex/convex.config.ts
app.use(svix, {
  env: { SVIX_API_KEY: app.env.SVIX_API_KEY, SVIX_MESSAGE_RETENTION_DAYS: "7" },
});
```

For a custom policy, set it to `0` and call `cleanupMessages` from your own
cron. It deletes up to `batchSize` (default 100, at most 1000) messages per
call:

```ts
// convex/crons.ts
import { cronJobs } from "convex/server";
import { internal } from "./_generated/api";
import { internalMutation } from "./_generated/server";
import { svix } from "./orders";

export const cleanupSvixMessages = internalMutation({
  args: {},
  handler: async (ctx) => {
    const { hasMore } = await svix.cleanupMessages(ctx, {
      before: Date.now() - 7 * 24 * 60 * 60 * 1000,
    });
    if (hasMore) {
      await ctx.scheduler.runAfter(0, internal.crons.cleanupSvixMessages, {});
    }
  },
});

const crons = cronJobs();
crons.daily(
  "cleanup svix messages",
  { hourUTC: 4, minuteUTC: 0 },
  internal.crons.cleanupSvixMessages,
  {},
);
export default crons;
```

## Managing applications and the App Portal

The App Portal lets your customers add endpoints, see delivery attempts and
replay messages, without you building any UI. Generate a short-lived link from
an action:

```ts
export const webhooksPortalUrl = action({
  args: {},
  handler: async (ctx) => {
    const customerId = await getCustomerId(ctx); // your auth logic
    const { url } = await svix.appPortalAccess(ctx, {
      appId: customerId,
      application: { name: "Acme Inc.", uid: customerId },
    });
    return url;
  },
});
```

You can also manage applications directly with
`svix.getOrCreateApplication(ctx, { name, uid })` and
`svix.deleteApplication(ctx, appId)`.

For anything else, use the
[Svix JavaScript SDK](https://www.npmjs.com/package/svix) from a Convex action.

## Receiving webhooks

`svix.webhookHandler` returns an HTTP action that verifies the webhook's
signature, then calls your handler. It reads the signing secret from the
`SVIX_WEBHOOK_SECRET` env var, or from the `webhookSecret` option passed to
`new Svix(...)`.

```ts
// convex/http.ts
import { httpRouter } from "convex/server";
import { internal } from "./_generated/api";
import { svix } from "./orders";

const http = httpRouter();

http.route({
  path: "/webhooks/svix",
  method: "POST",
  handler: svix.webhookHandler(async (ctx, event) => {
    // event.id is the same across retries of a message, so use it to dedupe.
    await ctx.runMutation(internal.webhooks.handle, {
      svixId: event.id,
      // `event.payload` is the parsed body. Pass the raw body when storing
      // it, as JSON objects can have keys Convex values don't allow.
      body: event.body,
    });
  }),
});

export default http;
```

```sh
npx convex env set SVIX_WEBHOOK_SECRET whsec_...
```

The route responds with `400` if the signature is invalid, `500` if your handler
throws (so the webhook is retried), and `204` otherwise.

## Testing

To use the component with
[`convex-test`](https://docs.convex.dev/testing/convex-test), register it on
your test instance:

```ts
import { convexTest } from "convex-test";
import svixTest from "@svix/convex/test";
import schema from "./schema";

const modules = import.meta.glob("./**/*.ts");

const t = convexTest(schema, modules);
svixTest.register(t);
```

Set `process.env.SVIX_API_KEY` in your tests and stub `fetch` to avoid calling
the Svix API.

## Development

```sh
npm install
npm run dev     # runs the example app in example/ against a Convex deployment
npm test
```

`npm run dev` regenerates the component's code and rebuilds the package when
`src/` changes. Set `SVIX_API_KEY` (and optionally `SVIX_WEBHOOK_SECRET`) on the
deployment with `npx convex env set`.
