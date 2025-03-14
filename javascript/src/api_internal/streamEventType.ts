// this file is @generated
import {
  ListResponseStreamEventTypeOut,
  ListResponseStreamEventTypeOutSerializer,
} from "../models/listResponseStreamEventTypeOut";
import { Ordering } from "../models/ordering";
import {
  StreamEventTypeIn,
  StreamEventTypeInSerializer,
} from "../models/streamEventTypeIn";
import {
  StreamEventTypeOut,
  StreamEventTypeOutSerializer,
} from "../models/streamEventTypeOut";
import {
  StreamEventTypePatch,
  StreamEventTypePatchSerializer,
} from "../models/streamEventTypePatch";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";

export interface StreamEventTypeListOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** The sorting order of the returned items */
  order?: Ordering;
}

export interface StreamEventTypeCreateOptions {
  idempotencyKey?: string;
}

export class StreamEventType {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** List of all the organization's event types for streaming. */
  public list(
    options?: StreamEventTypeListOptions
  ): Promise<ListResponseStreamEventTypeOut> {
    const request = new SvixRequest(HttpMethod.GET, "/api/v1/stream/event-type");

    request.setQueryParam("limit", options?.limit);
    request.setQueryParam("iterator", options?.iterator);
    request.setQueryParam("order", options?.order);

    return request.send(
      this.requestCtx,
      ListResponseStreamEventTypeOutSerializer._fromJsonObject
    );
  }

  /** Create an event type for Streams. */
  public create(
    streamEventTypeIn: StreamEventTypeIn,
    options?: StreamEventTypeCreateOptions
  ): Promise<StreamEventTypeOut> {
    const request = new SvixRequest(HttpMethod.POST, "/api/v1/stream/event-type");

    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(StreamEventTypeInSerializer._toJsonObject(streamEventTypeIn));

    return request.send(this.requestCtx, StreamEventTypeOutSerializer._fromJsonObject);
  }

  /** Get an event type. */
  public get(name: string): Promise<StreamEventTypeOut> {
    const request = new SvixRequest(HttpMethod.GET, "/api/v1/stream/event-type/{name}");

    request.setPathParam("name", name);

    return request.send(this.requestCtx, StreamEventTypeOutSerializer._fromJsonObject);
  }

  /** Update or create a event type for Streams. */
  public update(
    name: string,
    streamEventTypeIn: StreamEventTypeIn
  ): Promise<StreamEventTypeOut> {
    const request = new SvixRequest(HttpMethod.PUT, "/api/v1/stream/event-type/{name}");

    request.setPathParam("name", name);
    request.setBody(StreamEventTypeInSerializer._toJsonObject(streamEventTypeIn));

    return request.send(this.requestCtx, StreamEventTypeOutSerializer._fromJsonObject);
  }

  /** Delete an event type. */
  public delete(name: string): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.DELETE,
      "/api/v1/stream/event-type/{name}"
    );

    request.setPathParam("name", name);

    return request.sendNoResponseBody(this.requestCtx);
  }

  /** Patch an event type for Streams. */
  public patch(
    name: string,
    streamEventTypePatch: StreamEventTypePatch
  ): Promise<StreamEventTypeOut> {
    const request = new SvixRequest(HttpMethod.PATCH, "/api/v1/stream/event-type/{name}");

    request.setPathParam("name", name);
    request.setBody(StreamEventTypePatchSerializer._toJsonObject(streamEventTypePatch));

    return request.send(this.requestCtx, StreamEventTypeOutSerializer._fromJsonObject);
  }
}
