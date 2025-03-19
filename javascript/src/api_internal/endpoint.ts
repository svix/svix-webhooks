// this file is @generated
import {
  EndpointMtlsConfigIn,
  EndpointMtlsConfigInSerializer,
} from "../models/endpointMtlsConfigIn";
import {
  EndpointMtlsConfigOut,
  EndpointMtlsConfigOutSerializer,
} from "../models/endpointMtlsConfigOut";
import {
  EndpointOauthConfigIn,
  EndpointOauthConfigInSerializer,
} from "../models/endpointOauthConfigIn";
import {
  EndpointOauthConfigOut,
  EndpointOauthConfigOutSerializer,
} from "../models/endpointOauthConfigOut";
import {
  EndpointTransformationSimulateIn,
  EndpointTransformationSimulateInSerializer,
} from "../models/endpointTransformationSimulateIn";
import {
  EndpointTransformationSimulateOut,
  EndpointTransformationSimulateOutSerializer,
} from "../models/endpointTransformationSimulateOut";
import {
  HubspotOauthConfigIn,
  HubspotOauthConfigInSerializer,
} from "../models/hubspotOauthConfigIn";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";

export interface EndpointTransformationSimulateOptions {
  idempotencyKey?: string;
}

export class Endpoint {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** Get endpoint mTLS configuration. */
  public getMtlsConfig(
    appId: string,
    endpointId: string
  ): Promise<EndpointMtlsConfigOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/app/{app_id}/endpoint/{endpoint_id}/mtls"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);

    return request.send(this.requestCtx, EndpointMtlsConfigOutSerializer._fromJsonObject);
  }

  /** Create / update endpoint mTLS configuration. */
  public updateMtlsConfig(
    appId: string,
    endpointId: string,
    endpointMtlsConfigIn: EndpointMtlsConfigIn
  ): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.PUT,
      "/api/v1/app/{app_id}/endpoint/{endpoint_id}/mtls"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);
    request.setBody(EndpointMtlsConfigInSerializer._toJsonObject(endpointMtlsConfigIn));

    return request.sendNoResponseBody(this.requestCtx);
  }

  /** Delete endpoint mTLS configuration. */
  public deleteMtlsConfig(appId: string, endpointId: string): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.DELETE,
      "/api/v1/app/{app_id}/endpoint/{endpoint_id}/mtls"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);

    return request.sendNoResponseBody(this.requestCtx);
  }

  /** Get endpoint OAuth configuration. */
  public getOauthConfig(
    appId: string,
    endpointId: string
  ): Promise<EndpointOauthConfigOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/app/{app_id}/endpoint/{endpoint_id}/oauth"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);

    return request.send(
      this.requestCtx,
      EndpointOauthConfigOutSerializer._fromJsonObject
    );
  }

  /** Create / update endpoint OAuth configuration. */
  public updateOauthConfig(
    appId: string,
    endpointId: string,
    endpointOauthConfigIn: EndpointOauthConfigIn
  ): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.PUT,
      "/api/v1/app/{app_id}/endpoint/{endpoint_id}/oauth"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);
    request.setBody(EndpointOauthConfigInSerializer._toJsonObject(endpointOauthConfigIn));

    return request.sendNoResponseBody(this.requestCtx);
  }

  /** Delete endpoint OAuth configuration. */
  public deleteOauthConfig(appId: string, endpointId: string): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.DELETE,
      "/api/v1/app/{app_id}/endpoint/{endpoint_id}/oauth"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);

    return request.sendNoResponseBody(this.requestCtx);
  }

  /**
   * Create / update endpoint Hubspot OAuth configuration.
   *
   * Specific private endpoint just for us, to avoid exposing the Hubspot secret to the client.
   */
  public updateHubspotOauthConfig(
    appId: string,
    endpointId: string,
    hubspotOauthConfigIn: HubspotOauthConfigIn
  ): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.PUT,
      "/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation-template/oauth/hubspot"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);
    request.setBody(HubspotOauthConfigInSerializer._toJsonObject(hubspotOauthConfigIn));

    return request.sendNoResponseBody(this.requestCtx);
  }

  /** Simulate running the transformation on the payload and code. */
  public transformationSimulate(
    appId: string,
    endpointId: string,
    endpointTransformationSimulateIn: EndpointTransformationSimulateIn,
    options?: EndpointTransformationSimulateOptions
  ): Promise<EndpointTransformationSimulateOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/app/{app_id}/endpoint/{endpoint_id}/transformation/simulate"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("endpoint_id", endpointId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(
      EndpointTransformationSimulateInSerializer._toJsonObject(
        endpointTransformationSimulateIn
      )
    );

    return request.send(
      this.requestCtx,
      EndpointTransformationSimulateOutSerializer._fromJsonObject
    );
  }
}
