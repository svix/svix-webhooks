// this file is @generated

export interface RutterConfig {
  secret: string;
}

export const RutterConfigSerializer = {
  _fromJsonObject(object: any): RutterConfig {
    return {
      secret: object["secret"],
    };
  },

  _toJsonObject(self: RutterConfig): any {
    return {
      secret: self.secret,
    };
  },
};
