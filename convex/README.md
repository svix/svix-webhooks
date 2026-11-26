# Svix Convex Component

Send and receive webhooks from your [Convex](https://convex.dev) app with
[Svix](https://www.svix.com).

- **Transactional sending.** Call `svix.sendMessage(ctx, ...)` from a mutation.
  The message is only sent if the mutation commits, so you never notify your
  customers about something that was rolled back.
- **Reliable delivery to Svix.** Messages are delivered in the background,
  retried with exponential backoff on transient errors, and sent with an
  idempotency key so retries never create duplicates.
- **Delivery status.** Query a message's status (`pending`, `sent` or `failed`)
  reactively from your app.
- **App Portal.** Generate links to the Svix App Portal, where your customers
  manage their own webhook endpoints.
- **Receiving webhooks.** A ready-made HTTP action that verifies Svix webhook
  signatures before handing the event to your code.

## Setup

1. Install the package:

   ```sh
   npm install @svix/convex
   ```

2. Install the component in `convex/convex.config.ts`, passing it your Svix API
   key. Declaring the key as an app env var and passing it by reference means
   the component always sees its current value.

   ```ts
   // convex/convex.config.ts
   import { defineApp } from "convex/server";
   import { v } from "convex/values";
   import svix from "@svix/convex/convex.config.js";

   const app = defineApp({
     env: {
       SVIX_API_KEY: v.string(),
       // Optional: only for self-hosted Svix or a specific region.
       SVIX_SERVER_URL: v.optional(v.string()),
     },
   });
   app.use(svix, {
     env: {
       SVIX_API_KEY: app.env.SVIX_API_KEY,
       SVIX_SERVER_URL: app.env.SVIX_SERVER_URL,
     },
   });

   export default app;
   ```

3. Create the client, and export it so the rest of your app can use it:

   ```ts
   // convex/svix.ts
   import { Svix } from "@svix/convex";
   import { components } from "./_generated/api";

   export const svix = new Svix(components.svix);
   ```

4. Set the env vars:

   ```sh
   npx convex env set SVIX_API_KEY sk_...
   # Only needed to receive webhooks:
   npx convex env set SVIX_WEBHOOK_SECRET whsec_...
   ```

### Configuration

| Env var                       | Passed to                                     | Purpose                                                                                                       |
| ----------------------------- | --------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| `SVIX_API_KEY`                | The component, via `app.use`                  | Required to send messages and manage applications.                                                            |
| `SVIX_SERVER_URL`             | The component, via `app.use`                  | Optional. For self-hosted Svix or a specific region. By default, the region is inferred from the API key.     |
| `SVIX_MESSAGE_RETENTION_DAYS` | The component, via `app.use`                  | Optional. How many days to keep sent and failed messages for (default `30`). See [Cleaning up](#cleaning-up). |
| `SVIX_WEBHOOK_SECRET`         | Nothing: read by `webhookHandler` in your app | Required to receive webhooks, unless you pass `webhookSecret` to `new Svix(...)`.                             |

The component can't read your app's env vars, so every variable it uses must be
declared in `defineApp` and passed in `app.use`, like `SVIX_API_KEY` above.
`SVIX_WEBHOOK_SECRET` is different: webhooks are verified in your app's HTTP
action, so it's read from your app's env directly.

## Sending webhooks

```ts
// convex/orders.ts
import { v } from "convex/values";
import { mutation } from "./_generated/server";
import { svix } from "./svix";

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
      // Optional: `eventId`, `channels` (up to 10) and `tags` (up to 5).
    });
    return orderId;
  },
});
```

`sendMessage` accepts `eventType`, `payload`, and optionally `eventId`,
`channels`, `tags` and `application`, which map to the
[Create Message](https://api.svix.com/docs#tag/Message/operation/v1.message.create)
API. It can also be called from actions.

The payload must be a JSON-serializable object (not an array or a primitive).
It's stored JSON-encoded, so keys that aren't valid in Convex values (such as
`$schema`) are fine, but the encoded payload must fit in a Convex document (1
MiB).

Every delivery attempt for a message uses the same idempotency key, so retries
never create duplicates in Svix. If you set an `eventId` and Svix has already
seen it in your environment, the message is marked as `sent` without a
`svixMessageId`, as the event was already delivered to Svix.

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
`getMessage` to get one. Messages are listed by the `appId` they were sent with,
so always refer to an application the same way (by its UID or by its `app_...`
ID, not both).

Delivery runs on a [workpool](https://www.convex.dev/components/workpool), at
most 10 messages at a time. Server errors, rate limiting, network errors and
crashed or timed out attempts are retried, up to 8 attempts in total, with
exponential backoff. Other errors (for example, a 4xx because the application
doesn't exist, or an invalid API key) fail the message immediately.

Once you've fixed the problem, use `svix.retryMessage(ctx, messageId)` to retry
a failed message (it returns `false` if the message isn't `failed`), or
`svix.retryFailedMessages(ctx, { appId })` to retry all failed messages of an
application (omit `appId` to retry all of them). Large backlogs are retried in
the background, in batches.

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

It must be a non-negative number of days. If it isn't, the daily cleanup fails
(you'll see the error in your Convex logs) and no messages are deleted.

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

`appPortalAccess` also takes `capabilities` (e.g. `["ViewBase"]` for read-only
access), `readOnly`, `expiry` (in seconds) and `featureFlags`.

For anything else, such as managing applications or endpoints, `svix.api()`
returns a [Svix JavaScript SDK](https://www.npmjs.com/package/svix) client
configured from your app's `SVIX_API_KEY` and `SVIX_SERVER_URL`. Like any call
to the Svix API, it can only be used from an action:

```ts
export const listEndpoints = action({
  args: { customerId: v.string() },
  handler: async (_ctx, args) => {
    const { data } = await svix.api().endpoint.list(args.customerId);
    return data.map((endpoint) => endpoint.url);
  },
});
```

## Receiving webhooks

`svix.webhookHandler` returns an HTTP action that verifies the webhook's
signature, then calls your handler. It reads the signing secret from your app's
`SVIX_WEBHOOK_SECRET` env var (no need to declare it in `defineApp`), or from
the `webhookSecret` option passed to `new Svix(...)`.

```ts
// convex/http.ts
import { httpRouter } from "convex/server";
import { internal } from "./_generated/api";
import { svix } from "./svix";

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

The route responds with `400` if the signature is invalid, `500` if your handler
throws (so the webhook is retried), and `204` otherwise.

## Rules of thumb

For people and coding agents using this component:

- Call `svix.sendMessage` from the **mutation** that makes the change you're
  notifying about, so the message is only sent if that mutation commits. Don't
  call it from an action unless there's no mutation involved.
- Use your own customer or tenant ID as the application `uid` and pass it as
  `appId`, with `application: { name, uid }` so the application is created on
  first use. Don't store Svix application IDs.
- `appPortalAccess` and `svix.api()` call the Svix API, so they can only be
  called from **actions**.
- The payload must be an object; `sendMessage` throws otherwise.
- Check the user owns the application before returning an App Portal URL.
- When receiving webhooks, dedupe on `event.id`, and store `event.body` (a
  string) rather than `event.payload`, which can have keys Convex doesn't allow.
- `getMessage` and `listMessages` are reactive: call them from a query to show
  delivery status in your UI.

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

This also registers the workpool the component uses for delivery. Set
`process.env.SVIX_API_KEY` in your tests and stub `fetch` to avoid calling the
Svix API.

## Development

```sh
npm install
npm run dev     # runs the example app in example/ against a Convex deployment
npm test
```

`npm run dev` regenerates the component's code and rebuilds the package when
`src/` changes. Set `SVIX_API_KEY` (and optionally `SVIX_WEBHOOK_SECRET`) on the
deployment with `npx convex env set`.
