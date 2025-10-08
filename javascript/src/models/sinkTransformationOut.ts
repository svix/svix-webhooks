// this file is @generated

export interface SinkTransformationOut {
  code?: string | null;
  enabled: boolean;
}

export const SinkTransformationOutSerializer = {
  _fromJsonObject(object: any): SinkTransformationOut {
    return {
      code: object["code"],
      enabled: object["enabled"],
    };
  },

  _toJsonObject(self: SinkTransformationOut): any {
    return {
      code: self.code,
      enabled: self.enabled,
    };
  },
};
