// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface StreamPortalAccessIn {
  /**
   * How long the token will be valid for, in seconds.
   *
   * Valid values are between 1 hour and 7 days. The default is 7 days.
   */
  expiry?: number | null;
  /** The set of feature flags the created token will have access to. */
  featureFlags?: string[];
  /** Whether the app portal should be in read-only mode. */
  readOnly?: boolean | null;
}

export const StreamPortalAccessInSerializer = {
  _fromJsonObject(object: any): StreamPortalAccessIn {
    return {
      expiry: object["expiry"],
      featureFlags: object["featureFlags"],
      readOnly: object["readOnly"],
    };
  },

  _toJsonObject(self: StreamPortalAccessIn): any {
    return {
      expiry: self.expiry,
      featureFlags: self.featureFlags,
      readOnly: self.readOnly,
    };
  },
};
