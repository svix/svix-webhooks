// this file is @generated

export interface StreamEventTypeOut {
  archived: boolean;
  createdAt: Date;
  deprecated: boolean;
  description?: string | null;
  featureFlags?: string[] | null;
  /** The event type's name */
  name: string;
  updatedAt: Date;
}

export const StreamEventTypeOutSerializer = {
  _fromJsonObject(object: any): StreamEventTypeOut {
    return {
      archived: object["archived"],
      createdAt: new Date(object["createdAt"]),
      deprecated: object["deprecated"],
      description: object["description"],
      featureFlags: object["featureFlags"],
      name: object["name"],
      updatedAt: new Date(object["updatedAt"]),
    };
  },

  _toJsonObject(self: StreamEventTypeOut): any {
    return {
      archived: self.archived,
      createdAt: self.createdAt,
      deprecated: self.deprecated,
      description: self.description,
      featureFlags: self.featureFlags,
      name: self.name,
      updatedAt: self.updatedAt,
    };
  },
};
