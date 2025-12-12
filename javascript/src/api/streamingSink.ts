// this file is @generated

import { type EmptyResponse, EmptyResponseSerializer } from "../models/emptyResponse";
import {
  type EndpointSecretRotateIn,
  EndpointSecretRotateInSerializer,
} from "../models/endpointSecretRotateIn";
import {
  type ListResponseStreamSinkOut,
  ListResponseStreamSinkOutSerializer,
} from "../models/listResponseStreamSinkOut";
import type { Ordering } from "../models/ordering";
import { type SinkSecretOut, SinkSecretOutSerializer } from "../models/sinkSecretOut";
import {
  type SinkTransformIn,
  SinkTransformInSerializer,
} from "../models/sinkTransformIn";
import { type StreamSinkIn, StreamSinkInSerializer } from "../models/streamSinkIn";
import { type StreamSinkOut, StreamSinkOutSerializer } from "../models/streamSinkOut";
import {
  type StreamSinkPatch,
  StreamSinkPatchSerializer,
} from "../models/streamSinkPatch";
import { HttpMethod, SvixRequest, type SvixRequestContext } from "../request";

export interface StreamingSinkListOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** The sorting order of the returned items */
  order?: Ordering;
}

export interface StreamingSinkCreateOptions {
  idempotencyKey?: string;
}

export interface StreamingSinkRotateSecretOptions {
  idempotencyKey?: string;
}

export class StreamingSink {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** List of all the stream's sinks. */
  public list(
    streamId: string,
    options?: StreamingSinkListOptions
  ): Promise<ListResponseStreamSinkOut> {
    const request = new SvixRequest(HttpMethod.GET, "/api/v1/stream/{stream_id}/sink");

    request.setPathParam("stream_id", streamId);
    request.setQueryParams({
      limit: options?.limit,
      iterator: options?.iterator,
      order: options?.order,
    });

    return request.send(
      this.requestCtx,
      ListResponseStreamSinkOutSerializer._fromJsonObject
    );
  }

  /** Creates a new sink. */
  public create(
    streamId: string,
    streamSinkIn: StreamSinkIn,
    options?: StreamingSinkCreateOptions
  ): Promise<StreamSinkOut> {
    const request = new SvixRequest(HttpMethod.POST, "/api/v1/stream/{stream_id}/sink");

    request.setPathParam("stream_id", streamId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(StreamSinkInSerializer._toJsonObject(streamSinkIn));

    return request.send(this.requestCtx, StreamSinkOutSerializer._fromJsonObject);
  }

  /** Get a sink by id or uid. */
  public get(streamId: string, sinkId: string): Promise<StreamSinkOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/stream/{stream_id}/sink/{sink_id}"
    );

    request.setPathParam("stream_id", streamId);
    request.setPathParam("sink_id", sinkId);

    return request.send(this.requestCtx, StreamSinkOutSerializer._fromJsonObject);
  }

  /** Update a sink. */
  public update(
    streamId: string,
    sinkId: string,
    streamSinkIn: StreamSinkIn
  ): Promise<StreamSinkOut> {
    const request = new SvixRequest(
      HttpMethod.PUT,
      "/api/v1/stream/{stream_id}/sink/{sink_id}"
    );

    request.setPathParam("stream_id", streamId);
    request.setPathParam("sink_id", sinkId);
    request.setBody(StreamSinkInSerializer._toJsonObject(streamSinkIn));

    return request.send(this.requestCtx, StreamSinkOutSerializer._fromJsonObject);
  }

  /** Delete a sink. */
  public delete(streamId: string, sinkId: string): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.DELETE,
      "/api/v1/stream/{stream_id}/sink/{sink_id}"
    );

    request.setPathParam("stream_id", streamId);
    request.setPathParam("sink_id", sinkId);

    return request.sendNoResponseBody(this.requestCtx);
  }

  /** Partially update a sink. */
  public patch(
    streamId: string,
    sinkId: string,
    streamSinkPatch: StreamSinkPatch
  ): Promise<StreamSinkOut> {
    const request = new SvixRequest(
      HttpMethod.PATCH,
      "/api/v1/stream/{stream_id}/sink/{sink_id}"
    );

    request.setPathParam("stream_id", streamId);
    request.setPathParam("sink_id", sinkId);
    request.setBody(StreamSinkPatchSerializer._toJsonObject(streamSinkPatch));

    return request.send(this.requestCtx, StreamSinkOutSerializer._fromJsonObject);
  }

  /**
   * Get the sink's signing secret (only supported for http sinks)
   *
   * This is used to verify the authenticity of the delivery.
   *
   * For more information please refer to [the consuming webhooks docs](https://docs.svix.com/consuming-webhooks/).
   */
  public getSecret(streamId: string, sinkId: string): Promise<SinkSecretOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/stream/{stream_id}/sink/{sink_id}/secret"
    );

    request.setPathParam("stream_id", streamId);
    request.setPathParam("sink_id", sinkId);

    return request.send(this.requestCtx, SinkSecretOutSerializer._fromJsonObject);
  }

  /** Rotates the signing secret (only supported for http sinks). */
  public rotateSecret(
    streamId: string,
    sinkId: string,
    endpointSecretRotateIn: EndpointSecretRotateIn,
    options?: StreamingSinkRotateSecretOptions
  ): Promise<EmptyResponse> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/stream/{stream_id}/sink/{sink_id}/secret/rotate"
    );

    request.setPathParam("stream_id", streamId);
    request.setPathParam("sink_id", sinkId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(
      EndpointSecretRotateInSerializer._toJsonObject(endpointSecretRotateIn)
    );

    return request.send(this.requestCtx, EmptyResponseSerializer._fromJsonObject);
  }

  /** Set or unset the transformation code associated with this sink. */
  public transformationPartialUpdate(
    streamId: string,
    sinkId: string,
    sinkTransformIn: SinkTransformIn
  ): Promise<EmptyResponse> {
    const request = new SvixRequest(
      HttpMethod.PATCH,
      "/api/v1/stream/{stream_id}/sink/{sink_id}/transformation"
    );

    request.setPathParam("stream_id", streamId);
    request.setPathParam("sink_id", sinkId);
    request.setBody(SinkTransformInSerializer._toJsonObject(sinkTransformIn));

    return request.send(this.requestCtx, EmptyResponseSerializer._fromJsonObject);
  }
}
