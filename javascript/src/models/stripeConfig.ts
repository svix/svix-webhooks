// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface StripeConfig {
  secret: string;
}

export const StripeConfigSerializer = {
  _fromJsonObject(object: any): StripeConfig {
    return {
      secret: object["secret"],
    };
  },

  _toJsonObject(self: StripeConfig): any {
    return {
      secret: self.secret,
    };
  },
};
