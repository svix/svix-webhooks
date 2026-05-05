// this file is @generated

import {
  type EndpointTransformationIn,
  EndpointTransformationInSerializer,
} from "../models/endpointTransformationIn";
import { EndpointAutoConfig } from "./endpointAutoConfig";
import { HttpMethod, SvixRequest, type SvixRequestContext } from "../request";

export class Endpoint {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  public get auto_config() {
    return new EndpointAutoConfig(this.requestCtx);
  }

  /**
   * This operation was renamed to `set-transformation`.
   *
   * @deprecated
   */
  public transformationPartialUpdate(
    appId: string,
    endpointId: string,
    endpointTransformationIn: EndpointTransformationIn
  ): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.PATCH,
      "/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);
    request.setBody(
      EndpointTransformationInSerializer._toJsonObject(endpointTransformationIn)
    );

    return request.sendNoResponseBody(this.requestCtx);
  }
}
