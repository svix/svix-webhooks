// this file is @generated

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
