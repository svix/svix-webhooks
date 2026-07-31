// this file is @generated

export interface IntegrationUpdate {
  name: string;
  /** The set of feature flags the integration will have access to. */
  featureFlags?: string[];
}

export const IntegrationUpdateSerializer = {
  _fromJsonObject(object: any): IntegrationUpdate {
    return {
      name: object["name"],
      featureFlags: object["featureFlags"],
    };
  },

  _toJsonObject(self: IntegrationUpdate): any {
    return {
      name: self.name,
      featureFlags: self.featureFlags,
    };
  },
};
