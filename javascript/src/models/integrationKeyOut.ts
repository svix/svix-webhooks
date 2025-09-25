// this file is @generated

export interface IntegrationKeyOut {
  key: string;
}

export const IntegrationKeyOutSerializer = {
  _fromJsonObject(object: any): IntegrationKeyOut {
    return {
      key: object["key"],
    };
  },

  _toJsonObject(self: IntegrationKeyOut): any {
    return {
      key: self.key,
    };
  },
};
