// this file is @generated

export interface NangoConfig {
  secret: string;
}

export const NangoConfigSerializer = {
  _fromJsonObject(object: any): NangoConfig {
    return {
      secret: object["secret"],
    };
  },

  _toJsonObject(self: NangoConfig): any {
    return {
      secret: self.secret,
    };
  },
};
