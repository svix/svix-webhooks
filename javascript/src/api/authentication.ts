// this file is @generated

import { type ApiTokenOut, ApiTokenOutSerializer } from "../models/apiTokenOut";
import {
  type AppPortalAccessIn,
  AppPortalAccessInSerializer,
} from "../models/appPortalAccessIn";
import {
  type AppPortalAccessOut,
  AppPortalAccessOutSerializer,
} from "../models/appPortalAccessOut";
import {
  type ApplicationTokenExpireIn,
  ApplicationTokenExpireInSerializer,
} from "../models/applicationTokenExpireIn";
import {
  type RotatePollerTokenIn,
  RotatePollerTokenInSerializer,
} from "../models/rotatePollerTokenIn";
import {
  type StreamPortalAccessIn,
  StreamPortalAccessInSerializer,
} from "../models/streamPortalAccessIn";
import {
  type DashboardAccessOut,
  DashboardAccessOutSerializer,
} from "../models/dashboardAccessOut";
import { HttpMethod, SvixRequest, type SvixRequestContext } from "../request";

export interface AuthenticationAppPortalAccessOptions {
  idempotencyKey?: string;
}

export interface AuthenticationExpireAllOptions {
  idempotencyKey?: string;
}

export interface AuthenticationLogoutOptions {
  idempotencyKey?: string;
}

export interface AuthenticationStreamPortalAccessOptions {
  idempotencyKey?: string;
}

export interface AuthenticationRotateStreamPollerTokenOptions {
  idempotencyKey?: string;
}

/** @deprecated */
export interface AuthenticationDashboardAccessOptions {
  idempotencyKey?: string;
}

export class Authentication {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal. */
  public appPortalAccess(
    appId: string,
    appPortalAccessIn: AppPortalAccessIn,
    options?: AuthenticationAppPortalAccessOptions
  ): Promise<AppPortalAccessOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/auth/app-portal-access/{app_id}"
    );

    request.setPathParam("app_id", appId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(AppPortalAccessInSerializer._toJsonObject(appPortalAccessIn));

    return request.send(this.requestCtx, AppPortalAccessOutSerializer._fromJsonObject);
  }

  /** Expire all of the tokens associated with a specific application. */
  public expireAll(
    appId: string,
    applicationTokenExpireIn: ApplicationTokenExpireIn,
    options?: AuthenticationExpireAllOptions
  ): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/auth/app/{app_id}/expire-all"
    );

    request.setPathParam("app_id", appId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(
      ApplicationTokenExpireInSerializer._toJsonObject(applicationTokenExpireIn)
    );

    return request.sendNoResponseBody(this.requestCtx);
  }

  /** @deprecated Please use `appPortalAccess` instead. */
  public dashboardAccess(
    appId: string,
    options?: AuthenticationDashboardAccessOptions
  ): Promise<DashboardAccessOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/auth/dashboard-access/{app_id}"
    );

    request.setPathParam("app_id", appId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);

    return request.send(this.requestCtx, DashboardAccessOutSerializer._fromJsonObject);
  }

  /**
   * Logout an app token.
   *
   * Trying to log out other tokens will fail.
   */
  public logout(options?: AuthenticationLogoutOptions): Promise<void> {
    const request = new SvixRequest(HttpMethod.POST, "/api/v1/auth/logout");

    request.setHeaderParam("idempotency-key", options?.idempotencyKey);

    return request.sendNoResponseBody(this.requestCtx);
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

  /** Get the current auth token for the stream poller. */
  public getStreamPollerToken(streamId: string, sinkId: string): Promise<ApiTokenOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/auth/stream/{stream_id}/sink/{sink_id}/poller/token"
    );

    request.setPathParam("stream_id", streamId);
    request.setPathParam("sink_id", sinkId);

    return request.send(this.requestCtx, ApiTokenOutSerializer._fromJsonObject);
  }

  /** Create a new auth token for the stream poller API. */
  public rotateStreamPollerToken(
    streamId: string,
    sinkId: string,
    rotatePollerTokenIn: RotatePollerTokenIn,
    options?: AuthenticationRotateStreamPollerTokenOptions
  ): Promise<ApiTokenOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/auth/stream/{stream_id}/sink/{sink_id}/poller/token/rotate"
    );

    request.setPathParam("stream_id", streamId);
    request.setPathParam("sink_id", sinkId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(RotatePollerTokenInSerializer._toJsonObject(rotatePollerTokenIn));

    return request.send(this.requestCtx, ApiTokenOutSerializer._fromJsonObject);
  }
}
