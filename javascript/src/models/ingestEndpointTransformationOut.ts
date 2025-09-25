// this file is @generated

export interface IngestEndpointTransformationOut {
  code?: string | null;
  enabled?: boolean;
}

export const IngestEndpointTransformationOutSerializer = {
  _fromJsonObject(object: any): IngestEndpointTransformationOut {
    return {
      code: object["code"],
      enabled: object["enabled"],
    };
  },

  _toJsonObject(self: IngestEndpointTransformationOut): any {
    return {
      code: self.code,
      enabled: self.enabled,
    };
  },
};
