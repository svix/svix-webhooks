// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export enum Oauth2GrantTypeInOut {
  ClientCredentials = "clientCredentials",
  RefreshToken = "refreshToken",
}

export const Oauth2GrantTypeInOutSerializer = {
  _fromJsonObject(object: any): Oauth2GrantTypeInOut {
    return object;
  },

  _toJsonObject(self: Oauth2GrantTypeInOut): any {
    return self;
  },
};
