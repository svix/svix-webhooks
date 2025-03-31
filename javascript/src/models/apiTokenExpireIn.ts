// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface ApiTokenExpireIn {
  /** How many seconds until the old key is expired. */
  expiry?: number;
}

export const ApiTokenExpireInSerializer = {
  _fromJsonObject(object: any): ApiTokenExpireIn {
    return {
      expiry: object["expiry"],
    };
  },

  _toJsonObject(self: ApiTokenExpireIn): any {
    return {
      expiry: self.expiry,
    };
  },
};
