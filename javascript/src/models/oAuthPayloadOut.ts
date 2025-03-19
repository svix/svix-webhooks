// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface OAuthPayloadOut {
  accessToken?: string | null;
  error?: string | null;
  refreshToken?: string | null;
}

export const OAuthPayloadOutSerializer = {
  _fromJsonObject(object: any): OAuthPayloadOut {
    return {
      accessToken: object["accessToken"],
      error: object["error"],
      refreshToken: object["refreshToken"],
    };
  },

  _toJsonObject(self: OAuthPayloadOut): any {
    return {
      accessToken: self.accessToken,
      error: self.error,
      refreshToken: self.refreshToken,
    };
  },
};
