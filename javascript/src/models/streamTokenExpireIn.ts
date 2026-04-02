// this file is @generated

export interface StreamTokenExpireIn {
  /** How many seconds until the old key is expired. */
  expiry?: number | null;
  /**
   * An optional list of session ids.
   *
   * If any session ids are specified, only Stream tokens created with that session id will be expired.
   */
  sessionIds?: string[];
}

export const StreamTokenExpireInSerializer = {
  _fromJsonObject(object: any): StreamTokenExpireIn {
    return {
      expiry: object["expiry"],
      sessionIds: object["sessionIds"],
    };
  },

  _toJsonObject(self: StreamTokenExpireIn): any {
    return {
      expiry: self.expiry,
      sessionIds: self.sessionIds,
    };
  },
};
