// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

/** Sent when an endpoint is created, updated, or deleted */
export interface EndpointDeletedEventData {
  /** The Application's ID. */
  appId: string;
  /** The Application's UID. */
  appUid?: string | null;
  /** The Endpoint's ID. */
  endpointId: string;
  /** The Endpoint's UID. */
  endpointUid?: string | null;
}

export const EndpointDeletedEventDataSerializer = {
  _fromJsonObject(object: any): EndpointDeletedEventData {
    return {
      appId: object["appId"],
      appUid: object["appUid"],
      endpointId: object["endpointId"],
      endpointUid: object["endpointUid"],
    };
  },

  _toJsonObject(self: EndpointDeletedEventData): any {
    return {
      appId: self.appId,
      appUid: self.appUid,
      endpointId: self.endpointId,
      endpointUid: self.endpointUid,
    };
  },
};
