// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

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
