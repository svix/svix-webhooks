// this file is @generated

import {
  type IngestEndpointTransformationOut,
  IngestEndpointTransformationOutSerializer,
} from "../models/ingestEndpointTransformationOut";
import {
  type IngestEndpointTransformationPatch,
  IngestEndpointTransformationPatchSerializer,
} from "../models/ingestEndpointTransformationPatch";
import { HttpMethod, SvixRequest, type SvixRequestContext } from "../request";

export class IngestEndpointTransformation {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** Get the transformation code associated with this ingest endpoint. */
  public async transformation(
    sourceId: string,
    endpointId: string
  ): Promise<IngestEndpointTransformationOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/transformation"
    );

    request.setPathParam("source_id", sourceId);
    request.setPathParam("endpoint_id", endpointId);

    return await request.send(
      this.requestCtx,
      IngestEndpointTransformationOutSerializer._fromJsonObject
    );
  }

  /** Set or unset the transformation code associated with this ingest endpoint. */
  public async patch(
    sourceId: string,
    endpointId: string,
    ingestEndpointTransformationPatch: IngestEndpointTransformationPatch
  ): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.PATCH,
      "/ingest/api/v1/source/{source_id}/endpoint/{endpoint_id}/transformation"
    );

    request.setPathParam("source_id", sourceId);
    request.setPathParam("endpoint_id", endpointId);
    request.setBody(
      IngestEndpointTransformationPatchSerializer._toJsonObject(
        ingestEndpointTransformationPatch
      )
    );

    return await request.sendNoResponseBody(this.requestCtx);
  }
}
