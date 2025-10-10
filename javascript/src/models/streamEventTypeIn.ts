// this file is @generated

export interface StreamEventTypeIn {
  archived?: boolean;
  deprecated?: boolean;
  description?: string | null;
  featureFlags?: string[] | null;
  /** The event type's name */
  name: string;
}

export const StreamEventTypeInSerializer = {
  _fromJsonObject(object: any): StreamEventTypeIn {
    return {
      archived: object["archived"],
      deprecated: object["deprecated"],
      description: object["description"],
      featureFlags: object["featureFlags"],
      name: object["name"],
    };
  },

  _toJsonObject(self: StreamEventTypeIn): any {
    return {
      archived: self.archived,
      deprecated: self.deprecated,
      description: self.description,
      featureFlags: self.featureFlags,
      name: self.name,
    };
  },
};
