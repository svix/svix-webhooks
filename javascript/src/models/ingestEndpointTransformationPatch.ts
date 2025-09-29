// this file is @generated

export interface IngestEndpointTransformationPatch {
  code?: string | null;
  enabled?: boolean;
}

export const IngestEndpointTransformationPatchSerializer = {
  _fromJsonObject(object: any): IngestEndpointTransformationPatch {
    return {
      code: object["code"],
      enabled: object["enabled"],
    };
  },

  _toJsonObject(self: IngestEndpointTransformationPatch): any {
    return {
      code: self.code,
      enabled: self.enabled,
    };
  },
};
