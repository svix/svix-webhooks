// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface TelnyxConfig {
  publicKey: string;
}

export const TelnyxConfigSerializer = {
  _fromJsonObject(object: any): TelnyxConfig {
    return {
      publicKey: object["publicKey"],
    };
  },

  _toJsonObject(self: TelnyxConfig): any {
    return {
      publicKey: self.publicKey,
    };
  },
};
