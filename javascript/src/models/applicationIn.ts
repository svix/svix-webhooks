// this file is @generated

export interface ApplicationIn {
  metadata?: { [key: string]: string };
  /** Application name for human consumption. */
  name: string;
  /**
   * Deprecated, use `throttleRate` instead.
   *
   * @deprecated
   */
  rateLimit?: number | null;
  /**
   * Maximum messages per second to send to this application.
   *
   * Outgoing messages will be throttled to this rate.
   */
  throttleRate?: number | null;
  /** Optional unique identifier for the application. */
  uid?: string | null;
}

export const ApplicationInSerializer = {
  _fromJsonObject(object: any): ApplicationIn {
    return {
      metadata: object["metadata"],
      name: object["name"],
      rateLimit: object["rateLimit"],
      throttleRate: object["throttleRate"],
      uid: object["uid"],
    };
  },

  _toJsonObject(self: ApplicationIn): any {
    return {
      metadata: self.metadata,
      name: self.name,
      rateLimit: self.rateLimit,
      throttleRate: self.throttleRate,
      uid: self.uid,
    };
  },
};
