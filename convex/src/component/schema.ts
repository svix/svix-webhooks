import { defineSchema, defineTable } from "convex/server";
import { v } from "convex/values";

export const vMessageStatus = v.union(
  v.literal("pending"),
  v.literal("sent"),
  v.literal("failed"),
);

export const vApplicationIn = v.object({
  name: v.string(),
  uid: v.optional(v.string()),
});

export default defineSchema({
  // An outbox of messages to be sent to Svix. Rows are inserted
  // transactionally by `messages.send` and delivered by the workpool.
  messages: defineTable({
    // The Svix application ID or UID to send the message to.
    appId: v.string(),
    eventType: v.string(),
    // Stored in its own table so that queries over many messages don't read
    // every payload.
    payloadId: v.id("payloads"),
    eventId: v.optional(v.string()),
    channels: v.optional(v.array(v.string())),
    tags: v.optional(v.array(v.string())),
    // If set, the application is created alongside the message if it
    // doesn't exist yet.
    application: v.optional(vApplicationIn),
    status: vMessageStatus,
    attempts: v.number(),
    // The ID Svix assigned to the message, once it was sent. Unset if Svix
    // already had a message with the same `eventId`.
    svixMessageId: v.optional(v.string()),
    lastError: v.optional(v.string()),
  })
    .index("by_appId", ["appId"])
    .index("by_appId_status", ["appId", "status"])
    .index("by_status", ["status"]),
  payloads: defineTable({
    // JSON-encoded, as Convex values can't hold every JSON object (e.g. ones
    // with keys starting with `$`).
    payload: v.string(),
  }),
});
