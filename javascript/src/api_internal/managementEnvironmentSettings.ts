// this file is @generated
import { OtelConfig, OtelConfigSerializer } from "../models/otelConfig";
import {
  SettingsInternalIn,
  SettingsInternalInSerializer,
} from "../models/settingsInternalIn";
import {
  SettingsInternalOut,
  SettingsInternalOutSerializer,
} from "../models/settingsInternalOut";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";

export class ManagementEnvironmentSettings {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** Get the environments's settings */
  public get(): Promise<SettingsInternalOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/management/environment-settings"
    );

    return request.send(this.requestCtx, SettingsInternalOutSerializer._fromJsonObject);
  }

  /** Update the environment's settings */
  public update(settingsInternalIn: SettingsInternalIn): Promise<SettingsInternalOut> {
    const request = new SvixRequest(
      HttpMethod.PUT,
      "/api/v1/management/environment-settings"
    );

    request.setBody(SettingsInternalInSerializer._toJsonObject(settingsInternalIn));

    return request.send(this.requestCtx, SettingsInternalOutSerializer._fromJsonObject);
  }

  /** Update customer otel config. */
  public updateOtelConfig(otelConfig: OtelConfig): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.PUT,
      "/api/v1/management/environment-settings/customer-otel"
    );

    request.setBody(OtelConfigSerializer._toJsonObject(otelConfig));

    return request.sendNoResponseBody(this.requestCtx);
  }

  /** Delete customer otel config. */
  public deleteOtelConfig(): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.DELETE,
      "/api/v1/management/environment-settings/customer-otel"
    );

    return request.sendNoResponseBody(this.requestCtx);
  }
}
