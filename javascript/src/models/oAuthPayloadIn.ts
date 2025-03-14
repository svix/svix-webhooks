// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface OAuthPayloadIn {
  code: string;
  redirectUri: string;
}

export const OAuthPayloadInSerializer = {
  _fromJsonObject(object: any): OAuthPayloadIn {
    return {
      code: object["code"],
      redirectUri: object["redirectUri"],
    };
  },

  _toJsonObject(self: OAuthPayloadIn): any {
    return {
      code: self.code,
      redirectUri: self.redirectUri,
    };
  },
};
