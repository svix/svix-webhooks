// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface VapiConfig {
  secret: string;
}

export const VapiConfigSerializer = {
  _fromJsonObject(object: any): VapiConfig {
    return {
      secret: object["secret"],
    };
  },

  _toJsonObject(self: VapiConfig): any {
    return {
      secret: self.secret,
    };
  },
};
