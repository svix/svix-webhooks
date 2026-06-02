// this file is @generated

import { type IntegrationIn, IntegrationInSerializer } from "../models/integrationIn";
import {
  type IntegrationKeyOut,
  IntegrationKeyOutSerializer,
} from "../models/integrationKeyOut";
import { type IntegrationOut, IntegrationOutSerializer } from "../models/integrationOut";
import {
  type IntegrationUpdate,
  IntegrationUpdateSerializer,
} from "../models/integrationUpdate";
import {
  type ListResponseIntegrationOut,
  ListResponseIntegrationOutSerializer,
} from "../models/listResponseIntegrationOut";
import type { Ordering } from "../models/ordering";
import { HttpMethod, SvixRequest, type SvixRequestContext } from "../request";

export interface IntegrationListOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** The sorting order of the returned items */
  order?: Ordering;
}

export interface IntegrationCreateOptions {
  idempotencyKey?: string;
}

export interface IntegrationRotateKeyOptions {
  idempotencyKey?: string;
}

export class Integration {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** List the application's integrations. */
  public async list(
    appId: string,
    options?: IntegrationListOptions
  ): Promise<ListResponseIntegrationOut> {
    const request = new SvixRequest(HttpMethod.GET, "/api/v1/app/{app_id}/integration");

    request.setPathParam("app_id", appId);
    request.setQueryParams({
      limit: options?.limit,
      iterator: options?.iterator,
      order: options?.order,
    });

    return await request.send(
      this.requestCtx,
      ListResponseIntegrationOutSerializer._fromJsonObject
    );
  }

  /** Create an integration. */
  public async create(
    appId: string,
    integrationIn: IntegrationIn,
    options?: IntegrationCreateOptions
  ): Promise<IntegrationOut> {
    const request = new SvixRequest(HttpMethod.POST, "/api/v1/app/{app_id}/integration");

    request.setPathParam("app_id", appId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(IntegrationInSerializer._toJsonObject(integrationIn));

    return await request.send(this.requestCtx, IntegrationOutSerializer._fromJsonObject);
  }

  /** Get an integration. */
  public async get(appId: string, integId: string): Promise<IntegrationOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/app/{app_id}/integration/{integ_id}"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("integ_id", integId);

    return await request.send(this.requestCtx, IntegrationOutSerializer._fromJsonObject);
  }

  /** Update an integration. */
  public async update(
    appId: string,
    integId: string,
    integrationUpdate: IntegrationUpdate
  ): Promise<IntegrationOut> {
    const request = new SvixRequest(
      HttpMethod.PUT,
      "/api/v1/app/{app_id}/integration/{integ_id}"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("integ_id", integId);
    request.setBody(IntegrationUpdateSerializer._toJsonObject(integrationUpdate));

    return await request.send(this.requestCtx, IntegrationOutSerializer._fromJsonObject);
  }

  /** Delete an integration. */
  public async delete(appId: string, integId: string): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.DELETE,
      "/api/v1/app/{app_id}/integration/{integ_id}"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("integ_id", integId);

    return await request.sendNoResponseBody(this.requestCtx);
  }

  /**
   * Get an integration's key.
   *
   * @deprecated
   */
  public async getKey(appId: string, integId: string): Promise<IntegrationKeyOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/app/{app_id}/integration/{integ_id}/key"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("integ_id", integId);

    return await request.send(
      this.requestCtx,
      IntegrationKeyOutSerializer._fromJsonObject
    );
  }

  /** Rotate the integration's key. The previous key will be immediately revoked. */
  public async rotateKey(
    appId: string,
    integId: string,
    options?: IntegrationRotateKeyOptions
  ): Promise<IntegrationKeyOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/app/{app_id}/integration/{integ_id}/key/rotate"
    );

    request.setPathParam("app_id", appId);
    request.setPathParam("integ_id", integId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);

    return await request.send(
      this.requestCtx,
      IntegrationKeyOutSerializer._fromJsonObject
    );
  }
}
