// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface RotatePollerTokenIn {
  /** How long the token will be valid for, in seconds. Can be up to 31,536,000 seconds (1 year). */
  expiry?: number | null;
  /**
   * Updates the previous token's expiration, in seconds.
   *
   * If set to 0, the old token will immediately be revoked. Must be between 0 and 86,400 seconds (1 day).
   *
   * Defaults to 300 seconds (5 minutes).
   */
  oldTokenExpiry?: number;
}

export const RotatePollerTokenInSerializer = {
  _fromJsonObject(object: any): RotatePollerTokenIn {
    return {
      expiry: object["expiry"],
      oldTokenExpiry: object["oldTokenExpiry"],
    };
  },

  _toJsonObject(self: RotatePollerTokenIn): any {
    return {
      expiry: self.expiry,
      oldTokenExpiry: self.oldTokenExpiry,
    };
  },
};
