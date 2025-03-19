// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import {
  Oauth2AuthMethodInOut,
  Oauth2AuthMethodInOutSerializer,
} from "./oauth2AuthMethodInOut";
import {
  Oauth2GrantTypeInOut,
  Oauth2GrantTypeInOutSerializer,
} from "./oauth2GrantTypeInOut";

export interface EndpointOauthConfigOut {
  authMethod: Oauth2AuthMethodInOut;
  clientId: string;
  extraParams?: { [key: string]: string } | null;
  grantType: Oauth2GrantTypeInOut;
  scopes?: string[] | null;
  tokenUrl: string;
}

export const EndpointOauthConfigOutSerializer = {
  _fromJsonObject(object: any): EndpointOauthConfigOut {
    return {
      authMethod: Oauth2AuthMethodInOutSerializer._fromJsonObject(object["authMethod"]),
      clientId: object["clientId"],
      extraParams: object["extraParams"],
      grantType: Oauth2GrantTypeInOutSerializer._fromJsonObject(object["grantType"]),
      scopes: object["scopes"],
      tokenUrl: object["tokenUrl"],
    };
  },

  _toJsonObject(self: EndpointOauthConfigOut): any {
    return {
      authMethod: Oauth2AuthMethodInOutSerializer._toJsonObject(self.authMethod),
      clientId: self.clientId,
      extraParams: self.extraParams,
      grantType: Oauth2GrantTypeInOutSerializer._toJsonObject(self.grantType),
      scopes: self.scopes,
      tokenUrl: self.tokenUrl,
    };
  },
};
