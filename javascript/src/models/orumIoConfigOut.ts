// this file is @generated

export interface OrumIoConfigOut {
  publicKey: string;
}

export const OrumIoConfigOutSerializer = {
  _fromJsonObject(object: any): OrumIoConfigOut {
    return {
      publicKey: object["publicKey"],
    };
  },

  _toJsonObject(self: OrumIoConfigOut): any {
    return {
      publicKey: self.publicKey,
    };
  },
};
