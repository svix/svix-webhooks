// this file is @generated

export interface EventTypeFromOpenApi {
  /** The event type's name */
  name: string;
  description: string;
  schemas?: any | null;
  deprecated: boolean;
  /** The event type group's name */
  groupName?: string | null;
  featureFlags?: string[] | null;
}

export const EventTypeFromOpenApiSerializer = {
  _fromJsonObject(object: any): EventTypeFromOpenApi {
    return {
      name: object["name"],
      description: object["description"],
      schemas: object["schemas"],
      deprecated: object["deprecated"],
      groupName: object["groupName"],
      featureFlags: object["featureFlags"],
    };
  },

  _toJsonObject(self: EventTypeFromOpenApi): any {
    return {
      name: self.name,
      description: self.description,
      schemas: self.schemas,
      deprecated: self.deprecated,
      groupName: self.groupName,
      featureFlags: self.featureFlags,
    };
  },
};
