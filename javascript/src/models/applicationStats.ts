// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface ApplicationStats {
  /** The Application's ID. */
  appId: string;
  /** The Application's UID. */
  appUid?: string | null;
  messageDestinations: number;
}

export const ApplicationStatsSerializer = {
  _fromJsonObject(object: any): ApplicationStats {
    return {
      appId: object["appId"],
      appUid: object["appUid"],
      messageDestinations: object["messageDestinations"],
    };
  },

  _toJsonObject(self: ApplicationStats): any {
    return {
      appId: self.appId,
      appUid: self.appUid,
      messageDestinations: self.messageDestinations,
    };
  },
};
