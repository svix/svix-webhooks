// this file is @generated

export interface ApplicationPatch {
  name?: string;
  /**
   * Maximum messages per second to send to this application.
   *
   * Outgoing messages will be throttled to this rate.
   */
  throttleRate?: number | null;
  /** The Application's UID. */
  uid?: string | null;
  metadata?: { [key: string]: string };
}

export const ApplicationPatchSerializer = {
  _fromJsonObject(object: any): ApplicationPatch {
    return {
      name: object["name"],
      throttleRate: object["throttleRate"],
      uid: object["uid"],
      metadata: object["metadata"],
    };
  },

  _toJsonObject(self: ApplicationPatch): any {
    return {
      name: self.name,
      throttleRate: self.throttleRate,
      uid: self.uid,
      metadata: self.metadata,
    };
  },
};
