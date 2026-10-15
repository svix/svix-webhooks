import { httpActionGeneric } from "convex/server";
import type {
  FunctionArgs,
  FunctionReturnType,
  GenericActionCtx,
  GenericDataModel,
  GenericMutationCtx,
  GenericQueryCtx,
} from "convex/server";
import { Webhook, WebhookVerificationError } from "svix";
import type { ComponentApi } from "../component/_generated/component.js";

export type SendMessageArgs = Omit<
  FunctionArgs<ComponentApi["messages"]["send"]>,
  "payload"
> & {
  /** The message's payload. Must be JSON-serializable. */
  payload: object;
};
type StoredMessage = NonNullable<
  FunctionReturnType<ComponentApi["messages"]["get"]>
>;
export type Message = Omit<StoredMessage, "payload"> & { payload: unknown };
/** A message as returned by {@link Svix.listMessages}, without its payload. */
export type MessageSummary = FunctionReturnType<
  ComponentApi["messages"]["list"]
>[number];
export type MessageStatus = Message["status"];
export type Application = FunctionReturnType<
  ComponentApi["applications"]["getOrCreate"]
>;
export type PortalAccessArgs = FunctionArgs<
  ComponentApi["applications"]["portalAccess"]
>;

/** A verified webhook received from Svix. */
export type WebhookEvent = {
  /** The `svix-id` header. Unique per message, so it can be used to dedupe. */
  id: string;
  /** The `svix-timestamp` header, in seconds. */
  timestamp: number;
  /** The parsed JSON body of the webhook, or `null` if it isn't JSON. */
  payload: unknown;
  /** The raw request body, as signed. */
  body: string;
  headers: Headers;
};

export type SvixOptions = {
  /**
   * The signing secret (`whsec_...`) used to verify incoming webhooks.
   * Defaults to the `SVIX_WEBHOOK_SECRET` environment variable.
   */
  webhookSecret?: string;
};

/**
 * Client for the Svix component.
 *
 * ```ts
 * const svix = new Svix(components.svix);
 * ```
 */
export class Svix {
  constructor(
    public component: ComponentApi,
    private options: SvixOptions = {},
  ) {}

  /**
   * Send a webhook message.
   *
   * When called from a mutation, the message is only sent if the mutation
   * commits. Delivery to Svix happens in the background and is retried on
   * transient errors. Returns the ID of the message in the component, which
   * can be passed to {@link Svix.getMessage} to track its status.
   */
  async sendMessage(ctx: RunMutationCtx, args: SendMessageArgs) {
    return await ctx.runMutation(this.component.messages.send, {
      ...args,
      // Convex values can't hold every JSON object, so it's stored encoded.
      payload: JSON.stringify(args.payload),
    });
  }

  /** Returns `null` if there's no message with this ID. */
  async getMessage(
    ctx: RunQueryCtx,
    messageId: string,
  ): Promise<Message | null> {
    const message = await ctx.runQuery(this.component.messages.get, {
      messageId,
    });
    return message && decodeMessage(message);
  }

  /**
   * The most recent messages sent to an application, at most 100. Payloads
   * aren't included; use {@link Svix.getMessage} to get one.
   */
  async listMessages(
    ctx: RunQueryCtx,
    args: { appId: string; limit?: number },
  ): Promise<MessageSummary[]> {
    return await ctx.runQuery(this.component.messages.list, args);
  }

  /** Retry delivery of a message that permanently failed. */
  async retryMessage(ctx: RunMutationCtx, messageId: string) {
    return await ctx.runMutation(this.component.messages.retry, { messageId });
  }

  /**
   * Retry delivery of all failed messages, or only those of `appId`, e.g.
   * after fixing an invalid API key. Retries up to `batchSize` messages; call
   * again while `hasMore` is true.
   */
  async retryFailedMessages(
    ctx: RunMutationCtx,
    args: { appId?: string; batchSize?: number } = {},
  ) {
    return await ctx.runMutation(this.component.messages.retryFailed, args);
  }

  /**
   * Delete sent and failed messages created before `before` (ms since epoch).
   * Deletes up to `batchSize` (default 100, at most 1000) messages; call again
   * while `hasMore` is true.
   */
  async cleanupMessages(
    ctx: RunMutationCtx,
    args: { before: number; batchSize?: number },
  ) {
    return await ctx.runMutation(this.component.messages.cleanup, args);
  }

  /**
   * Get the application with the given UID, creating it if it doesn't exist.
   */
  async getOrCreateApplication(
    ctx: RunActionCtx,
    args: FunctionArgs<ComponentApi["applications"]["getOrCreate"]>,
  ) {
    return await ctx.runAction(this.component.applications.getOrCreate, args);
  }

  async deleteApplication(ctx: RunActionCtx, appId: string) {
    return await ctx.runAction(this.component.applications.remove, { appId });
  }

  /**
   * Get a short-lived, authenticated link to the Svix App Portal for an
   * application, where its owner can manage their endpoints.
   */
  async appPortalAccess(ctx: RunActionCtx, args: PortalAccessArgs) {
    return await ctx.runAction(this.component.applications.portalAccess, args);
  }

  /**
   * An HTTP action that verifies incoming Svix webhooks and passes them to
   * `handler`. Mount it in your app's `convex/http.ts`:
   *
   * ```ts
   * http.route({
   *   path: "/webhooks/svix",
   *   method: "POST",
   *   handler: svix.webhookHandler(async (ctx, event) => { ... }),
   * });
   * ```
   *
   * Responds with 400 if the signature is invalid, 500 if `handler` throws
   * (so Svix retries the webhook), and 204 otherwise.
   */
  webhookHandler<DataModel extends GenericDataModel = GenericDataModel>(
    handler: (
      ctx: GenericActionCtx<DataModel>,
      event: WebhookEvent,
    ) => Promise<void>,
  ) {
    return httpActionGeneric(async (ctx, request) => {
      const secret =
        this.options.webhookSecret ?? process.env.SVIX_WEBHOOK_SECRET;
      if (!secret) {
        throw new Error(
          "Missing webhook secret: set SVIX_WEBHOOK_SECRET or pass `webhookSecret`",
        );
      }
      const body = await request.text();
      try {
        new Webhook(secret).verify(
          body,
          Object.fromEntries(request.headers.entries()),
        );
      } catch (error) {
        if (error instanceof WebhookVerificationError) {
          return new Response(error.message, { status: 400 });
        }
        throw error;
      }
      let payload: unknown = null;
      try {
        payload = JSON.parse(body);
      } catch {
        // Not JSON; the handler can use `event.body`.
      }
      const header = (name: string) =>
        request.headers.get(`svix-${name}`) ??
        request.headers.get(`webhook-${name}`)!;
      await handler(ctx as unknown as GenericActionCtx<DataModel>, {
        id: header("id"),
        timestamp: Number(header("timestamp")),
        payload,
        body,
        headers: request.headers,
      });
      return new Response(null, { status: 204 });
    });
  }
}

function decodeMessage(message: StoredMessage): Message {
  return { ...message, payload: JSON.parse(message.payload) };
}

// Minimal `ctx` types, so these methods can be called from any function type
// that supports them.
type RunQueryCtx = Pick<GenericQueryCtx<GenericDataModel>, "runQuery">;
type RunMutationCtx = Pick<GenericMutationCtx<GenericDataModel>, "runMutation">;
type RunActionCtx = Pick<GenericActionCtx<GenericDataModel>, "runAction">;
