// this file is @generated

export interface EventTypePatch {
  description?: string;
  archived?: boolean;
  deprecated?: boolean;
  schemas?: any | null;
  featureFlags?: string[] | null;
  /** The event type group's name */
  groupName?: string | null;
}

export const EventTypePatchSerializer = {
  _fromJsonObject(object: any): EventTypePatch {
    return {
      description: object["description"],
      archived: object["archived"],
      deprecated: object["deprecated"],
      schemas: object["schemas"],
      featureFlags: object["featureFlags"],
      groupName: object["groupName"],
    };
  },

  _toJsonObject(self: EventTypePatch): any {
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
