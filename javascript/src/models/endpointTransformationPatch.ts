// this file is @generated

export interface EndpointTransformationPatch {
  code?: string | null;
  enabled?: boolean;
  variables?: { [key: string]: string } | null;
}

export const EndpointTransformationPatchSerializer = {
  _fromJsonObject(object: any): EndpointTransformationPatch {
    return {
      code: object["code"],
      enabled: object["enabled"],
      variables: object["variables"],
    };
  },

  _toJsonObject(self: EndpointTransformationPatch): any {
    return {
      code: self.code,
      enabled: self.enabled,
      variables: self.variables,
    };
  },
};
