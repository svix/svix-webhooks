// this file is @generated

import { type AutoConfigOut, AutoConfigOutSerializer } from "../models/autoConfigOut";
import {
  type CreateAutoConfigSubscriptionIn,
  CreateAutoConfigSubscriptionInSerializer,
} from "../models/createAutoConfigSubscriptionIn";
import {
  type RotateSubscriptionIn2,
  RotateSubscriptionIn2Serializer,
} from "../models/rotateSubscriptionIn2";
import { HttpMethod, SvixRequest, type SvixRequestContext } from "../request";

export interface AutoconfigSubscriptionCreateOptions {
  idempotencyKey?: string;
}

export interface AutoconfigSubscriptionRotateOptions {
  idempotencyKey?: string;
}

export class AutoconfigSubscription {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** Create an AutoConfig subscription. */
  public async create(
    appId: string,
    createAutoConfigSubscriptionIn: CreateAutoConfigSubscriptionIn = {},
    options?: AutoconfigSubscriptionCreateOptions
  ): Promise<AutoConfigOut> {
    const request = new SvixRequest(HttpMethod.POST, "/api/v1/app/{app_id}/autoconfig");

    request.setPathParam("app_id", appId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(
      CreateAutoConfigSubscriptionInSerializer._toJsonObject(
        createAutoConfigSubscriptionIn
      )
    );

    return await request.send(this.requestCtx, AutoConfigOutSerializer._fromJsonObject);
  }

  /** Delete an AutoConfig subscription. This also invalidates its auth token. */
  public async delete(appId: string, autoconfigId: string): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.DELETE,
      "/api/v1/app/{app_id}/autoconfig/{autoconfig_id}"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("autoconfig_id", autoconfigId);

    return await request.sendNoResponseBody(this.requestCtx);
  }

  /** Rotate the auth token and signing secret for an AutoConfig subscription. */
  public async rotate(
    appId: string,
    autoconfigId: string,
    rotateSubscriptionIn2: RotateSubscriptionIn2 = {},
    options?: AutoconfigSubscriptionRotateOptions
  ): Promise<AutoConfigOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/app/{app_id}/autoconfig/{autoconfig_id}/rotate"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("autoconfig_id", autoconfigId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(RotateSubscriptionIn2Serializer._toJsonObject(rotateSubscriptionIn2));

    return await request.send(this.requestCtx, AutoConfigOutSerializer._fromJsonObject);
  }
}
