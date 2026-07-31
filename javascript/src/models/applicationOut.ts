// this file is @generated

export interface ApplicationOut {
  /** Optional unique identifier for the application. */
  uid?: string | null;
  /** Application name for human consumption. */
  name: string;
  /**
   * Maximum messages per second to send to this application.
   *
   * Outgoing messages will be throttled to this rate.
   */
  throttleRate?: number | null;
  /** The Application's ID. */
  id: string;
  createdAt: Date;
  updatedAt: Date;
  metadata: { [key: string]: string };
}

export const ApplicationOutSerializer = {
  _fromJsonObject(object: any): ApplicationOut {
    return {
      uid: object["uid"],
      name: object["name"],
      throttleRate: object["throttleRate"],
      id: object["id"],
      createdAt: new Date(object["createdAt"]),
      updatedAt: new Date(object["updatedAt"]),
      metadata: object["metadata"],
    };
  },

  _toJsonObject(self: ApplicationOut): any {
    return {
      uid: self.uid,
      name: self.name,
      throttleRate: self.throttleRate,
      id: self.id,
      createdAt: self.createdAt,
      updatedAt: self.updatedAt,
      metadata: self.metadata,
    };
  },
};
