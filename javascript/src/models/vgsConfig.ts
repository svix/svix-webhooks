// this file is @generated

export interface VgsConfig {
  secret: string;
}

export const VgsConfigSerializer = {
  _fromJsonObject(object: any): VgsConfig {
    return {
      secret: object["secret"],
    };
  },

  _toJsonObject(self: VgsConfig): any {
    return {
      secret: self.secret,
    };
  },
};
