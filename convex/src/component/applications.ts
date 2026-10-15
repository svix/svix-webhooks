import { v } from "convex/values";
import { action } from "./_generated/server.js";
import type { AppPortalCapability } from "svix";
import { vApplicationIn } from "./schema.js";
import { svixClient } from "./svix.js";

const vAppPortalCapability = v.union(
  // Basic read-only access, without the endpoint secret.
  v.literal("ViewBase"),
  v.literal("ViewEndpointSecret"),
  v.literal("ManageEndpointSecret"),
  v.literal("ManageTransformations"),
  // Replay missing messages and send example messages.
  v.literal("CreateAttempts"),
  // Manage endpoints, including their secrets.
  v.literal("ManageEndpoint"),
);

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
    capabilities: v.optional(v.array(vAppPortalCapability)),
    // Creates the application if it doesn't exist yet.
    application: v.optional(vApplicationIn),
  },
  returns: v.object({ url: v.string(), token: v.string() }),
  handler: async (_ctx, { appId, capabilities, ...args }) => {
    const out = await svixClient().authentication.appPortalAccess(appId, {
      ...args,
      // The literals match `AppPortalCapability`'s string values.
      capabilities: capabilities as AppPortalCapability[] | undefined,
    });
    return { url: out.url, token: out.token };
  },
});
