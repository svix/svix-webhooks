import { defineSchema, defineTable } from "convex/server";
import { v } from "convex/values";

export default defineSchema({
  orders: defineTable({
    customerId: v.string(),
    amount: v.number(),
  }),
  receivedWebhooks: defineTable({
    svixId: v.string(),
    // The raw JSON body. Webhook payloads can contain keys that Convex values
    // don't allow (e.g. starting with `$`), so they're stored as a string.
    body: v.string(),
  }).index("svixId", ["svixId"]),
});
