import { Svix } from "@svix/convex";
import { v } from "convex/values";
import { components } from "./_generated/api.js";
import {
  action,
  internalMutation,
  mutation,
  query,
} from "./_generated/server.js";

export const svix = new Svix(components.svix);

// Each of your customers gets their own Svix application. Here we use the
// customer ID as the application's UID, so we never need to store the Svix ID.
export const createOrder = mutation({
  args: { customerId: v.string(), amount: v.number() },
  returns: v.object({ orderId: v.id("orders"), messageId: v.string() }),
  handler: async (ctx, args) => {
    const orderId = await ctx.db.insert("orders", args);
    // Only sent if this mutation commits.
    const messageId = await svix.sendMessage(ctx, {
      appId: args.customerId,
      eventType: "order.created",
      payload: { orderId, amount: args.amount },
      // Create the application on the first message to this customer.
      application: {
        name: `Customer ${args.customerId}`,
        uid: args.customerId,
      },
    });
    return { orderId, messageId };
  },
});

export const messageStatus = query({
  args: { messageId: v.string() },
  handler: async (ctx, args) => {
    const message = await svix.getMessage(ctx, args.messageId);
    return message && { status: message.status, error: message.lastError };
  },
});

export const listMessages = query({
  args: { customerId: v.string() },
  handler: async (ctx, args) => {
    return await svix.listMessages(ctx, { appId: args.customerId, limit: 20 });
  },
});

// Lets a customer manage their webhook endpoints. In a real app, check that
// the authenticated user owns `customerId`.
export const portalUrl = action({
  args: { customerId: v.string() },
  returns: v.string(),
  handler: async (ctx, args) => {
    const { url } = await svix.appPortalAccess(ctx, {
      appId: args.customerId,
      application: {
        name: `Customer ${args.customerId}`,
        uid: args.customerId,
      },
    });
    return url;
  },
});

export const recordWebhook = internalMutation({
  args: { svixId: v.string(), body: v.string() },
  returns: v.null(),
  handler: async (ctx, args) => {
    const existing = await ctx.db
      .query("receivedWebhooks")
      .withIndex("svixId", (q) => q.eq("svixId", args.svixId))
      .unique();
    if (existing === null) {
      await ctx.db.insert("receivedWebhooks", args);
    }
    return null;
  },
});
