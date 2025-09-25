// this file is @generated

export interface TelnyxConfigOut {
  publicKey: string;
}

export const TelnyxConfigOutSerializer = {
  _fromJsonObject(object: any): TelnyxConfigOut {
    return {
      publicKey: object["publicKey"],
    };
  },

  _toJsonObject(self: TelnyxConfigOut): any {
    return {
      publicKey: self.publicKey,
    };
  },
};
