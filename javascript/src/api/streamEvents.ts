// this file is @generated

import {
  type CreateStreamEventsIn,
  CreateStreamEventsInSerializer,
} from "../models/createStreamEventsIn";
import {
  type CreateStreamEventsOut,
  CreateStreamEventsOutSerializer,
} from "../models/createStreamEventsOut";
import { type EventStreamOut, EventStreamOutSerializer } from "../models/eventStreamOut";
import { HttpMethod, SvixRequest, type SvixRequestContext } from "../request";

export interface StreamEventsCreateOptions {
  idempotencyKey?: string;
}

export interface StreamEventsGetOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  after?: Date | null;
}

export class StreamEvents {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** Creates events on the Stream. */
  public create(
    streamId: string,
    createStreamEventsIn: CreateStreamEventsIn,
    options?: StreamEventsCreateOptions
  ): Promise<CreateStreamEventsOut> {
    const request = new SvixRequest(HttpMethod.POST, "/api/v1/stream/{stream_id}/events");

    request.setPathParam("stream_id", streamId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(CreateStreamEventsInSerializer._toJsonObject(createStreamEventsIn));

    return request.send(this.requestCtx, CreateStreamEventsOutSerializer._fromJsonObject);
  }

  /**
   * Iterate over a stream of events.
   *
   * The sink must be of type `poller` to use the poller endpoint.
   */
  public get(
    streamId: string,
    sinkId: string,
    options?: StreamEventsGetOptions
  ): Promise<EventStreamOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/stream/{stream_id}/sink/{sink_id}/events"
    );

    request.setPathParam("stream_id", streamId);
    request.setPathParam("sink_id", sinkId);
    request.setQueryParam("limit", options?.limit);
    request.setQueryParam("iterator", options?.iterator);
    request.setQueryParam("after", options?.after);

    return request.send(this.requestCtx, EventStreamOutSerializer._fromJsonObject);
  }
}
