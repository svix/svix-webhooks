// this file is @generated

export interface EventTypeOut {
  /** The event type's name */
  name: string;
  description: string;
  archived?: boolean;
  deprecated: boolean;
  /** The schema for the event type for a specific version as a JSON schema. */
  schemas?: any | null;
  createdAt: Date;
  updatedAt: Date;
  /** The event type group's name */
  groupName?: string | null;
  featureFlags?: string[] | null;
  featureFlag?: string | null;
}

export const EventTypeOutSerializer = {
  _fromJsonObject(object: any): EventTypeOut {
    return {
      name: object["name"],
      description: object["description"],
      archived: object["archived"],
      deprecated: object["deprecated"],
      schemas: object["schemas"],
      createdAt: new Date(object["createdAt"]),
      updatedAt: new Date(object["updatedAt"]),
      groupName: object["groupName"],
      featureFlags: object["featureFlags"],
      featureFlag: object["featureFlag"],
    };
  },

  _toJsonObject(self: EventTypeOut): any {
    return {
      name: self.name,
      description: self.description,
      archived: self.archived,
      deprecated: self.deprecated,
      schemas: self.schemas,
      createdAt: self.createdAt,
      updatedAt: self.updatedAt,
      groupName: self.groupName,
      featureFlags: self.featureFlags,
      featureFlag: self.featureFlag,
    };
  },
};
