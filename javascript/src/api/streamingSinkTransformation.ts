// this file is @generated

import { type EmptyResponse, EmptyResponseSerializer } from "../models/emptyResponse";
import {
  type SinkTransformIn,
  SinkTransformInSerializer,
} from "../models/sinkTransformIn";
import {
  type SinkTransformationOut,
  SinkTransformationOutSerializer,
} from "../models/sinkTransformationOut";
import { HttpMethod, SvixRequest, type SvixRequestContext } from "../request";

export class StreamingSinkTransformation {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** Get the transformation code associated with this sink. */
  public async get(streamId: string, sinkId: string): Promise<SinkTransformationOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/stream/{stream_id}/sink/{sink_id}/transformation"
    );

    request.setPathParam("stream_id", streamId);
    request.setPathParam("sink_id", sinkId);

    return await request.send(
      this.requestCtx,
      SinkTransformationOutSerializer._fromJsonObject
    );
  }

  /** Set or unset the transformation code associated with this sink. */
  public async patch(
    streamId: string,
    sinkId: string,
    sinkTransformIn: SinkTransformIn = {}
  ): Promise<EmptyResponse> {
    const request = new SvixRequest(
      HttpMethod.PATCH,
      "/api/v1/stream/{stream_id}/sink/{sink_id}/transformation"
    );

    request.setPathParam("stream_id", streamId);
    request.setPathParam("sink_id", sinkId);
    request.setBody(SinkTransformInSerializer._toJsonObject(sinkTransformIn));

    return await request.send(this.requestCtx, EmptyResponseSerializer._fromJsonObject);
  }
}
