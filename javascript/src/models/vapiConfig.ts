// this file is @generated

export interface VapiConfig {
  secret: string;
}

export const VapiConfigSerializer = {
  _fromJsonObject(object: any): VapiConfig {
    return {
      secret: object["secret"],
    };
  },

  _toJsonObject(self: VapiConfig): any {
    return {
      secret: self.secret,
    };
  },
};
