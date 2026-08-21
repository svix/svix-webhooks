// this file is @generated

export interface IngestEndpointTransformationPatch {
  code?: string | null;
  enabled?: boolean;
  variables?: { [key: string]: string } | null;
}

export const IngestEndpointTransformationPatchSerializer = {
  _fromJsonObject(object: any): IngestEndpointTransformationPatch {
    return {
      code: object["code"],
      enabled: object["enabled"],
      variables: object["variables"],
    };
  },

  _toJsonObject(self: IngestEndpointTransformationPatch): any {
    return {
      code: self.code,
      enabled: self.enabled,
      variables: self.variables,
    };
  },
};
