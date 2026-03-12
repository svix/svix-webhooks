// this file is @generated

export interface EventTypeUpdate {
  archived?: boolean;
  deprecated?: boolean;
  description: string;
  featureFlag?: string | null;
  featureFlags?: string[] | null;
  /** The event type group's name */
  groupName?: string | null;
  /** The schema for the event type for a specific version as a JSON schema. */
  schemas?: any | null;
}

export const EventTypeUpdateSerializer = {
  _fromJsonObject(object: any): EventTypeUpdate {
    return {
      archived: object["archived"],
      deprecated: object["deprecated"],
      description: object["description"],
      featureFlag: object["featureFlag"],
      featureFlags: object["featureFlags"],
      groupName: object["groupName"],
      schemas: object["schemas"],
    };
  },

  _toJsonObject(self: EventTypeUpdate): any {
    return {
      archived: self.archived,
      deprecated: self.deprecated,
      description: self.description,
      featureFlag: self.featureFlag,
      featureFlags: self.featureFlags,
      groupName: self.groupName,
      schemas: self.schemas,
    };
  },
};
