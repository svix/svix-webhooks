// this file is @generated

export interface EndpointTransformationOut {
  code?: string | null;
  enabled?: boolean;
  variables?: { [key: string]: string } | null;
  updatedAt?: Date | null;
}

export const EndpointTransformationOutSerializer = {
  _fromJsonObject(object: any): EndpointTransformationOut {
    return {
      code: object["code"],
      enabled: object["enabled"],
      variables: object["variables"],
      updatedAt: object["updatedAt"] ? new Date(object["updatedAt"]) : null,
    };
  },

  _toJsonObject(self: EndpointTransformationOut): any {
    return {
      code: self.code,
      enabled: self.enabled,
      variables: self.variables,
      updatedAt: self.updatedAt,
    };
  },
};
