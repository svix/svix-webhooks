// this file is @generated

export interface TricentisQtestConfig {
  secret: string;
}

export const TricentisQtestConfigSerializer = {
  _fromJsonObject(object: any): TricentisQtestConfig {
    return {
      secret: object["secret"],
    };
  },

  _toJsonObject(self: TricentisQtestConfig): any {
    return {
      secret: self.secret,
    };
  },
};
