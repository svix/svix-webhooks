// this file is @generated (with minor manual changes)
import {
  Configuration,
  MessageAttemptApi,
  ListResponseEndpointMessageOut,
  ListResponseMessageAttemptOut,
  ListResponseMessageEndpointOut,
  MessageAttemptOut,
  MessageStatus,
  StatusCodeClass,
} from "../openapi";
import { PostOptions } from "../util";

export interface MessageAttemptListOptions {
  iterator?: string | null;
  limit?: number;
  status?: MessageStatus;
  eventTypes?: string[];
  before?: Date | null;
  after?: Date | null;
  statusCodeClass?: StatusCodeClass;
  channel?: string;
  withContent?: boolean;
}

export interface MessageAttemptListByEndpointOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** Filter response based on the status of the attempt: Success (0), Pending (1), Failed (2), or Sending (3) */
  status?: MessageStatus;
  /** Filter response based on the HTTP status code */
  statusCodeClass?: StatusCodeClass;
  /** Filter response based on the channel */
  channel?: string;
  /** Filter response based on the tag */
  tag?: string;
  /** Only include items created before a certain date */
  before?: Date | null;
  /** Only include items created after a certain date */
  after?: Date | null;
  /** When `true` attempt content is included in the response */
  withContent?: boolean;
  /** When `true`, the message information is included in the response */
  withMsg?: boolean;
  /** Filter response based on the event type */
  eventTypes?: string[];
}

export interface MessageAttemptListByMsgOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** Filter response based on the status of the attempt: Success (0), Pending (1), Failed (2), or Sending (3) */
  status?: MessageStatus;
  /** Filter response based on the HTTP status code */
  statusCodeClass?: StatusCodeClass;
  /** Filter response based on the channel */
  channel?: string;
  /** Filter response based on the tag */
  tag?: string;
  /** Filter the attempts based on the attempted endpoint */
  endpointId?: string;
  /** Only include items created before a certain date */
  before?: Date | null;
  /** Only include items created after a certain date */
  after?: Date | null;
  /** When `true` attempt content is included in the response */
  withContent?: boolean;
  /** Filter response based on the event type */
  eventTypes?: string[];
}

export interface MessageAttemptListAttemptedMessagesOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** Filter response based on the channel */
  channel?: string;
  /** Filter response based on the message tags */
  tag?: string;
  /** Filter response based on the status of the attempt: Success (0), Pending (1), Failed (2), or Sending (3) */
  status?: MessageStatus;
  /** Only include items created before a certain date */
  before?: Date | null;
  /** Only include items created after a certain date */
  after?: Date | null;
  /** When `true` message payloads are included in the response */
  withContent?: boolean;
  /** Filter response based on the event type */
  eventTypes?: string[];
}

export interface MessageAttemptListAttemptedDestinationsOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
}

export class MessageAttempt {
  private readonly api: MessageAttemptApi;

  public constructor(config: Configuration) {
    this.api = new MessageAttemptApi(config);
  }

  /**
   * @deprecated Since version 0.48.0. Use listByMsg or listByEndpoint instead.
   */
  public list(
    appId: string,
    msgId: string,
    options?: MessageAttemptListByMsgOptions
  ): Promise<ListResponseMessageAttemptOut> {
    return this.listByMsg(appId, msgId, options);
  }

  /**
   * List attempts by endpoint id
   *
   * Note that by default this endpoint is limited to retrieving 90 days' worth of data
   * relative to now or, if an iterator is provided, 90 days before/after the time indicated
   * by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
   * set the `before` or `after` parameter as appropriate.
   */
  public listByEndpoint(
    appId: string,
    endpointId: string,
    options?: MessageAttemptListByEndpointOptions
  ): Promise<ListResponseMessageAttemptOut> {
    return this.api.v1MessageAttemptListByEndpoint({
      appId,
      endpointId,
      ...options,
      iterator: options?.iterator ?? undefined,
      before: options?.before ?? undefined,
      after: options?.after ?? undefined,
    });
  }

  /**
   * List attempts by message ID.
   *
   * Note that by default this endpoint is limited to retrieving 90 days' worth of data
   * relative to now or, if an iterator is provided, 90 days before/after the time indicated
   * by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
   * set the `before` or `after` parameter as appropriate.
   */
  public listByMsg(
    appId: string,
    msgId: string,
    options?: MessageAttemptListByMsgOptions
  ): Promise<ListResponseMessageAttemptOut> {
    return this.api.v1MessageAttemptListByMsg({
      appId,
      msgId,
      ...options,
      iterator: options?.iterator ?? undefined,
      before: options?.before ?? undefined,
      after: options?.after ?? undefined,
    });
  }

  /**
   * List messages for a particular endpoint. Additionally includes metadata about the latest message attempt.
   *
   * The `before` parameter lets you filter all items created before a certain date and is ignored if an iterator is passed.
   *
   * Note that by default this endpoint is limited to retrieving 90 days' worth of data
   * relative to now or, if an iterator is provided, 90 days before/after the time indicated
   * by the iterator ID. If you require data beyond those time ranges, you will need to explicitly
   * set the `before` or `after` parameter as appropriate.
   */
  public listAttemptedMessages(
    appId: string,
    endpointId: string,
    options?: MessageAttemptListAttemptedMessagesOptions
  ): Promise<ListResponseEndpointMessageOut> {
    return this.api.v1MessageAttemptListAttemptedMessages({
      appId,
      endpointId,
      ...options,
      iterator: options?.iterator ?? undefined,
      before: options?.before ?? undefined,
      after: options?.after ?? undefined,
    });
  }

  /** `msg_id`: Use a message id or a message `eventId` */
  public get(
    appId: string,
    msgId: string,
    attemptId: string
  ): Promise<MessageAttemptOut> {
    return this.api.v1MessageAttemptGet({
      appId,
      msgId,
      attemptId,
    });
  }

  /**
   * Deletes the given attempt's response body.
   *
   * Useful when an endpoint accidentally returned sensitive content.
   * The message can't be replayed or resent once its payload has been deleted or expired.
   */
  public expungeContent(appId: string, msgId: string, attemptId: string): Promise<void> {
    return this.api.v1MessageAttemptExpungeContent({
      appId,
      msgId,
      attemptId,
    });
  }

  /**
   * List endpoints attempted by a given message.
   *
   * Additionally includes metadata about the latest message attempt.
   * By default, endpoints are listed in ascending order by ID.
   */
  public listAttemptedDestinations(
    appId: string,
    msgId: string,
    options?: MessageAttemptListAttemptedDestinationsOptions
  ): Promise<ListResponseMessageEndpointOut> {
    return this.api.v1MessageAttemptListAttemptedDestinations({
      appId,
      msgId,
      ...options,
      iterator: options?.iterator ?? undefined,
    });
  }

  /** Resend a message to the specified endpoint. */
  public resend(
    appId: string,
    msgId: string,
    endpointId: string,
    options?: PostOptions
  ): Promise<void> {
    return this.api.v1MessageAttemptResend({
      appId,
      msgId,
      endpointId,
      ...options,
    });
  }
}
