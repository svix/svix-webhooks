// this file is @generated

export interface EventTypeIn {
  /** The event type's name */
  name: string;
  description: string;
  archived?: boolean;
  deprecated?: boolean;
  /** The schema for the event type for a specific version as a JSON schema. */
  schemas?: any | null;
  /** The event type group's name */
  groupName?: string | null;
  featureFlags?: string[] | null;
}

export const EventTypeInSerializer = {
  _fromJsonObject(object: any): EventTypeIn {
    return {
      name: object["name"],
      description: object["description"],
      archived: object["archived"],
      deprecated: object["deprecated"],
      schemas: object["schemas"],
      groupName: object["groupName"],
      featureFlags: object["featureFlags"],
    };
  },

  _toJsonObject(self: EventTypeIn): any {
    return {
      name: self.name,
      description: self.description,
      archived: self.archived,
      deprecated: self.deprecated,
      schemas: self.schemas,
      groupName: self.groupName,
      featureFlags: self.featureFlags,
    };
  },
};
