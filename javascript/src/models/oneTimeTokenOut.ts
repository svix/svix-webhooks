// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface OneTimeTokenOut {
  token: string;
}

export const OneTimeTokenOutSerializer = {
  _fromJsonObject(object: any): OneTimeTokenOut {
    return {
      token: object["token"],
    };
  },

  _toJsonObject(self: OneTimeTokenOut): any {
    return {
      token: self.token,
    };
  },
};
