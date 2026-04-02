// this file is @generated

export interface MetaConfig {
  secret: string;
  verifyToken: string;
}

export const MetaConfigSerializer = {
  _fromJsonObject(object: any): MetaConfig {
    return {
      secret: object["secret"],
      verifyToken: object["verifyToken"],
    };
  },

  _toJsonObject(self: MetaConfig): any {
    return {
      secret: self.secret,
      verifyToken: self.verifyToken,
    };
  },
};
