import { v } from "convex/values";
import {
  internalAction,
  internalMutation,
  internalQuery,
  mutation,
  query,
  type MutationCtx,
  type QueryCtx,
  env,
} from "./_generated/server.js";
import { internal } from "./_generated/api.js";
import type { Doc, Id } from "./_generated/dataModel.js";
import schema, { vApplicationIn } from "./schema.js";
import { errorMessage, isConflict, isRetryable, svixClient } from "./svix.js";

export const MAX_ATTEMPTS = 8;
const BASE_BACKOFF_MS = 1000;
export const MAX_BACKOFF_MS = 5 * 60 * 1000;
const REQUEUE_BATCH_SIZE = 100;
// How long after an attempt is due it may take to record its outcome before
// `requeueStuck` considers it stuck. Actions time out after 10 minutes.
export const DELIVERY_DEADLINE_MS = 15 * 60 * 1000;
export const MAX_LIST_LIMIT = 100;
const DEFAULT_CLEANUP_BATCH_SIZE = 100;
export const MAX_CLEANUP_BATCH_SIZE = 1000;
export const DEFAULT_RETENTION_DAYS = 30;
const DAY_MS = 24 * 60 * 60 * 1000;

const vMessageSummary = schema.tables.messages.validator
  .omit("payloadId", "deliverBy")
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
  handler: async (ctx, { payload, ...args }) => {
    try {
      JSON.parse(payload);
    } catch {
      throw new Error("`payload` must be a JSON-encoded string");
    }
    const payloadId = await ctx.db.insert("payloads", { payload });
    const messageId = await ctx.db.insert("messages", {
      ...args,
      payloadId,
      status: "pending",
      attempts: 0,
    });
    await scheduleDelivery(ctx, messageId, 0);
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
    const limit = clamp(args.limit ?? MAX_LIST_LIMIT, 1, MAX_LIST_LIMIT);
    const messages = await ctx.db
      .query("messages")
      .withIndex("by_appId", (q) => q.eq("appId", args.appId))
      .order("desc")
      .take(limit);
    return messages.map(summarize);
  },
});

/**
 * Delete messages that are no longer pending and older than `before`, up to
 * `batchSize` (at most `MAX_CLEANUP_BATCH_SIZE`) at a time.
 */
export const cleanup = mutation({
  args: { before: v.number(), batchSize: v.optional(v.number()) },
  returns: v.object({ deleted: v.number(), hasMore: v.boolean() }),
  handler: async (ctx, args) => {
    const batchSize = clamp(
      Math.floor(args.batchSize ?? DEFAULT_CLEANUP_BATCH_SIZE),
      1,
      MAX_CLEANUP_BATCH_SIZE,
    );
    return await deleteMessagesBefore(ctx, args.before, batchSize);
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
      DEFAULT_CLEANUP_BATCH_SIZE,
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

export const getPending = internalQuery({
  args: { messageId: v.id("messages") },
  returns: v.union(v.null(), vMessage),
  handler: async (ctx, args) => {
    const message = await ctx.db.get("messages", args.messageId);
    return message?.status === "pending"
      ? await withPayload(ctx, message)
      : null;
  },
});

export const deliver = internalAction({
  args: { messageId: v.id("messages") },
  returns: v.null(),
  handler: async (ctx, args) => {
    const message = await ctx.runQuery(internal.messages.getPending, args);
    if (message === null) {
      return null;
    }
    try {
      const out = await svixClient().message.create(
        message.appId,
        {
          eventType: message.eventType,
          payload: JSON.parse(message.payload),
          // Svix rejects a second message with the same `eventId`, so this
          // prevents duplicates even when retried after the idempotency key
          // expired.
          eventId: message.eventId ?? `convex-${message._id}`,
          channels: message.channels,
          tags: message.tags,
          application: message.application,
        },
        // If a previous attempt reached Svix but its outcome was never
        // recorded, lets a retry still get the Svix message ID.
        { idempotencyKey: `convex-${message._id}` },
      );
      await ctx.runMutation(internal.messages.markSent, {
        messageId: message._id,
        svixMessageId: out.id,
      });
    } catch (error) {
      if (isConflict(error)) {
        // Svix already has a message with this `eventId`, so the event was
        // already sent, e.g. by an earlier attempt or by an earlier `send`
        // with the same `eventId`.
        await ctx.runMutation(internal.messages.markSent, {
          messageId: message._id,
        });
        return null;
      }
      await ctx.runMutation(internal.messages.markFailedAttempt, {
        messageId: message._id,
        error: errorMessage(error),
        retryable: isRetryable(error),
      });
    }
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
      attempts: message.attempts + 1,
      svixMessageId: args.svixMessageId,
      lastError: undefined,
      deliveryJobId: undefined,
      deliverBy: undefined,
    });
    return null;
  },
});

export const markFailedAttempt = internalMutation({
  args: {
    messageId: v.id("messages"),
    error: v.string(),
    retryable: v.boolean(),
  },
  returns: v.null(),
  handler: async (ctx, args) => {
    const message = await ctx.db.get("messages", args.messageId);
    if (message === null) {
      return null;
    }
    await recordFailedAttempt(ctx, message, args.error, args.retryable);
    return null;
  },
});

/**
 * Re-queue pending messages whose delivery job finished without recording an
 * outcome, e.g. because the action crashed or timed out. Runs from a cron.
 *
 * Only looks at messages past their `deliverBy` deadline. Each one is either
 * re-queued or given a new deadline, so it drops out of the range.
 */
export const requeueStuck = internalMutation({
  args: {},
  returns: v.null(),
  handler: async (ctx) => {
    const now = Date.now();
    const overdue = await ctx.db
      .query("messages")
      .withIndex("by_status_deliverBy", (q) =>
        q.eq("status", "pending").lt("deliverBy", now),
      )
      .take(REQUEUE_BATCH_SIZE);
    for (const message of overdue) {
      const job =
        message.deliveryJobId &&
        (await ctx.db.system.get(
          "_scheduled_functions",
          message.deliveryJobId,
        ));
      if (job?.state.kind === "pending" || job?.state.kind === "inProgress") {
        // Running late, e.g. because the scheduler is backed up.
        await ctx.db.patch("messages", message._id, {
          deliverBy: now + DELIVERY_DEADLINE_MS,
        });
        continue;
      }
      const reason =
        job?.state.kind === "failed"
          ? `Delivery attempt failed: ${job.state.error}`
          : "Delivery attempt didn't complete";
      await recordFailedAttempt(ctx, message, reason, true);
    }
    if (overdue.length === REQUEUE_BATCH_SIZE) {
      await ctx.scheduler.runAfter(0, internal.messages.requeueStuck, {});
    }
    return null;
  },
});

/** Retry delivery of a message that permanently failed. */
export const retry = mutation({
  args: { messageId: v.string() },
  returns: v.null(),
  handler: async (ctx, args) => {
    const messageId = ctx.db.normalizeId("messages", args.messageId);
    const message = messageId && (await ctx.db.get("messages", messageId));
    if (!message) {
      throw new Error(`Message ${args.messageId} not found`);
    }
    if (message.status === "failed") {
      await requeueFailed(ctx, message._id);
    }
    return null;
  },
});

/**
 * Retry delivery of failed messages, e.g. after fixing an invalid API key.
 * Retries up to `batchSize` messages; call again while `hasMore` is true.
 */
export const retryFailed = mutation({
  args: { appId: v.optional(v.string()), batchSize: v.optional(v.number()) },
  returns: v.object({ retried: v.number(), hasMore: v.boolean() }),
  handler: async (ctx, args) => {
    const batchSize = Math.max(1, args.batchSize ?? 100);
    const query =
      args.appId === undefined
        ? ctx.db
            .query("messages")
            .withIndex("by_status", (q) => q.eq("status", "failed"))
        : ctx.db
            .query("messages")
            .withIndex("by_appId_status", (q) =>
              q.eq("appId", args.appId!).eq("status", "failed"),
            );
    const failed = await query.take(batchSize + 1);
    for (const message of failed.slice(0, batchSize)) {
      await requeueFailed(ctx, message._id);
    }
    return {
      retried: Math.min(failed.length, batchSize),
      hasMore: failed.length > batchSize,
    };
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
  await scheduleDelivery(ctx, messageId, 0);
}

async function scheduleDelivery(
  ctx: MutationCtx,
  messageId: Id<"messages">,
  delayMs: number,
) {
  const deliveryJobId = await ctx.scheduler.runAfter(
    delayMs,
    internal.messages.deliver,
    { messageId },
  );
  await ctx.db.patch("messages", messageId, {
    deliveryJobId,
    deliverBy: Date.now() + delayMs + DELIVERY_DEADLINE_MS,
  });
}

async function recordFailedAttempt(
  ctx: MutationCtx,
  message: Doc<"messages">,
  error: string,
  retryable: boolean,
) {
  const attempts = message.attempts + 1;
  const giveUp = !retryable || attempts >= MAX_ATTEMPTS;
  await ctx.db.patch("messages", message._id, {
    status: giveUp ? "failed" : "pending",
    attempts,
    lastError: error,
    deliveryJobId: undefined,
    deliverBy: undefined,
  });
  if (!giveUp) {
    const backoff = Math.min(
      BASE_BACKOFF_MS * 2 ** (attempts - 1),
      MAX_BACKOFF_MS,
    );
    const jitter = Math.random() * backoff * 0.2;
    await scheduleDelivery(ctx, message._id, backoff + jitter);
  }
}

function summarize({
  payloadId: _payloadId,
  deliverBy: _deliverBy,
  ...message
}: Doc<"messages">) {
  return message;
}

async function withPayload(ctx: QueryCtx, message: Doc<"messages">) {
  const { payload } = (await ctx.db.get("payloads", message.payloadId))!;
  return { ...summarize(message), payload };
}

function clamp(value: number, min: number, max: number) {
  return Math.min(Math.max(value, min), max);
}
