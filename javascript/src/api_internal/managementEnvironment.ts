// this file is @generated
import {
  EnvironmentModelIn,
  EnvironmentModelInSerializer,
} from "../models/environmentModelIn";
import {
  EnvironmentModelOut,
  EnvironmentModelOutSerializer,
} from "../models/environmentModelOut";
import {
  EnvironmentModelUpdate,
  EnvironmentModelUpdateSerializer,
} from "../models/environmentModelUpdate";
import {
  ListResponseEnvironmentModelOut,
  ListResponseEnvironmentModelOutSerializer,
} from "../models/listResponseEnvironmentModelOut";
import { Ordering } from "../models/ordering";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";

export interface ManagementEnvironmentListOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** The sorting order of the returned items */
  order?: Ordering;
}

export interface ManagementEnvironmentCreateOptions {
  idempotencyKey?: string;
}

export class ManagementEnvironment {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** List all environments. */
  public list(
    options?: ManagementEnvironmentListOptions
  ): Promise<ListResponseEnvironmentModelOut> {
    const request = new SvixRequest(HttpMethod.GET, "/api/v1/management/environment");

    request.setQueryParam("limit", options?.limit);
    request.setQueryParam("iterator", options?.iterator);
    request.setQueryParam("order", options?.order);

    return request.send(
      this.requestCtx,
      ListResponseEnvironmentModelOutSerializer._fromJsonObject
    );
  }

  /** Create a new environment. */
  public create(
    environmentModelIn: EnvironmentModelIn,
    options?: ManagementEnvironmentCreateOptions
  ): Promise<EnvironmentModelOut> {
    const request = new SvixRequest(HttpMethod.POST, "/api/v1/management/environment");

    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(EnvironmentModelInSerializer._toJsonObject(environmentModelIn));

    return request.send(this.requestCtx, EnvironmentModelOutSerializer._fromJsonObject);
  }

  /** Get an environment. */
  public get(envId: string): Promise<EnvironmentModelOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/management/environment/{env_id}"
    );

    request.setPathParam("env_id", envId);

    return request.send(this.requestCtx, EnvironmentModelOutSerializer._fromJsonObject);
  }

  /** Update an environment. */
  public update(
    envId: string,
    environmentModelUpdate: EnvironmentModelUpdate
  ): Promise<EnvironmentModelOut> {
    const request = new SvixRequest(
      HttpMethod.PUT,
      "/api/v1/management/environment/{env_id}"
    );

    request.setPathParam("env_id", envId);
    request.setBody(
      EnvironmentModelUpdateSerializer._toJsonObject(environmentModelUpdate)
    );

    return request.send(this.requestCtx, EnvironmentModelOutSerializer._fromJsonObject);
  }

  /** Delete an environment. */
  public delete(envId: string): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.DELETE,
      "/api/v1/management/environment/{env_id}"
    );

    request.setPathParam("env_id", envId);

    return request.sendNoResponseBody(this.requestCtx);
  }
}
