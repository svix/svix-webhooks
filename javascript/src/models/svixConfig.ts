// this file is @generated

export interface SvixConfig {
  secret: string;
}

export const SvixConfigSerializer = {
  _fromJsonObject(object: any): SvixConfig {
    return {
      secret: object["secret"],
    };
  },

  _toJsonObject(self: SvixConfig): any {
    return {
      secret: self.secret,
    };
  },
};
