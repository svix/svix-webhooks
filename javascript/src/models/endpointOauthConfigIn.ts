// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import {
  ClientSecretJwtParamsIn,
  ClientSecretJwtParamsInSerializer,
} from "./clientSecretJwtParamsIn";
import {
  Oauth2AuthMethodInOut,
  Oauth2AuthMethodInOutSerializer,
} from "./oauth2AuthMethodInOut";
import {
  Oauth2GrantTypeInOut,
  Oauth2GrantTypeInOutSerializer,
} from "./oauth2GrantTypeInOut";

export interface EndpointOauthConfigIn {
  authMethod: Oauth2AuthMethodInOut;
  /** The client ID. Required for all authentication types. */
  clientId: string;
  /**
   * Optional client secret. This is only used for `clientSecretBasic` and `clientSecretPost`.
   *
   * For `clientSecretBasic`, the secret will be appended to the `Authorization` header. For `clientSecretPost`, this will be added to the body in a `client_secret` parameter.
   */
  clientSecret?: string | null;
  /** Extra parameters added to the request body as key-value pairs. */
  extraParams?: { [key: string]: string } | null;
  /** The OAuth grant type. */
  grantType: Oauth2GrantTypeInOut;
  /** Optional JWT parameters. Only required for `clientSecretJwt` */
  jwtParams?: ClientSecretJwtParamsIn | null;
  /** For `refreshToken` grant type. */
  refreshToken?: string | null;
  /** Optional OAuth scopes added to the request body. */
  scopes?: string[] | null;
  /** The URL of the authorization server. */
  tokenUrl: string;
}

export const EndpointOauthConfigInSerializer = {
  _fromJsonObject(object: any): EndpointOauthConfigIn {
    return {
      authMethod: Oauth2AuthMethodInOutSerializer._fromJsonObject(object["authMethod"]),
      clientId: object["clientId"],
      clientSecret: object["clientSecret"],
      extraParams: object["extraParams"],
      grantType: Oauth2GrantTypeInOutSerializer._fromJsonObject(object["grantType"]),
      jwtParams: object["jwtParams"]
        ? ClientSecretJwtParamsInSerializer._fromJsonObject(object["jwtParams"])
        : undefined,
      refreshToken: object["refreshToken"],
      scopes: object["scopes"],
      tokenUrl: object["tokenUrl"],
    };
  },

  _toJsonObject(self: EndpointOauthConfigIn): any {
    return {
      authMethod: Oauth2AuthMethodInOutSerializer._toJsonObject(self.authMethod),
      clientId: self.clientId,
      clientSecret: self.clientSecret,
      extraParams: self.extraParams,
      grantType: Oauth2GrantTypeInOutSerializer._toJsonObject(self.grantType),
      jwtParams: self.jwtParams
        ? ClientSecretJwtParamsInSerializer._toJsonObject(self.jwtParams)
        : undefined,
      refreshToken: self.refreshToken,
      scopes: self.scopes,
      tokenUrl: self.tokenUrl,
    };
  },
};
