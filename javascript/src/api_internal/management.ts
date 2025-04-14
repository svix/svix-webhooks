import { SvixRequestContext } from "../request";
import { ManagementEnvironment } from "./managementEnvironment";
import { ManagementEnvironmentSettings } from "./managementEnvironmentSettings";

export class Management {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  public get environment() {
    return new ManagementEnvironment(this.requestCtx);
  }

  public get environment_settings() {
    return new ManagementEnvironmentSettings(this.requestCtx);
  }
}
