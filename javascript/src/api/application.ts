// this file is @generated

import { type ApplicationIn, ApplicationInSerializer } from "../models/applicationIn";
import { type ApplicationOut, ApplicationOutSerializer } from "../models/applicationOut";
import {
  type ApplicationPatch,
  ApplicationPatchSerializer,
} from "../models/applicationPatch";
import {
  type ListResponseApplicationOut,
  ListResponseApplicationOutSerializer,
} from "../models/listResponseApplicationOut";
import type { Ordering } from "../models/ordering";
import { HttpMethod, SvixRequest, type SvixRequestContext } from "../request";

export interface ApplicationListOptions {
  /** Exclude applications that have no endpoints. Default is false. */
  excludeAppsWithNoEndpoints?: boolean;
  /** Exclude applications that have only disabled endpoints. Default is false. */
  excludeAppsWithDisabledEndpoints?: boolean;
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** The sorting order of the returned items */
  order?: Ordering;
}

export interface ApplicationCreateOptions {
  idempotencyKey?: string;
}

export class Application {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** List of all the organization's applications. */
  public list(options?: ApplicationListOptions): Promise<ListResponseApplicationOut> {
    const request = new SvixRequest(HttpMethod.GET, "/api/v1/app");

    request.setQueryParams({
      exclude_apps_with_no_endpoints: options?.excludeAppsWithNoEndpoints,
      exclude_apps_with_disabled_endpoints: options?.excludeAppsWithDisabledEndpoints,
      limit: options?.limit,
      iterator: options?.iterator,
      order: options?.order,
    });

    return request.send(
      this.requestCtx,
      ListResponseApplicationOutSerializer._fromJsonObject
    );
  }

  /** Create a new application. */
  public create(
    applicationIn: ApplicationIn,
    options?: ApplicationCreateOptions
  ): Promise<ApplicationOut> {
    const request = new SvixRequest(HttpMethod.POST, "/api/v1/app");

    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(ApplicationInSerializer._toJsonObject(applicationIn));

    return request.send(this.requestCtx, ApplicationOutSerializer._fromJsonObject);
  }

  /** Get the application with the UID from `applicationIn`, or create it if it doesn't exist yet. */
  public getOrCreate(
    applicationIn: ApplicationIn,
    options?: ApplicationCreateOptions
  ): Promise<ApplicationOut> {
    const request = new SvixRequest(HttpMethod.POST, "/api/v1/app");

    request.setQueryParam("get_if_exists", true);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(ApplicationInSerializer._toJsonObject(applicationIn));

    return request.send(this.requestCtx, ApplicationOutSerializer._fromJsonObject);
  }

  /** Get an application. */
  public get(appId: string): Promise<ApplicationOut> {
    const request = new SvixRequest(HttpMethod.GET, "/api/v1/app/{app_id}");

    request.setPathParam("app_id", appId);

    return request.send(this.requestCtx, ApplicationOutSerializer._fromJsonObject);
  }

  /** Update an application. */
  public update(appId: string, applicationIn: ApplicationIn): Promise<ApplicationOut> {
    const request = new SvixRequest(HttpMethod.PUT, "/api/v1/app/{app_id}");

    request.setPathParam("app_id", appId);
    request.setBody(ApplicationInSerializer._toJsonObject(applicationIn));

    return request.send(this.requestCtx, ApplicationOutSerializer._fromJsonObject);
  }

  /** Delete an application. */
  public delete(appId: string): Promise<void> {
    const request = new SvixRequest(HttpMethod.DELETE, "/api/v1/app/{app_id}");

    request.setPathParam("app_id", appId);

    return request.sendNoResponseBody(this.requestCtx);
  }

  /** Partially update an application. */
  public patch(
    appId: string,
    applicationPatch: ApplicationPatch
  ): Promise<ApplicationOut> {
    const request = new SvixRequest(HttpMethod.PATCH, "/api/v1/app/{app_id}");

    request.setPathParam("app_id", appId);
    request.setBody(ApplicationPatchSerializer._toJsonObject(applicationPatch));

    return request.send(this.requestCtx, ApplicationOutSerializer._fromJsonObject);
  }
}
