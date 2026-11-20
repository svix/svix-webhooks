// this file is @generated
import {
  type ClientSecretJwtParamsIn,
  ClientSecretJwtParamsInSerializer,
} from "./clientSecretJwtParamsIn";
import {
  type Oauth2AuthMethodInOut,
  Oauth2AuthMethodInOutSerializer,
} from "./oauth2AuthMethodInOut";
import {
  type Oauth2GrantTypeInOut,
  Oauth2GrantTypeInOutSerializer,
} from "./oauth2GrantTypeInOut";

export interface EndpointOauthConfigIn {
  /** The OAuth grant type. */
  grantType: Oauth2GrantTypeInOut;
  authMethod: Oauth2AuthMethodInOut;
  /** Optional JWT parameters. Only required for `clientSecretJwt` */
  jwtParams?: ClientSecretJwtParamsIn | null;
  /** The client ID. Required for all authentication types. */
  clientId: string;
  /**
   * Optional client secret. This is only used for `clientSecretBasic` and `clientSecretPost`.
   *
   * For `clientSecretBasic`, the secret will be appended to the `Authorization` header.
   * For `clientSecretPost`, this will be added to the body in a `client_secret` parameter.
   */
  clientSecret?: string | null;
  /** The URL of the authorization server. */
  tokenUrl: string;
  /** Optional OAuth scopes added to the request body. */
  scopes?: string[] | null;
  /** Extra parameters added to the request body as key-value pairs. */
  extraParams?: { [key: string]: string } | null;
  /** For `refreshToken` grant type. */
  refreshToken?: string | null;
  /** Optional headers added to the token request. */
  extraHeaders?: { [key: string]: string } | null;
}

export const EndpointOauthConfigInSerializer = {
  _fromJsonObject(object: any): EndpointOauthConfigIn {
    return {
      grantType: Oauth2GrantTypeInOutSerializer._fromJsonObject(object["grantType"]),
      authMethod: Oauth2AuthMethodInOutSerializer._fromJsonObject(object["authMethod"]),
      jwtParams:
        object["jwtParams"] != null
          ? ClientSecretJwtParamsInSerializer._fromJsonObject(object["jwtParams"])
          : undefined,
      clientId: object["clientId"],
      clientSecret: object["clientSecret"],
      tokenUrl: object["tokenUrl"],
      scopes: object["scopes"],
      extraParams: object["extraParams"],
      refreshToken: object["refreshToken"],
      extraHeaders: object["extraHeaders"],
    };
  },

  _toJsonObject(self: EndpointOauthConfigIn): any {
    return {
      grantType: Oauth2GrantTypeInOutSerializer._toJsonObject(self.grantType),
      authMethod: Oauth2AuthMethodInOutSerializer._toJsonObject(self.authMethod),
      jwtParams:
        self.jwtParams != null
          ? ClientSecretJwtParamsInSerializer._toJsonObject(self.jwtParams)
          : undefined,
      clientId: self.clientId,
      clientSecret: self.clientSecret,
      tokenUrl: self.tokenUrl,
      scopes: self.scopes,
      extraParams: self.extraParams,
      refreshToken: self.refreshToken,
      extraHeaders: self.extraHeaders,
    };
  },
};
