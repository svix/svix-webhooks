// this file is @generated

export interface DestinationTransformationOut {
  code?: string | null;
  enabled: boolean;
}

export const DestinationTransformationOutSerializer = {
  _fromJsonObject(object: any): DestinationTransformationOut {
    return {
      code: object["code"],
      enabled: object["enabled"],
    };
  },

  _toJsonObject(self: DestinationTransformationOut): any {
    return {
      code: self.code,
      enabled: self.enabled,
    };
  },
};
