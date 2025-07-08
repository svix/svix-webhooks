// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface ApplicationTokenExpireIn {
  /** How many seconds until the old key is expired. */
  expiry?: number | null;
  /**
   * An optional list of session ids.
   *
   * If any session ids are specified, only Application tokens created with that session id will be expired.
   */
  sessionIds?: string[];
}

export const ApplicationTokenExpireInSerializer = {
  _fromJsonObject(object: any): ApplicationTokenExpireIn {
    return {
      expiry: object["expiry"],
      sessionIds: object["sessionIds"],
    };
  },

  _toJsonObject(self: ApplicationTokenExpireIn): any {
    return {
      expiry: self.expiry,
      sessionIds: self.sessionIds,
    };
  },
};
