// this file is @generated
import {
  type Oauth2AuthMethodInOut,
  Oauth2AuthMethodInOutSerializer,
} from "./oauth2AuthMethodInOut";
import {
  type Oauth2GrantTypeInOut,
  Oauth2GrantTypeInOutSerializer,
} from "./oauth2GrantTypeInOut";

export interface EndpointOauthConfigOut {
  grantType: Oauth2GrantTypeInOut;
  authMethod: Oauth2AuthMethodInOut;
  clientId: string;
  tokenUrl: string;
  scopes?: string[] | null;
  extraParams?: { [key: string]: string } | null;
  extraHeaders?: { [key: string]: string } | null;
}

export const EndpointOauthConfigOutSerializer = {
  _fromJsonObject(object: any): EndpointOauthConfigOut {
    return {
      grantType: Oauth2GrantTypeInOutSerializer._fromJsonObject(object["grantType"]),
      authMethod: Oauth2AuthMethodInOutSerializer._fromJsonObject(object["authMethod"]),
      clientId: object["clientId"],
      tokenUrl: object["tokenUrl"],
      scopes: object["scopes"],
      extraParams: object["extraParams"],
      extraHeaders: object["extraHeaders"],
    };
  },

  _toJsonObject(self: EndpointOauthConfigOut): any {
    return {
      grantType: Oauth2GrantTypeInOutSerializer._toJsonObject(self.grantType),
      authMethod: Oauth2AuthMethodInOutSerializer._toJsonObject(self.authMethod),
      clientId: self.clientId,
      tokenUrl: self.tokenUrl,
      scopes: self.scopes,
      extraParams: self.extraParams,
      extraHeaders: self.extraHeaders,
    };
  },
};
