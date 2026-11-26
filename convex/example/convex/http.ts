import { httpRouter } from "convex/server";
import { internal } from "./_generated/api.js";
import { svix } from "./example.js";

const http = httpRouter();

// Receive webhooks sent by Svix, e.g. from Svix Ingest or another Svix user.
// The signing secret is read from the SVIX_WEBHOOK_SECRET env var.
http.route({
  path: "/webhooks/svix",
  method: "POST",
  handler: svix.webhookHandler(async (ctx, event) => {
    await ctx.runMutation(internal.example.recordWebhook, {
      svixId: event.id,
      body: event.body,
    });
  }),
});

export default http;
