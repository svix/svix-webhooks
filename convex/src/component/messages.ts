import {
  NonRetryableError,
  vOnCompleteArgs,
  Workpool,
} from "@convex-dev/workpool";
import { v } from "convex/values";
import {
  internalAction,
  internalMutation,
  mutation,
  query,
  type MutationCtx,
  type QueryCtx,
  env,
} from "./_generated/server.js";
import { api, components, internal } from "./_generated/api.js";
import type { Doc, Id } from "./_generated/dataModel.js";
import schema, { vApplicationIn } from "./schema.js";
import { errorMessage, isConflict, isRetryable, svixClient } from "./svix.js";

export const MAX_ATTEMPTS = 8;
export const MAX_LIST_LIMIT = 100;
const CLEANUP_BATCH_SIZE = 100;
const RETRY_BATCH_SIZE = 100;
export const DEFAULT_RETENTION_DAYS = 30;
const DAY_MS = 24 * 60 * 60 * 1000;

// Retries failed attempts with exponential backoff (1s, 2s, 4s, ...), and
// reports attempts that crashed or timed out as failures, so every message
// eventually leaves `pending`.
const deliveryPool = new Workpool(components.deliveryPool, {
  maxParallelism: 10,
  retryActionsByDefault: true,
  defaultRetryBehavior: {
    maxAttempts: MAX_ATTEMPTS,
    initialBackoffMs: 1000,
    base: 2,
  },
});

const vMessageSummary = schema.tables.messages.validator
  .omit("payloadId")
  .extend({ _id: v.id("messages"), _creationTime: v.number() });
const vMessage = vMessageSummary.extend({ payload: v.string() });

/**
 * Enqueue a message to be sent to Svix.
 *
 * The message is stored in the same transaction as the calling mutation, so
 * it's only sent if that mutation commits. Delivery happens asynchronously
 * and is retried on transient errors.
 */
export const send = mutation({
  args: {
    appId: v.string(),
    eventType: v.string(),
    // JSON-encoded payload.
    payload: v.string(),
    eventId: v.optional(v.string()),
    channels: v.optional(v.array(v.string())),
    tags: v.optional(v.array(v.string())),
    application: v.optional(vApplicationIn),
  },
  returns: v.id("messages"),
  handler: async (ctx, { payload, channels, tags, ...args }) => {
    // Svix rejects anything but an object, but only once delivery is attempted,
    // so fail fast instead.
    if (!isJsonObject(payload)) {
      throw new Error("`payload` must be a JSON-encoded object");
    }
    const payloadId = await ctx.db.insert("payloads", { payload });
    const messageId = await ctx.db.insert("messages", {
      ...args,
      // Svix rejects empty lists, and they mean the same as none.
      channels: channels?.length ? channels : undefined,
      tags: tags?.length ? tags : undefined,
      payloadId,
      status: "pending",
      attempts: 0,
    });
    await enqueueDelivery(ctx, messageId);
    return messageId;
  },
});

/** Returns `null` if the message doesn't exist or the ID is malformed. */
export const get = query({
  args: { messageId: v.string() },
  returns: v.union(v.null(), vMessage),
  handler: async (ctx, args) => {
    const messageId = ctx.db.normalizeId("messages", args.messageId);
    const message = messageId && (await ctx.db.get("messages", messageId));
    return message && (await withPayload(ctx, message));
  },
});

/**
 * The most recent messages for an application, at most `MAX_LIST_LIMIT`.
 * Payloads aren't included, as they can be large.
 */
export const list = query({
  args: { appId: v.string(), limit: v.optional(v.number()) },
  returns: v.array(vMessageSummary),
  handler: async (ctx, args) => {
    const limit = clamp(
      Math.floor(args.limit ?? MAX_LIST_LIMIT),
      1,
      MAX_LIST_LIMIT,
    );
    const messages = await ctx.db
      .query("messages")
      .withIndex("by_appId", (q) => q.eq("appId", args.appId))
      .order("desc")
      .take(limit);
    return messages.map(summarize);
  },
});

/**
 * Delete sent and failed messages older than `SVIX_MESSAGE_RETENTION_DAYS`.
 * Runs daily from a cron.
 */
export const deleteExpired = internalMutation({
  args: {},
  returns: v.null(),
  handler: async (ctx) => {
    const retentionDays = parseRetentionDays(env.SVIX_MESSAGE_RETENTION_DAYS);
    if (retentionDays === 0) {
      return null;
    }
    const { hasMore } = await deleteMessagesBefore(
      ctx,
      Date.now() - retentionDays * DAY_MS,
      CLEANUP_BATCH_SIZE,
    );
    if (hasMore) {
      await ctx.scheduler.runAfter(0, internal.messages.deleteExpired, {});
    }
    return null;
  },
});

export function parseRetentionDays(value: string | undefined): number {
  if (value === undefined || value.trim() === "") {
    return DEFAULT_RETENTION_DAYS;
  }
  const days = Number(value);
  if (!Number.isFinite(days) || days < 0) {
    throw new Error(
      `Invalid SVIX_MESSAGE_RETENTION_DAYS ${JSON.stringify(value)}: must be a non-negative number of days`,
    );
  }
  return days;
}

/**
 * Count a delivery attempt, so attempts that crash or time out are counted
 * too. Returns `null` if the message is no longer pending.
 */
export const startAttempt = internalMutation({
  args: { messageId: v.id("messages") },
  returns: v.union(v.null(), vMessage),
  handler: async (ctx, args) => {
    const message = await ctx.db.get("messages", args.messageId);
    if (message?.status !== "pending") {
      return null;
    }
    const attempts = message.attempts + 1;
    await ctx.db.patch("messages", message._id, { attempts });
    return await withPayload(ctx, { ...message, attempts });
  },
});

export const deliver = internalAction({
  args: { messageId: v.id("messages") },
  returns: v.null(),
  handler: async (ctx, args) => {
    const message = await ctx.runMutation(internal.messages.startAttempt, args);
    if (message === null) {
      return null;
    }
    let svixMessageId: string | undefined;
    try {
      const out = await svixClient().message.create(
        message.appId,
        {
          eventType: message.eventType,
          payload: JSON.parse(message.payload),
          eventId: message.eventId,
          channels: message.channels,
          tags: message.tags,
          application: message.application,
        },
        // Makes retries safe: if a previous attempt reached Svix but its
        // outcome was never recorded, Svix returns the original response
        // instead of creating a duplicate.
        { idempotencyKey: `convex-${message._id}` },
      );
      svixMessageId = out.id;
    } catch (error) {
      // A conflict means Svix already has a message with this `eventId`, e.g.
      // from an earlier `send` with the same `eventId`, so the event was sent.
      if (!isConflict(error)) {
        const reason = errorMessage(error);
        await ctx.runMutation(internal.messages.recordFailedAttempt, {
          messageId: message._id,
          error: reason,
        });
        throw isRetryable(error)
          ? new Error(reason)
          : new NonRetryableError(reason);
      }
    }
    await ctx.runMutation(internal.messages.markSent, {
      messageId: message._id,
      svixMessageId,
    });
    return null;
  },
});

export const markSent = internalMutation({
  args: {
    messageId: v.id("messages"),
    svixMessageId: v.optional(v.string()),
  },
  returns: v.null(),
  handler: async (ctx, args) => {
    const message = await ctx.db.get("messages", args.messageId);
    if (message === null) {
      return null;
    }
    await ctx.db.patch("messages", args.messageId, {
      status: "sent",
      svixMessageId: args.svixMessageId,
      lastError: undefined,
    });
    return null;
  },
});

export const recordFailedAttempt = internalMutation({
  args: { messageId: v.id("messages"), error: v.string() },
  returns: v.null(),
  handler: async (ctx, args) => {
    const message = await ctx.db.get("messages", args.messageId);
    if (message === null) {
      return null;
    }
    await ctx.db.patch("messages", args.messageId, { lastError: args.error });
    return null;
  },
});

/**
 * Called by the workpool once delivery gave up: after the last attempt failed,
 * on a non-retryable error, or if the attempt crashed or timed out.
 */
export const onDeliveryFailed = internalMutation({
  args: vOnCompleteArgs(v.object({ messageId: v.id("messages") })),
  returns: v.null(),
  handler: async (ctx, { context, result }) => {
    const message = await ctx.db.get("messages", context.messageId);
    if (message?.status !== "pending") {
      return null;
    }
    await ctx.db.patch("messages", message._id, {
      status: "failed",
      lastError:
        result.kind === "failed" ? result.error : "Delivery was canceled",
    });
    return null;
  },
});

/**
 * Retry delivery of a message that permanently failed. Returns `false`, and
 * does nothing, if the message isn't `failed`.
 */
export const retry = mutation({
  args: { messageId: v.string() },
  returns: v.boolean(),
  handler: async (ctx, args) => {
    const messageId = ctx.db.normalizeId("messages", args.messageId);
    const message = messageId && (await ctx.db.get("messages", messageId));
    if (!message) {
      throw new Error(`Message ${args.messageId} not found`);
    }
    if (message.status !== "failed") {
      return false;
    }
    await requeueFailed(ctx, message._id);
    return true;
  },
});

/**
 * Retry delivery of all failed messages, or only those of `appId`, e.g. after
 * fixing an invalid API key. Large backlogs are retried in the background, in
 * batches.
 */
export const retryFailed = mutation({
  args: {
    appId: v.optional(v.string()),
    // Only retries messages created after this, so messages that fail again
    // while a backlog is being retried aren't retried twice.
    after: v.optional(v.number()),
  },
  returns: v.null(),
  handler: async (ctx, { appId, after = 0 }) => {
    const query =
      appId === undefined
        ? ctx.db
            .query("messages")
            .withIndex("by_status", (q) =>
              q.eq("status", "failed").gt("_creationTime", after),
            )
        : ctx.db
            .query("messages")
            .withIndex("by_appId_status", (q) =>
              q
                .eq("appId", appId)
                .eq("status", "failed")
                .gt("_creationTime", after),
            );
    const failed = await query.take(RETRY_BATCH_SIZE);
    for (const message of failed) {
      await requeueFailed(ctx, message._id);
    }
    if (failed.length === RETRY_BATCH_SIZE) {
      await ctx.scheduler.runAfter(0, api.messages.retryFailed, {
        appId,
        after: failed[failed.length - 1]._creationTime,
      });
    }
    return null;
  },
});

async function deleteMessagesBefore(
  ctx: MutationCtx,
  before: number,
  batchSize: number,
) {
  let deleted = 0;
  let hasMore = false;
  for (const status of ["sent", "failed"] as const) {
    const docs = await ctx.db
      .query("messages")
      .withIndex("by_status", (q) =>
        q.eq("status", status).lt("_creationTime", before),
      )
      .take(batchSize - deleted + 1);
    hasMore ||= docs.length > batchSize - deleted;
    for (const doc of docs.slice(0, batchSize - deleted)) {
      await ctx.db.delete("messages", doc._id);
      await ctx.db.delete("payloads", doc.payloadId);
      deleted++;
    }
  }
  return { deleted, hasMore };
}

async function requeueFailed(ctx: MutationCtx, messageId: Id<"messages">) {
  await ctx.db.patch("messages", messageId, { status: "pending", attempts: 0 });
  await enqueueDelivery(ctx, messageId);
}

async function enqueueDelivery(ctx: MutationCtx, messageId: Id<"messages">) {
  await deliveryPool.enqueueAction(
    ctx,
    internal.messages.deliver,
    { messageId },
    {
      onComplete: internal.messages.onDeliveryFailed,
      onCompleteExcludeKinds: ["success"],
      context: { messageId },
    },
  );
}

function summarize({ payloadId: _payloadId, ...message }: Doc<"messages">) {
  return message;
}

async function withPayload(ctx: QueryCtx, message: Doc<"messages">) {
  const { payload } = (await ctx.db.get("payloads", message.payloadId))!;
  return { ...summarize(message), payload };
}

function isJsonObject(json: string) {
  try {
    const value: unknown = JSON.parse(json);
    return typeof value === "object" && value !== null && !Array.isArray(value);
  } catch {
    return false;
  }
}

function clamp(value: number, min: number, max: number) {
  return Math.min(Math.max(value, min), max);
}
