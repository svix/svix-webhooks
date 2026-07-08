// this file is @generated

import {
  type EndpointTransformationOut,
  EndpointTransformationOutSerializer,
} from "../models/endpointTransformationOut";
import {
  type EndpointTransformationPatch,
  EndpointTransformationPatchSerializer,
} from "../models/endpointTransformationPatch";
import { HttpMethod, SvixRequest, type SvixRequestContext } from "../request";

export class EndpointTransformation {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** Get the transformation code associated with this endpoint. */
  public async get(
    appId: string,
    endpointId: string
  ): Promise<EndpointTransformationOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);

    return await request.send(
      this.requestCtx,
      EndpointTransformationOutSerializer._fromJsonObject
    );
  }

  /** Set or unset the transformation code associated with this endpoint. */
  public async patch(
    appId: string,
    endpointId: string,
    endpointTransformationPatch: EndpointTransformationPatch
  ): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.PATCH,
      "/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);
    request.setBody(
      EndpointTransformationPatchSerializer._toJsonObject(endpointTransformationPatch)
    );

    return await request.sendNoResponseBody(this.requestCtx);
  }
}
