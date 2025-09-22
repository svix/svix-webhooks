// this file is @generated

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
