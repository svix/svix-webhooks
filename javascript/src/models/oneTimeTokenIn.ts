// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface OneTimeTokenIn {
  oneTimeToken: string;
}

export const OneTimeTokenInSerializer = {
  _fromJsonObject(object: any): OneTimeTokenIn {
    return {
      oneTimeToken: object["oneTimeToken"],
    };
  },

  _toJsonObject(self: OneTimeTokenIn): any {
    return {
      oneTimeToken: self.oneTimeToken,
    };
  },
};
