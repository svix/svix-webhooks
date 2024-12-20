// this file is @generated (with minor manual changes)
import {
  Configuration,
  MessageApi,
  ListResponseMessageOut,
  MessageIn,
  MessageOut,
} from "../openapi";
import { PostOptions } from "../util";

export interface MessageListOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** Filter response based on the channel. */
  channel?: string;
  /** Only include items created before a certain date. */
  before?: Date | null;
  /** Only include items created after a certain date. */
  after?: Date | null;
  /** When `true` message payloads are included in the response. */
  withContent?: boolean;
  /** Filter messages matching the provided tag. */
  tag?: string;
  /** Filter response based on the event type */
  eventTypes?: string[];
}

export class Message {
  private readonly api: MessageApi;

  public constructor(config: Configuration) {
    this.api = new MessageApi(config);
  }

  /**
   * List all of the application's messages.
   *
   * The `before` and `after` parameters let you filter all items created before or after a certain date. These can be used alongside an iterator to paginate over results
   * within a certain window.
   *
   * Note that by default this endpoint is limited to retrieving 90 days' worth of data
   * relative to now or, if an iterator is provided, 90 days before/after the time indicated
   * by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
   * set the `before` or `after` parameter as appropriate.
   */
  public list(
    appId: string,
    options?: MessageListOptions
  ): Promise<ListResponseMessageOut> {
    return this.api.v1MessageList({
      appId,
      ...options,
      iterator: options?.iterator ?? undefined,
      before: options?.before ?? undefined,
      after: options?.after ?? undefined,
    });
  }

  /**
   * Creates a new message and dispatches it to all of the application's endpoints.
   *
   * The `eventId` is an optional custom unique ID. It's verified to be unique only up to a day, after that no verification will be made.
   * If a message with the same `eventId` already exists for the application, a 409 conflict error will be returned.
   *
   * The `eventType` indicates the type and schema of the event. All messages of a certain `eventType` are expected to have the same schema. Endpoints can choose to only listen to specific event types.
   * Messages can also have `channels`, which similar to event types let endpoints filter by them. Unlike event types, messages can have multiple channels, and channels don't imply a specific message content or schema.
   *
   * The `payload` property is the webhook's body (the actual webhook message). Svix supports payload sizes of up to ~350kb, though it's generally a good idea to keep webhook payloads small, probably no larger than 40kb.
   */
  public create(
    appId: string,
    messageIn: MessageIn,
    options?: PostOptions
  ): Promise<MessageOut> {
    return this.api.v1MessageCreate({
      appId,
      messageIn,
      ...options,
    });
  }

  /** Get a message by its ID or eventID. */
  public get(appId: string, msgId: string): Promise<MessageOut> {
    return this.api.v1MessageGet({
      appId,
      msgId,
    });
  }

  /**
   * Delete the given message's payload.
   *
   * Useful in cases when a message was accidentally sent with sensitive content.
   * The message can't be replayed or resent once its payload has been deleted or expired.
   */
  public expungeContent(appId: string, msgId: string): Promise<void> {
    return this.api.v1MessageExpungeContent({
      appId,
      msgId,
    });
  }
}

/**
 * Creates a `MessageIn` with a raw string payload.
 *
 * The payload is not normalized on the server. Normally, payloads are
 * required to be JSON, and Svix will minify the payload before sending the
 * webhooks (for example, by removing extraneous whitespace or unnecessarily
 * escaped characters in strings). With this function, the payload will be
 * sent "as is", without any minification or other processing.
 *
 * @param payload Serialized message payload
 * @param contentType The value to use for the Content-Type header of the webhook sent by Svix, overwriting the default of `application/json` if specified
 *
 * See the class documentation for details about the other parameters.
 */
export function messageInRaw(
  eventType: string,
  payload: string,
  contentType?: string
): MessageIn {
  const headers = contentType ? { "content-type": contentType } : undefined;

  return {
    eventType,
    payload: {},
    transformationsParams: {
      rawPayload: payload,
      headers,
    },
  };
}
