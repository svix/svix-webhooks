// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface EndpointTransformationPatch {
  code?: string | null;
  enabled?: boolean;
}

export const EndpointTransformationPatchSerializer = {
  _fromJsonObject(object: any): EndpointTransformationPatch {
    return {
      code: object["code"],
      enabled: object["enabled"],
    };
  },

  _toJsonObject(self: EndpointTransformationPatch): any {
    return {
      code: self.code,
      enabled: self.enabled,
    };
  },
};
