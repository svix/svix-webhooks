// this file is @generated
import { ApiTokenOut, ApiTokenOutSerializer } from "../models/apiTokenOut";
import {
  AppPortalAccessOut,
  AppPortalAccessOutSerializer,
} from "../models/appPortalAccessOut";
import { CreateTokenIn, CreateTokenInSerializer } from "../models/createTokenIn";
import { OneTimeTokenIn, OneTimeTokenInSerializer } from "../models/oneTimeTokenIn";
import { OneTimeTokenOut, OneTimeTokenOutSerializer } from "../models/oneTimeTokenOut";
import {
  RotatePollerTokenIn,
  RotatePollerTokenInSerializer,
} from "../models/rotatePollerTokenIn";
import {
  StreamPortalAccessIn,
  StreamPortalAccessInSerializer,
} from "../models/streamPortalAccessIn";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";

export interface AuthenticationCreateMessageTokenOptions {
  idempotencyKey?: string;
}

export interface AuthenticationRotatePollerTokenOptions {
  idempotencyKey?: string;
}

export interface AuthenticationExchangeOneTimeTokenOptions {
  idempotencyKey?: string;
}

export interface AuthenticationStreamPortalAccessOptions {
  idempotencyKey?: string;
}

export class Authentication {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** Create a new access token that only allows creating messages inside this application. */
  public createMessageToken(
    appId: string,
    createTokenIn: CreateTokenIn,
    options?: AuthenticationCreateMessageTokenOptions
  ): Promise<ApiTokenOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/auth/app/{app_id}/create-message-token"
    );

    request.setPathParam("app_id", appId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(CreateTokenInSerializer._toJsonObject(createTokenIn));

    return request.send(this.requestCtx, ApiTokenOutSerializer._fromJsonObject);
  }

  /** Get the current auth token for the poller. */
  public getPollerToken(appId: string, endpointId: string): Promise<ApiTokenOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/auth/app/{app_id}/poller/{endpoint_id}/token"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);

    return request.send(this.requestCtx, ApiTokenOutSerializer._fromJsonObject);
  }

  /** Create a new auth token that can for the poller API. */
  public rotatePollerToken(
    appId: string,
    endpointId: string,
    rotatePollerTokenIn: RotatePollerTokenIn,
    options?: AuthenticationRotatePollerTokenOptions
  ): Promise<ApiTokenOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/auth/app/{app_id}/poller/{endpoint_id}/token/rotate"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(RotatePollerTokenInSerializer._toJsonObject(rotatePollerTokenIn));

    return request.send(this.requestCtx, ApiTokenOutSerializer._fromJsonObject);
  }

  /** This is a one time token. */
  public exchangeOneTimeToken(
    oneTimeTokenIn: OneTimeTokenIn,
    options?: AuthenticationExchangeOneTimeTokenOptions
  ): Promise<OneTimeTokenOut> {
    const request = new SvixRequest(HttpMethod.POST, "/api/v1/auth/one-time-token");

    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(OneTimeTokenInSerializer._toJsonObject(oneTimeTokenIn));

    return request.send(this.requestCtx, OneTimeTokenOutSerializer._fromJsonObject);
  }

  /** Use this function to get magic links (and authentication codes) for connecting your users to the Stream Consumer Portal. */
  public streamPortalAccess(
    streamId: string,
    streamPortalAccessIn: StreamPortalAccessIn,
    options?: AuthenticationStreamPortalAccessOptions
  ): Promise<AppPortalAccessOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/auth/stream-portal-access/{stream_id}"
    );

    request.setPathParam("stream_id", streamId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(StreamPortalAccessInSerializer._toJsonObject(streamPortalAccessIn));

    return request.send(this.requestCtx, AppPortalAccessOutSerializer._fromJsonObject);
  }
}
