// this file is @generated

import { type DestinationIn, DestinationInSerializer } from "../models/destinationIn";
import { type DestinationOut, DestinationOutSerializer } from "../models/destinationOut";
import { HttpMethod, SvixRequest, type SvixRequestContext } from "../request";

export class DestinationAutoconfig {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** Create or update the destination for an AutoConfig subscription. */
  public async subscribe(
    appId: string,
    autoconfigId: string,
    destinationIn: DestinationIn
  ): Promise<DestinationOut> {
    const request = new SvixRequest(
      HttpMethod.PUT,
      "/api/v1/app/{app_id}/autoconfig/{autoconfig_id}/destination"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("autoconfig_id", autoconfigId);
    request.setBody(DestinationInSerializer._toJsonObject(destinationIn));

    return await request.send(this.requestCtx, DestinationOutSerializer._fromJsonObject);
  }
}
