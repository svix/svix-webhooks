// this file is @generated

export interface IntegrationIn {
  name: string;
  /** The set of feature flags the integration will have access to. */
  featureFlags?: string[];
}

export const IntegrationInSerializer = {
  _fromJsonObject(object: any): IntegrationIn {
    return {
      name: object["name"],
      featureFlags: object["featureFlags"],
    };
  },

  _toJsonObject(self: IntegrationIn): any {
    return {
      name: self.name,
      featureFlags: self.featureFlags,
    };
  },
};
