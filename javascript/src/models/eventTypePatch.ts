// this file is @generated

export interface EventTypePatch {
  archived?: boolean;
  deprecated?: boolean;
  description?: string;
  featureFlags?: string[] | null;
  /** The event type group's name */
  groupName?: string | null;
  schemas?: any | null;
}

export const EventTypePatchSerializer = {
  _fromJsonObject(object: any): EventTypePatch {
    return {
      archived: object["archived"],
      deprecated: object["deprecated"],
      description: object["description"],
      featureFlags: object["featureFlags"],
      groupName: object["groupName"],
      schemas: object["schemas"],
    };
  },

  _toJsonObject(self: EventTypePatch): any {
    return {
      archived: self.archived,
      deprecated: self.deprecated,
      description: self.description,
      featureFlags: self.featureFlags,
      groupName: self.groupName,
      schemas: self.schemas,
    };
  },
};
