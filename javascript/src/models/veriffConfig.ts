// this file is @generated

export interface VeriffConfig {
  secret: string;
}

export const VeriffConfigSerializer = {
  _fromJsonObject(object: any): VeriffConfig {
    return {
      secret: object["secret"],
    };
  },

  _toJsonObject(self: VeriffConfig): any {
    return {
      secret: self.secret,
    };
  },
};
