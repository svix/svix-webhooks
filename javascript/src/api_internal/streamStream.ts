// this file is @generated
import {
  ListResponseStreamOut,
  ListResponseStreamOutSerializer,
} from "../models/listResponseStreamOut";
import { Ordering } from "../models/ordering";
import { StreamIn, StreamInSerializer } from "../models/streamIn";
import { StreamOut, StreamOutSerializer } from "../models/streamOut";
import { StreamPatch, StreamPatchSerializer } from "../models/streamPatch";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";

export interface StreamStreamListOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** The sorting order of the returned items */
  order?: Ordering;
}

export interface StreamStreamCreateOptions {
  idempotencyKey?: string;
}

export class StreamStream {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** List of all the organization's streams. */
  public list(options?: StreamStreamListOptions): Promise<ListResponseStreamOut> {
    const request = new SvixRequest(HttpMethod.GET, "/api/v1/stream");

    request.setQueryParam("limit", options?.limit);
    request.setQueryParam("iterator", options?.iterator);
    request.setQueryParam("order", options?.order);

    return request.send(this.requestCtx, ListResponseStreamOutSerializer._fromJsonObject);
  }

  /** Creates a new stream. */
  public create(
    streamIn: StreamIn,
    options?: StreamStreamCreateOptions
  ): Promise<StreamOut> {
    const request = new SvixRequest(HttpMethod.POST, "/api/v1/stream");

    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(StreamInSerializer._toJsonObject(streamIn));

    return request.send(this.requestCtx, StreamOutSerializer._fromJsonObject);
  }

  /** Get a stream by id or uid. */
  public get(streamId: string): Promise<StreamOut> {
    const request = new SvixRequest(HttpMethod.GET, "/api/v1/stream/{stream_id}");

    request.setPathParam("stream_id", streamId);

    return request.send(this.requestCtx, StreamOutSerializer._fromJsonObject);
  }

  /** Update a stream. */
  public update(streamId: string, streamIn: StreamIn): Promise<StreamOut> {
    const request = new SvixRequest(HttpMethod.PUT, "/api/v1/stream/{stream_id}");

    request.setPathParam("stream_id", streamId);
    request.setBody(StreamInSerializer._toJsonObject(streamIn));

    return request.send(this.requestCtx, StreamOutSerializer._fromJsonObject);
  }

  /** Delete a stream. */
  public delete(streamId: string): Promise<void> {
    const request = new SvixRequest(HttpMethod.DELETE, "/api/v1/stream/{stream_id}");

    request.setPathParam("stream_id", streamId);

    return request.sendNoResponseBody(this.requestCtx);
  }

  /** Partially update a stream. */
  public patch(streamId: string, streamPatch: StreamPatch): Promise<StreamOut> {
    const request = new SvixRequest(HttpMethod.PATCH, "/api/v1/stream/{stream_id}");

    request.setPathParam("stream_id", streamId);
    request.setBody(StreamPatchSerializer._toJsonObject(streamPatch));

    return request.send(this.requestCtx, StreamOutSerializer._fromJsonObject);
  }
}
