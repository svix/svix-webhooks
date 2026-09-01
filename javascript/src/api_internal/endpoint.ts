// this file is @generated

import {
  type EndpointTransformationIn,
  EndpointTransformationInSerializer,
} from "../models/endpointTransformationIn";
import { EndpointAutoConfigDeprecated } from "./endpointAutoConfigDeprecated";
import { EndpointAutoconfig } from "./endpointAutoconfig";
import { HttpMethod, SvixRequest, type SvixRequestContext } from "../request";

export class Endpoint {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  public get autoConfigDeprecated() {
    return new EndpointAutoConfigDeprecated(this.requestCtx);
  }

  public get autoconfig() {
    return new EndpointAutoconfig(this.requestCtx);
  }

  /**
   * This operation was renamed to `set-transformation`.
   *
   * @deprecated
   */
  public async transformationPartialUpdate(
    appId: string,
    endpointId: string,
    endpointTransformationIn: EndpointTransformationIn = {}
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

    return await request.sendNoResponseBody(this.requestCtx);
  }
}
