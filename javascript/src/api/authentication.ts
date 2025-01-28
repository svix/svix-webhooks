// this file is @generated
import {
  AppPortalAccessIn,
  AppPortalAccessOut,
  ApplicationTokenExpireIn,
  DashboardAccessOut,
} from "../openapi";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";
import { PostOptions } from "../util";

export class Authentication {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal. */
  public appPortalAccess(
    appId: string,
    appPortalAccessIn: AppPortalAccessIn,
    options?: PostOptions
  ): Promise<AppPortalAccessOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/auth/app-portal-access/{app_id}"
    );

    request.setPathParam("app_id", appId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.body = appPortalAccessIn;

    return request.send(this.requestCtx);
  }

  /** Expire all of the tokens associated with a specific application. */
  public expireAll(
    appId: string,
    applicationTokenExpireIn: ApplicationTokenExpireIn,
    options?: PostOptions
  ): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/auth/app/{app_id}/expire-all"
    );

    request.setPathParam("app_id", appId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.body = applicationTokenExpireIn;

    return request.sendNoResponseBody(this.requestCtx);
  }

  /**
   * DEPRECATED: Please use `app-portal-access` instead.
   *
   * Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal.
   *
   * @deprecated
   */
  public dashboardAccess(
    appId: string,
    options?: PostOptions
  ): Promise<DashboardAccessOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/auth/dashboard-access/{app_id}"
    );

    request.setPathParam("app_id", appId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);

    return request.send(this.requestCtx);
  }

  /**
   * Logout an app token.
   *
   * Trying to log out other tokens will fail.
   */
  public logout(options?: PostOptions): Promise<void> {
    const request = new SvixRequest(HttpMethod.POST, "/api/v1/auth/logout");

    request.setHeaderParam("idempotency-key", options?.idempotencyKey);

    return request.sendNoResponseBody(this.requestCtx);
  }
}
