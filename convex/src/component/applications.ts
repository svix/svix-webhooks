import { v } from "convex/values";
import { action } from "./_generated/server.js";
import type { AppPortalCapability } from "svix";
import { svixClient } from "./svix.js";

const vApplication = v.object({
  id: v.string(),
  uid: v.optional(v.string()),
  name: v.string(),
});

/**
 * Get the application with the given UID, creating it if it doesn't exist.
 * Without a UID, a new application is always created.
 */
export const getOrCreate = action({
  args: {
    name: v.string(),
    uid: v.optional(v.string()),
    metadata: v.optional(v.record(v.string(), v.string())),
  },
  returns: vApplication,
  handler: async (_ctx, args) => {
    const app = await svixClient().application.getOrCreate(args);
    return { id: app.id, uid: app.uid ?? undefined, name: app.name };
  },
});

export const remove = action({
  args: { appId: v.string() },
  returns: v.null(),
  handler: async (_ctx, args) => {
    await svixClient().application.delete(args.appId);
    return null;
  },
});

/**
 * Get a short-lived URL to the Svix App Portal, where the application's
 * owner can manage their endpoints and see delivery attempts.
 */
export const portalAccess = action({
  args: {
    appId: v.string(),
    // Session lifetime in seconds.
    expiry: v.optional(v.number()),
    readOnly: v.optional(v.boolean()),
    featureFlags: v.optional(v.array(v.string())),
    capabilities: v.optional(
      v.array(
        v.union(
          v.literal("ViewBase"),
          v.literal("ViewEndpointSecret"),
          v.literal("ManageEndpointSecret"),
          v.literal("ManageTransformations"),
          v.literal("CreateAttempts"),
          v.literal("ManageEndpoint"),
        ),
      ),
    ),
    // Creates the application if it doesn't exist yet.
    application: v.optional(
      v.object({ name: v.string(), uid: v.optional(v.string()) }),
    ),
  },
  returns: v.object({ url: v.string(), token: v.string() }),
  handler: async (_ctx, { appId, capabilities, ...args }) => {
    const out = await svixClient().authentication.appPortalAccess(appId, {
      ...args,
      capabilities: capabilities as AppPortalCapability[] | undefined,
    });
    return { url: out.url, token: out.token };
  },
});
