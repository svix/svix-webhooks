// this file is @generated

export interface EventTypeUpsertIn {
  description: string;
  archived?: boolean;
  deprecated?: boolean;
  /** The schema for the event type for a specific version as a JSON schema. */
  schemas?: any | null;
  featureFlags?: string[] | null;
  /** The event type group's name */
  groupName?: string | null;
}

export const EventTypeUpsertInSerializer = {
  _fromJsonObject(object: any): EventTypeUpsertIn {
    return {
      description: object["description"],
      archived: object["archived"],
      deprecated: object["deprecated"],
      schemas: object["schemas"],
      featureFlags: object["featureFlags"],
      groupName: object["groupName"],
    };
  },

  _toJsonObject(self: EventTypeUpsertIn): any {
    return {
      description: self.description,
      archived: self.archived,
      deprecated: self.deprecated,
      schemas: self.schemas,
      featureFlags: self.featureFlags,
      groupName: self.groupName,
    };
  },
};
