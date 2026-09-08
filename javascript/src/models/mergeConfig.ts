// this file is @generated

export interface MergeConfig {
  secret: string;
}

export const MergeConfigSerializer = {
  _fromJsonObject(object: any): MergeConfig {
    return {
      secret: object["secret"],
    };
  },

  _toJsonObject(self: MergeConfig): any {
    return {
      secret: self.secret,
    };
  },
};
