// this file is @generated

export interface StreamEventTypeOut {
  /** The event type's name */
  name: string;
  description?: string | null;
  createdAt: Date;
  updatedAt: Date;
  deprecated: boolean;
  archived: boolean;
  featureFlags?: string[] | null;
}

export const StreamEventTypeOutSerializer = {
  _fromJsonObject(object: any): StreamEventTypeOut {
    return {
      name: object["name"],
      description: object["description"],
      createdAt: new Date(object["createdAt"]),
      updatedAt: new Date(object["updatedAt"]),
      deprecated: object["deprecated"],
      archived: object["archived"],
      featureFlags: object["featureFlags"],
    };
  },

  _toJsonObject(self: StreamEventTypeOut): any {
    return {
      name: self.name,
      description: self.description,
      createdAt: self.createdAt,
      updatedAt: self.updatedAt,
      deprecated: self.deprecated,
      archived: self.archived,
      featureFlags: self.featureFlags,
    };
  },
};
