// this file is @generated
import { ApiTokenExpireIn, ApiTokenExpireInSerializer } from "../models/apiTokenExpireIn";
import { ApiTokenOut, ApiTokenOutSerializer } from "../models/apiTokenOut";
import { ApiTokenUpdate, ApiTokenUpdateSerializer } from "../models/apiTokenUpdate";
import { CreateTokenIn, CreateTokenInSerializer } from "../models/createTokenIn";
import {
  GlobalApiTokenCensoredOut,
  GlobalApiTokenCensoredOutSerializer,
} from "../models/globalApiTokenCensoredOut";
import {
  ListResponseGlobalApiTokenCensoredOut,
  ListResponseGlobalApiTokenCensoredOutSerializer,
} from "../models/listResponseGlobalApiTokenCensoredOut";
import { Ordering } from "../models/ordering";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";

export interface AuthenticationOrgGroupAdminTokenListOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** The sorting order of the returned items */
  order?: Ordering;
}

export interface AuthenticationOrgGroupAdminTokenCreateOptions {
  idempotencyKey?: string;
}

export interface AuthenticationOrgGroupAdminTokenExpireOptions {
  idempotencyKey?: string;
}

export class AuthenticationOrgGroupAdminToken {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** List all org group API tokens. */
  public list(
    options?: AuthenticationOrgGroupAdminTokenListOptions
  ): Promise<ListResponseGlobalApiTokenCensoredOut> {
    const request = new SvixRequest(HttpMethod.GET, "/api/v1/auth/org-group-admin-token");

    request.setQueryParam("limit", options?.limit);
    request.setQueryParam("iterator", options?.iterator);
    request.setQueryParam("order", options?.order);

    return request.send(
      this.requestCtx,
      ListResponseGlobalApiTokenCensoredOutSerializer._fromJsonObject
    );
  }

  /** Create a new auth token that can be used for all operations that aren't downstream for a app_id (so no apps, endpoints, messages, attempts, etc) */
  public create(
    createTokenIn: CreateTokenIn,
    options?: AuthenticationOrgGroupAdminTokenCreateOptions
  ): Promise<ApiTokenOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/auth/org-group-admin-token"
    );

    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(CreateTokenInSerializer._toJsonObject(createTokenIn));

    return request.send(this.requestCtx, ApiTokenOutSerializer._fromJsonObject);
  }

  /** Get the organization's settings (with dashboard-only fields exposed). */
  public update(
    keyId: string,
    apiTokenUpdate: ApiTokenUpdate
  ): Promise<GlobalApiTokenCensoredOut> {
    const request = new SvixRequest(
      HttpMethod.PUT,
      "/api/v1/auth/org-group-admin-token/{key_id}"
    );

    request.setPathParam("key_id", keyId);
    request.setBody(ApiTokenUpdateSerializer._toJsonObject(apiTokenUpdate));

    return request.send(
      this.requestCtx,
      GlobalApiTokenCensoredOutSerializer._fromJsonObject
    );
  }

  /** Expire the selected API Token. */
  public expire(
    keyId: string,
    apiTokenExpireIn: ApiTokenExpireIn,
    options?: AuthenticationOrgGroupAdminTokenExpireOptions
  ): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/auth/org-group-admin-token/{key_id}/expire"
    );

    request.setPathParam("key_id", keyId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(ApiTokenExpireInSerializer._toJsonObject(apiTokenExpireIn));

    return request.sendNoResponseBody(this.requestCtx);
  }
}
