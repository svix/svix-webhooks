// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

/** Sent when an endpoint has been enabled. */
export interface EndpointEnabledEventData {
  /** The Application's ID. */
  appId: string;
  /** The Application's UID. */
  appUid?: string | null;
  /** The Endpoint's ID. */
  endpointId: string;
  /** The Endpoint's UID. */
  endpointUid?: string | null;
}

export const EndpointEnabledEventDataSerializer = {
  _fromJsonObject(object: any): EndpointEnabledEventData {
    return {
      appId: object["appId"],
      appUid: object["appUid"],
      endpointId: object["endpointId"],
      endpointUid: object["endpointUid"],
    };
  },

  _toJsonObject(self: EndpointEnabledEventData): any {
    return {
      appId: self.appId,
      appUid: self.appUid,
      endpointId: self.endpointId,
      endpointUid: self.endpointUid,
    };
  },
};
