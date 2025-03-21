// this file is @generated
import { MessageEventsOut, MessageEventsOutSerializer } from "../models/messageEventsOut";
import { MessageIn } from "../models/messageIn";
import {
  MessagePrecheckIn,
  MessagePrecheckInSerializer,
} from "../models/messagePrecheckIn";
import {
  MessagePrecheckOut,
  MessagePrecheckOutSerializer,
} from "../models/messagePrecheckOut";
import {
  MessageRawPayloadOut,
  MessageRawPayloadOutSerializer,
} from "../models/messageRawPayloadOut";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";
import { MessageEventsSubscription } from "./messageEventsSubscription";

export interface MessageEventsOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** Filter response based on the event type */
  eventTypes?: string[];
  /** Filter response based on the event type. */
  channels?: string[];
  after?: Date | null;
}

export interface MessageEventsSubscriptionOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** Filter response based on the event type */
  eventTypes?: string[];
  /** Filter response based on the event type. */
  channels?: string[];
  after?: Date | null;
}

export interface MessagePrecheckOptions {
  idempotencyKey?: string;
}

export class Message {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  public get events_subscription() {
    return new MessageEventsSubscription(this.requestCtx);
  }

  /** Reads the stream of created messages for an application. */
  public events(
    appId: string,
    options?: MessageEventsOptions
  ): Promise<MessageEventsOut> {
    const request = new SvixRequest(HttpMethod.GET, "/api/v1/app/{app_id}/events");

    request.setPathParam("app_id", appId);
    request.setQueryParam("limit", options?.limit);
    request.setQueryParam("iterator", options?.iterator);
    request.setQueryParam("event_types", options?.eventTypes);
    request.setQueryParam("channels", options?.channels);
    request.setQueryParam("after", options?.after);

    return request.send(this.requestCtx, MessageEventsOutSerializer._fromJsonObject);
  }

  /** Reads the stream of created messages for an application, but using server-managed iterator tracking. */
  public eventsSubscription(
    appId: string,
    subscriptionId: string,
    options?: MessageEventsSubscriptionOptions
  ): Promise<MessageEventsOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/app/{app_id}/events/subscription/{subscription_id}"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("subscription_id", subscriptionId);
    request.setQueryParam("limit", options?.limit);
    request.setQueryParam("iterator", options?.iterator);
    request.setQueryParam("event_types", options?.eventTypes);
    request.setQueryParam("channels", options?.channels);
    request.setQueryParam("after", options?.after);

    return request.send(this.requestCtx, MessageEventsOutSerializer._fromJsonObject);
  }

  /**
   * A pre-check call for `create.message` that checks whether endpoints are actively listening to
   * this message.
   */
  public precheck(
    appId: string,
    messagePrecheckIn: MessagePrecheckIn,
    options?: MessagePrecheckOptions
  ): Promise<MessagePrecheckOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/app/{app_id}/msg/precheck/active"
    );

    request.setPathParam("app_id", appId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(MessagePrecheckInSerializer._toJsonObject(messagePrecheckIn));

    return request.send(this.requestCtx, MessagePrecheckOutSerializer._fromJsonObject);
  }

  /** Get a message raw payload by its ID or eventID. */
  public getRawPayload(appId: string, msgId: string): Promise<MessageRawPayloadOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/app/{app_id}/msg/{msg_id}/raw"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("msg_id", msgId);

    return request.send(this.requestCtx, MessageRawPayloadOutSerializer._fromJsonObject);
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
