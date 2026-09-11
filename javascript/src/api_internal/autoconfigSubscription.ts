// this file is @generated

import {
  type AutoConfigSubscriptionOut,
  AutoConfigSubscriptionOutSerializer,
} from "../models/autoConfigSubscriptionOut";
import { AutoconfigSubscriptionDestination } from "./autoconfigSubscriptionDestination";
import { AutoconfigSubscriptionEndpoint } from "./autoconfigSubscriptionEndpoint";
import { HttpMethod, SvixRequest, type SvixRequestContext } from "../request";

export class AutoconfigSubscription {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  public get destination() {
    return new AutoconfigSubscriptionDestination(this.requestCtx);
  }

  public get endpoint() {
    return new AutoconfigSubscriptionEndpoint(this.requestCtx);
  }

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
}
