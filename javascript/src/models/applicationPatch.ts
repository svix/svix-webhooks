// this file is @generated

export interface ApplicationPatch {
  metadata?: { [key: string]: string };
  name?: string;
  /**
   * Maximum messages per second to send to this application.
   *
   * Outgoing messages will be throttled to this rate.
   */
  throttleRate?: number | null;
  /** The Application's UID. */
  uid?: string | null;
}

export const ApplicationPatchSerializer = {
  _fromJsonObject(object: any): ApplicationPatch {
    return {
      metadata: object["metadata"],
      name: object["name"],
      throttleRate: object["throttleRate"],
      uid: object["uid"],
    };
  },

  _toJsonObject(self: ApplicationPatch): any {
    return {
      metadata: self.metadata,
      name: self.name,
      throttleRate: self.throttleRate,
      uid: self.uid,
    };
  },
};
