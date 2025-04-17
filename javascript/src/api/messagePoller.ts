// this file is @generated
import {
  PollingEndpointConsumerSeekIn,
  PollingEndpointConsumerSeekInSerializer,
} from "../models/pollingEndpointConsumerSeekIn";
import {
  PollingEndpointConsumerSeekOut,
  PollingEndpointConsumerSeekOutSerializer,
} from "../models/pollingEndpointConsumerSeekOut";
import {
  PollingEndpointOut,
  PollingEndpointOutSerializer,
} from "../models/pollingEndpointOut";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";

export interface MessagePollerPollOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** Filters messages sent with this event type (optional). */
  eventType?: string;
  /** Filters messages sent with this channel (optional). */
  channel?: string;
  after?: Date | null;
}

export interface MessagePollerConsumerPollOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
}

export interface MessagePollerConsumerSeekOptions {
  idempotencyKey?: string;
}

export class MessagePoller {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** Reads the stream of created messages for an application, filtered on the Sink's event types and Channels. */
  public poll(
    appId: string,
    sinkId: string,
    options?: MessagePollerPollOptions
  ): Promise<PollingEndpointOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/app/{app_id}/poller/{sink_id}"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("sink_id", sinkId);
    request.setQueryParam("limit", options?.limit);
    request.setQueryParam("iterator", options?.iterator);
    request.setQueryParam("event_type", options?.eventType);
    request.setQueryParam("channel", options?.channel);
    request.setQueryParam("after", options?.after);

    return request.send(this.requestCtx, PollingEndpointOutSerializer._fromJsonObject);
  }

  /**
   * Reads the stream of created messages for an application, filtered on the Sink's event types and
   * Channels, using server-managed iterator tracking.
   */
  public consumerPoll(
    appId: string,
    sinkId: string,
    consumerId: string,
    options?: MessagePollerConsumerPollOptions
  ): Promise<PollingEndpointOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/app/{app_id}/poller/{sink_id}/consumer/{consumer_id}"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("sink_id", sinkId);
    request.setPathParam("consumer_id", consumerId);
    request.setQueryParam("limit", options?.limit);
    request.setQueryParam("iterator", options?.iterator);

    return request.send(this.requestCtx, PollingEndpointOutSerializer._fromJsonObject);
  }

  /** Sets the starting offset for the consumer of a polling endpoint. */
  public consumerSeek(
    appId: string,
    sinkId: string,
    consumerId: string,
    pollingEndpointConsumerSeekIn: PollingEndpointConsumerSeekIn,
    options?: MessagePollerConsumerSeekOptions
  ): Promise<PollingEndpointConsumerSeekOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/app/{app_id}/poller/{sink_id}/consumer/{consumer_id}/seek"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("sink_id", sinkId);
    request.setPathParam("consumer_id", consumerId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(
      PollingEndpointConsumerSeekInSerializer._toJsonObject(pollingEndpointConsumerSeekIn)
    );

    return request.send(
      this.requestCtx,
      PollingEndpointConsumerSeekOutSerializer._fromJsonObject
    );
  }
}
