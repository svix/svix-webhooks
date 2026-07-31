// this file is @generated

export interface StreamPortalAccessIn {
  /** The set of feature flags the created token will have access to. */
  featureFlags?: string[];
  /**
   * How long the token will be valid for, in seconds.
   *
   * Valid values are between 1 hour and 7 days. The default is 7 days.
   */
  expiry?: number | null;
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
      featureFlags: object["featureFlags"],
      expiry: object["expiry"],
      sessionId: object["sessionId"],
    };
  },

  _toJsonObject(self: StreamPortalAccessIn): any {
    return {
      featureFlags: self.featureFlags,
      expiry: self.expiry,
      sessionId: self.sessionId,
    };
  },
};
