// this file is @generated

export interface StreamPortalAccessIn {
  /**
   * How long the token will be valid for, in seconds.
   *
   * Valid values are between 1 hour and 7 days. The default is 7 days.
   */
  expiry?: number | null;
  /** The set of feature flags the created token will have access to. */
  featureFlags?: string[];
  /**
   * An optional session ID to attach to the token.
   *
   * When expiring tokens with "Expire All", you can include the session ID to only expire tokens that were created with that session ID.
   */
  sessionId?: string | null;
}

export const StreamPortalAccessInSerializer = {
  _fromJsonObject(object: any): StreamPortalAccessIn {
    return {
      expiry: object["expiry"],
      featureFlags: object["featureFlags"],
      sessionId: object["sessionId"],
    };
  },

  _toJsonObject(self: StreamPortalAccessIn): any {
    return {
      expiry: self.expiry,
      featureFlags: self.featureFlags,
      sessionId: self.sessionId,
    };
  },
};
