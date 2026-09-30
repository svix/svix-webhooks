// this file is @generated

export interface CreateAutoConfigSubscriptionIn {
  /**
   * The set of feature flags the created token will have access to.
   *
   * When omitted or empty, the token inherits the calling token's feature flags.
   * When set, these flags are used instead. An application token may only grant a subset of its own flags.
   */
  featureFlags?: string[];
}

export const CreateAutoConfigSubscriptionInSerializer = {
  _fromJsonObject(object: any): CreateAutoConfigSubscriptionIn {
    return {
      featureFlags: object["featureFlags"],
    };
  },

  _toJsonObject(self: CreateAutoConfigSubscriptionIn): any {
    return {
      featureFlags: self.featureFlags,
    };
  },
};
