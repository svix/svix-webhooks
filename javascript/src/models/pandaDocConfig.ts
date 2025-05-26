// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface PandaDocConfig {
  secret: string;
}

export const PandaDocConfigSerializer = {
  _fromJsonObject(object: any): PandaDocConfig {
    return {
      secret: object["secret"],
    };
  },

  _toJsonObject(self: PandaDocConfig): any {
    return {
      secret: self.secret,
    };
  },
};
