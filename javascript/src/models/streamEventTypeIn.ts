// this file is @generated

export interface StreamEventTypeIn {
  /** The event type's name */
  name: string;
  description?: string | null;
  featureFlags?: string[] | null;
  deprecated?: boolean;
  archived?: boolean;
}

export const StreamEventTypeInSerializer = {
  _fromJsonObject(object: any): StreamEventTypeIn {
    return {
      name: object["name"],
      description: object["description"],
      featureFlags: object["featureFlags"],
      deprecated: object["deprecated"],
      archived: object["archived"],
    };
  },

  _toJsonObject(self: StreamEventTypeIn): any {
    return {
      name: self.name,
      description: self.description,
      featureFlags: self.featureFlags,
      deprecated: self.deprecated,
      archived: self.archived,
    };
  },
};
