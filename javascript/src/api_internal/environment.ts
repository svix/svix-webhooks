// this file is @generated
import {
  EnvironmentSettingsOut,
  EnvironmentSettingsOutSerializer,
} from "../models/environmentSettingsOut";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";

export class Environment {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** Get the environment's settings. */
  public getSettings(): Promise<EnvironmentSettingsOut> {
    const request = new SvixRequest(HttpMethod.GET, "/api/v1/environment/settings");

    return request.send(
      this.requestCtx,
      EnvironmentSettingsOutSerializer._fromJsonObject
    );
  }
}
