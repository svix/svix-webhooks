// this file is @generated

export interface ApplicationIn {
  metadata?: { [key: string]: string };
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
}

export const ApplicationInSerializer = {
  _fromJsonObject(object: any): ApplicationIn {
    return {
      metadata: object["metadata"],
      name: object["name"],
      throttleRate: object["throttleRate"],
      uid: object["uid"],
    };
  },

  _toJsonObject(self: ApplicationIn): any {
    return {
      metadata: self.metadata,
      name: self.name,
      throttleRate: self.throttleRate,
      uid: self.uid,
    };
  },
};
