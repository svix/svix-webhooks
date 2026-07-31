// this file is @generated

export interface IntegrationOut {
  name: string;
  /** The Integration's ID. */
  id: string;
  createdAt: Date;
  updatedAt: Date;
  /** The set of feature flags the integration has access to. */
  featureFlags?: string[];
}

export const IntegrationOutSerializer = {
  _fromJsonObject(object: any): IntegrationOut {
    return {
      name: object["name"],
      id: object["id"],
      createdAt: new Date(object["createdAt"]),
      updatedAt: new Date(object["updatedAt"]),
      featureFlags: object["featureFlags"],
    };
  },

  _toJsonObject(self: IntegrationOut): any {
    return {
      name: self.name,
      id: self.id,
      createdAt: self.createdAt,
      updatedAt: self.updatedAt,
      featureFlags: self.featureFlags,
    };
  },
};
