// this file is @generated

export interface ApplicationOut {
  createdAt: Date;
  /** The Application's ID. */
  id: string;
  metadata: { [key: string]: string };
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
  updatedAt: Date;
}

export const ApplicationOutSerializer = {
  _fromJsonObject(object: any): ApplicationOut {
    return {
      createdAt: new Date(object["createdAt"]),
      id: object["id"],
      metadata: object["metadata"],
      name: object["name"],
      rateLimit: object["rateLimit"],
      throttleRate: object["throttleRate"],
      uid: object["uid"],
      updatedAt: new Date(object["updatedAt"]),
    };
  },

  _toJsonObject(self: ApplicationOut): any {
    return {
      createdAt: self.createdAt,
      id: self.id,
      metadata: self.metadata,
      name: self.name,
      rateLimit: self.rateLimit,
      throttleRate: self.throttleRate,
      uid: self.uid,
      updatedAt: self.updatedAt,
    };
  },
};
