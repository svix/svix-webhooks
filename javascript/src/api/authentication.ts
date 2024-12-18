import {
  Configuration,
  AuthenticationApi,
  DashboardAccessOut,
  AppPortalAccessOut,
  AppPortalAccessIn,
} from "../openapi";
import { PostOptions } from "../util";

export class Authentication {
  private readonly api: AuthenticationApi;

  public constructor(config: Configuration) {
    this.api = new AuthenticationApi(config);
  }

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

  public logout(options?: PostOptions): Promise<void> {
    return this.api.v1AuthenticationLogout({ ...options });
  }
}
