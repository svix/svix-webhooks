// this file is @generated

export interface ApplicationIn {
  /** Application name for human consumption. */
  name: string;
  /**
   * Maximum messages per second to send to this application.
   *
   * Outgoing messages will be throttled to this rate.
   */
  throttleRate?: number | null;
  /** Optional unique identifier for the application. */
  uid?: string | null;
  metadata?: { [key: string]: string };
}

export const ApplicationInSerializer = {
  _fromJsonObject(object: any): ApplicationIn {
    return {
      name: object["name"],
      throttleRate: object["throttleRate"],
      uid: object["uid"],
      metadata: object["metadata"],
    };
  },

  _toJsonObject(self: ApplicationIn): any {
    return {
      name: self.name,
      throttleRate: self.throttleRate,
      uid: self.uid,
      metadata: self.metadata,
    };
  },
};
