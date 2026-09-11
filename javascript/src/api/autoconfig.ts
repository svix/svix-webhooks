// this file is @generated

import { type AutoConfigOut, AutoConfigOutSerializer } from "../models/autoConfigOut";
import {
  type RotateSubscriptionIn2,
  RotateSubscriptionIn2Serializer,
} from "../models/rotateSubscriptionIn2";
import { HttpMethod, SvixRequest, type SvixRequestContext } from "../request";

export interface AutoconfigCreateOptions {
  idempotencyKey?: string;
}

export interface AutoconfigRotateOptions {
  idempotencyKey?: string;
}

export class Autoconfig {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** Create an AutoConfig subscription. */
  public async create(
    appId: string,
    options?: AutoconfigCreateOptions
  ): Promise<AutoConfigOut> {
    const request = new SvixRequest(HttpMethod.POST, "/api/v1/app/{app_id}/autoconfig");

    request.setPathParam("app_id", appId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);

    return await request.send(this.requestCtx, AutoConfigOutSerializer._fromJsonObject);
  }

  /** Rotate the auth token and signing secret for an AutoConfig subscription. */
  public async rotate(
    appId: string,
    autoconfigId: string,
    rotateSubscriptionIn2: RotateSubscriptionIn2 = {},
    options?: AutoconfigRotateOptions
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
