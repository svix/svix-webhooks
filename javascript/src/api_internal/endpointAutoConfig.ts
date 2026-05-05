// this file is @generated

import { type EndpointOut, EndpointOutSerializer } from "../models/endpointOut";
import { type SubscribeIn, SubscribeInSerializer } from "../models/subscribeIn";
import { HttpMethod, SvixRequest, type SvixRequestContext } from "../request";

export class EndpointAutoConfig {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** Update an auto-config endpoint by providing endpoint details. */
  public update(
    appId: string,
    endpointId: string,
    subscribeIn: SubscribeIn
  ): Promise<EndpointOut> {
    const request = new SvixRequest(
      HttpMethod.PUT,
      "/api/v1/app/{app_id}/endpoint/{endpoint_id}/auto-config"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);
    request.setBody(SubscribeInSerializer._toJsonObject(subscribeIn));

    return request.send(this.requestCtx, EndpointOutSerializer._fromJsonObject);
  }
}
