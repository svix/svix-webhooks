// this file is @generated

import {
  type AutoConfigSubscriptionOut,
  AutoConfigSubscriptionOutSerializer,
} from "../models/autoConfigSubscriptionOut";
import { type EndpointIn, EndpointInSerializer } from "../models/endpointIn";
import { type EndpointOut, EndpointOutSerializer } from "../models/endpointOut";
import { HttpMethod, SvixRequest, type SvixRequestContext } from "../request";

export class EndpointAutoconfig {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** Get an AutoConfig subscription, including the bound endpoint or destination if any. */
  public async get(
    appId: string,
    autoconfigId: string
  ): Promise<AutoConfigSubscriptionOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/app/{app_id}/autoconfig/{autoconfig_id}"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("autoconfig_id", autoconfigId);

    return await request.send(
      this.requestCtx,
      AutoConfigSubscriptionOutSerializer._fromJsonObject
    );
  }

  /** Create or update the HTTP endpoint for an AutoConfig subscription. */
  public async subscribe(
    appId: string,
    autoconfigId: string,
    endpointIn: EndpointIn
  ): Promise<EndpointOut> {
    const request = new SvixRequest(
      HttpMethod.PUT,
      "/api/v1/app/{app_id}/autoconfig/{autoconfig_id}/endpoint"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("autoconfig_id", autoconfigId);
    request.setBody(EndpointInSerializer._toJsonObject(endpointIn));

    return await request.send(this.requestCtx, EndpointOutSerializer._fromJsonObject);
  }
}
