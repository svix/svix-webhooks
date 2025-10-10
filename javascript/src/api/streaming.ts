// this file is @generated

import {
  type EndpointHeadersOut,
  EndpointHeadersOutSerializer,
} from "../models/endpointHeadersOut";
import {
  type HttpSinkHeadersPatchIn,
  HttpSinkHeadersPatchInSerializer,
} from "../models/httpSinkHeadersPatchIn";
import {
  type SinkTransformationOut,
  SinkTransformationOutSerializer,
} from "../models/sinkTransformationOut";
import { StreamingEventType } from "./streamingEventType";
import { StreamingEvents } from "./streamingEvents";
import { StreamingSink } from "./streamingSink";
import { StreamingStream } from "./streamingStream";
import { HttpMethod, SvixRequest, type SvixRequestContext } from "../request";

export class Streaming {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  public get event_type() {
    return new StreamingEventType(this.requestCtx);
  }

  public get events() {
    return new StreamingEvents(this.requestCtx);
  }

  public get sink() {
    return new StreamingSink(this.requestCtx);
  }

  public get stream() {
    return new StreamingStream(this.requestCtx);
  }

  /** Get the HTTP sink headers. Only valid for `http` or `otelTracing` sinks. */
  public sinkHeadersGet(streamId: string, sinkId: string): Promise<EndpointHeadersOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/stream/{stream_id}/sink/{sink_id}/headers"
    );

    request.setPathParam("stream_id", streamId);
    request.setPathParam("sink_id", sinkId);

    return request.send(this.requestCtx, EndpointHeadersOutSerializer._fromJsonObject);
  }

  /** Updates the Sink's headers. Only valid for `http` or `otelTracing` sinks. */
  public sinkHeadersPatch(
    streamId: string,
    sinkId: string,
    httpSinkHeadersPatchIn: HttpSinkHeadersPatchIn
  ): Promise<EndpointHeadersOut> {
    const request = new SvixRequest(
      HttpMethod.PATCH,
      "/api/v1/stream/{stream_id}/sink/{sink_id}/headers"
    );

    request.setPathParam("stream_id", streamId);
    request.setPathParam("sink_id", sinkId);
    request.setBody(
      HttpSinkHeadersPatchInSerializer._toJsonObject(httpSinkHeadersPatchIn)
    );

    return request.send(this.requestCtx, EndpointHeadersOutSerializer._fromJsonObject);
  }

  /** Get the transformation code associated with this sink. */
  public sinkTransformationGet(
    streamId: string,
    sinkId: string
  ): Promise<SinkTransformationOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/stream/{stream_id}/sink/{sink_id}/transformation"
    );

    request.setPathParam("stream_id", streamId);
    request.setPathParam("sink_id", sinkId);

    return request.send(this.requestCtx, SinkTransformationOutSerializer._fromJsonObject);
  }
}
