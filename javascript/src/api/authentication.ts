// this file is @generated (with minor manual changes)
import {
  Configuration,
  AuthenticationApi,
  AppPortalAccessIn,
  AppPortalAccessOut,
  ApplicationTokenExpireIn,
  DashboardAccessOut,
} from "../openapi";
import { PostOptions } from "../util";

export class Authentication {
  private readonly api: AuthenticationApi;

  public constructor(config: Configuration) {
    this.api = new AuthenticationApi(config);
  }

  /** Use this function to get magic links (and authentication codes) for connecting your users to the Consumer Application Portal. */
  public appPortalAccess(
    appId: string,
    appPortalAccessIn: AppPortalAccessIn,
    options?: PostOptions
  ): Promise<AppPortalAccessOut> {
    return this.api.v1AuthenticationAppPortalAccess({
      appId,
      appPortalAccessIn,
      ...options,
    });
  }

  public dashboardAccess(
    appId: string,
    options?: PostOptions
  ): Promise<DashboardAccessOut> {
    return this.api.v1AuthenticationDashboardAccess({
      appId,
      ...options,
    });
  }

  /** Expire all of the tokens associated with a specific application. */
  public expireAll(
    appId: string,
    applicationTokenExpireIn: ApplicationTokenExpireIn,
    options?: PostOptions
  ): Promise<void> {
    return this.api.v1AuthenticationExpireAll({
      appId,
      applicationTokenExpireIn,
      ...options,
    });
  }

  /**
   * Logout an app token.
   *
   * Trying to log out other tokens will fail.
   */
  public logout(options?: PostOptions): Promise<void> {
    return this.api.v1AuthenticationLogout({
      ...options,
    });
  }
}
