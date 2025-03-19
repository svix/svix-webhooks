// this file is @generated
import { CreateStreamIn, CreateStreamInSerializer } from "../models/createStreamIn";
import { CreateStreamOut, CreateStreamOutSerializer } from "../models/createStreamOut";
import { EventStreamOut, EventStreamOutSerializer } from "../models/eventStreamOut";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";

export interface StreamEventsGetOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  after?: Date | null;
}

export interface StreamEventsCreateOptions {
  idempotencyKey?: string;
}

export class StreamEvents {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** Iterate over a stream of events. */
  public get(
    streamId: string,
    options?: StreamEventsGetOptions
  ): Promise<EventStreamOut> {
    const request = new SvixRequest(HttpMethod.GET, "/api/v1/stream/{stream_id}/events");

    request.setPathParam("stream_id", streamId);
    request.setQueryParam("limit", options?.limit);
    request.setQueryParam("iterator", options?.iterator);
    request.setQueryParam("after", options?.after);

    return request.send(this.requestCtx, EventStreamOutSerializer._fromJsonObject);
  }

  /** Creates events on the Stream. */
  public create(
    streamId: string,
    createStreamIn: CreateStreamIn,
    options?: StreamEventsCreateOptions
  ): Promise<CreateStreamOut> {
    const request = new SvixRequest(HttpMethod.POST, "/api/v1/stream/{stream_id}/events");

    request.setPathParam("stream_id", streamId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(CreateStreamInSerializer._toJsonObject(createStreamIn));

    return request.send(this.requestCtx, CreateStreamOutSerializer._fromJsonObject);
  }
}
