// this file is @generated

export interface AscendConfig {
  secret: string;
}

export const AscendConfigSerializer = {
  _fromJsonObject(object: any): AscendConfig {
    return {
      secret: object["secret"],
    };
  },

  _toJsonObject(self: AscendConfig): any {
    return {
      secret: self.secret,
    };
  },
};
