// this file is @generated

export interface CheckbookConfig {
  secret: string;
}

export const CheckbookConfigSerializer = {
  _fromJsonObject(object: any): CheckbookConfig {
    return {
      secret: object["secret"],
    };
  },

  _toJsonObject(self: CheckbookConfig): any {
    return {
      secret: self.secret,
    };
  },
};
