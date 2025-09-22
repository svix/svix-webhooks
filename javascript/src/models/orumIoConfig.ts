// this file is @generated

export interface OrumIoConfig {
  publicKey: string;
}

export const OrumIoConfigSerializer = {
  _fromJsonObject(object: any): OrumIoConfig {
    return {
      publicKey: object["publicKey"],
    };
  },

  _toJsonObject(self: OrumIoConfig): any {
    return {
      publicKey: self.publicKey,
    };
  },
};
