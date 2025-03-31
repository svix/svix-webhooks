// this file is @generated
import { ApiTokenExpireIn, ApiTokenExpireInSerializer } from "../models/apiTokenExpireIn";
import { ApiTokenIn, ApiTokenInSerializer } from "../models/apiTokenIn";
import { ApiTokenOut, ApiTokenOutSerializer } from "../models/apiTokenOut";
import {
  ListResponseApiTokenCensoredOut,
  ListResponseApiTokenCensoredOutSerializer,
} from "../models/listResponseApiTokenCensoredOut";
import { Ordering } from "../models/ordering";
import { HttpMethod, SvixRequest, SvixRequestContext } from "../request";

export interface ManagementAuthenticationListApiTokensOptions {
  /** Limit the number of returned items */
  limit?: number;
  /** The iterator returned from a prior invocation */
  iterator?: string | null;
  /** The sorting order of the returned items */
  order?: Ordering;
}

export interface ManagementAuthenticationCreateApiTokenOptions {
  idempotencyKey?: string;
}

export interface ManagementAuthenticationExpireApiTokenOptions {
  idempotencyKey?: string;
}

export class ManagementAuthentication {
  public constructor(private readonly requestCtx: SvixRequestContext) {}

  /** List all API Tokens. */
  public listApiTokens(
    options?: ManagementAuthenticationListApiTokensOptions
  ): Promise<ListResponseApiTokenCensoredOut> {
    const request = new SvixRequest(
      HttpMethod.GET,
      "/api/v1/management/authentication/api-token"
    );

    request.setQueryParam("limit", options?.limit);
    request.setQueryParam("iterator", options?.iterator);
    request.setQueryParam("order", options?.order);

    return request.send(
      this.requestCtx,
      ListResponseApiTokenCensoredOutSerializer._fromJsonObject
    );
  }

  /** Create a new API Token. */
  public createApiToken(
    apiTokenIn: ApiTokenIn,
    options?: ManagementAuthenticationCreateApiTokenOptions
  ): Promise<ApiTokenOut> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/management/authentication/api-token"
    );

    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(ApiTokenInSerializer._toJsonObject(apiTokenIn));

    return request.send(this.requestCtx, ApiTokenOutSerializer._fromJsonObject);
  }

  /** Expire the selected API Token. */
  public expireApiToken(
    keyId: string,
    apiTokenExpireIn: ApiTokenExpireIn,
    options?: ManagementAuthenticationExpireApiTokenOptions
  ): Promise<void> {
    const request = new SvixRequest(
      HttpMethod.POST,
      "/api/v1/management/authentication/api-token/{key_id}/expire"
    );

    request.setPathParam("key_id", keyId);
    request.setHeaderParam("idempotency-key", options?.idempotencyKey);
    request.setBody(ApiTokenExpireInSerializer._toJsonObject(apiTokenExpireIn));

    return request.sendNoResponseBody(this.requestCtx);
  }
}
