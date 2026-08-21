// this file is @generated

export interface IngestEndpointTransformationOut {
  code?: string | null;
  enabled?: boolean;
  variables?: { [key: string]: string } | null;
}

export const IngestEndpointTransformationOutSerializer = {
  _fromJsonObject(object: any): IngestEndpointTransformationOut {
    return {
      code: object["code"],
      enabled: object["enabled"],
      variables: object["variables"],
    };
  },

  _toJsonObject(self: IngestEndpointTransformationOut): any {
    return {
      code: self.code,
      enabled: self.enabled,
      variables: self.variables,
    };
  },
};
