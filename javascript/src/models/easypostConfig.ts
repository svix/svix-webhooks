// this file is @generated

export interface EasypostConfig {
  secret?: string | null;
}

export const EasypostConfigSerializer = {
  _fromJsonObject(object: any): EasypostConfig {
    return {
      secret: object["secret"],
    };
  },

  _toJsonObject(self: EasypostConfig): any {
    return {
      secret: self.secret,
    };
  },
};
