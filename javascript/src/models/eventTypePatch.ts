// this file is @generated

export interface EventTypePatch {
  description?: string;
  /** Translated descriptions keyed by BCP 47 language tag. */
  descriptionTranslations?: { [key: string]: string } | null;
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
      descriptionTranslations: object["descriptionTranslations"],
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
      descriptionTranslations: self.descriptionTranslations,
      archived: self.archived,
      deprecated: self.deprecated,
      schemas: self.schemas,
      featureFlags: self.featureFlags,
      groupName: self.groupName,
    };
  },
};
