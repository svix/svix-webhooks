// this file is @generated

export interface StreamEventTypePatch {
  archived?: boolean;
  deprecated?: boolean;
  description?: string | null;
  featureFlags?: string[] | null;
  /** The event type's name */
  name?: string | null;
}

export const StreamEventTypePatchSerializer = {
  _fromJsonObject(object: any): StreamEventTypePatch {
    return {
      archived: object["archived"],
      deprecated: object["deprecated"],
      description: object["description"],
      featureFlags: object["featureFlags"],
      name: object["name"],
    };
  },

  _toJsonObject(self: StreamEventTypePatch): any {
    return {
      archived: self.archived,
      deprecated: self.deprecated,
      description: self.description,
      featureFlags: self.featureFlags,
      name: self.name,
    };
  },
};
