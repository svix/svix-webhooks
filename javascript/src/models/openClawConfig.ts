// this file is @generated

export interface OpenClawConfig {
  secret: string;
}

export const OpenClawConfigSerializer = {
  _fromJsonObject(object: any): OpenClawConfig {
    return {
      secret: object["secret"],
    };
  },

  _toJsonObject(self: OpenClawConfig): any {
    return {
      secret: self.secret,
    };
  },
};
