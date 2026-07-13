// this file is @generated

export interface StreamEventTypePatch {
  description?: string | null;
  featureFlags?: string[] | null;
  deprecated?: boolean;
  archived?: boolean;
}

export const StreamEventTypePatchSerializer = {
  _fromJsonObject(object: any): StreamEventTypePatch {
    return {
      description: object["description"],
      featureFlags: object["featureFlags"],
      deprecated: object["deprecated"],
      archived: object["archived"],
    };
  },

  _toJsonObject(self: StreamEventTypePatch): any {
    return {
      description: self.description,
      featureFlags: self.featureFlags,
      deprecated: self.deprecated,
      archived: self.archived,
    };
  },
};
