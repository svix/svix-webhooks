// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

/** Sent when an endpoint is created, updated, or deleted */
export interface EndpointUpdatedEventData {
  /** The Application's ID. */
  appId: string;
  /** The Application's UID. */
  appUid?: string | null;
  /** The Endpoint's ID. */
  endpointId: string;
  /** The Endpoint's UID. */
  endpointUid?: string | null;
}

export const EndpointUpdatedEventDataSerializer = {
  _fromJsonObject(object: any): EndpointUpdatedEventData {
    return {
      appId: object["appId"],
      appUid: object["appUid"],
      endpointId: object["endpointId"],
      endpointUid: object["endpointUid"],
    };
  },

  _toJsonObject(self: EndpointUpdatedEventData): any {
    return {
      appId: self.appId,
      appUid: self.appUid,
      endpointId: self.endpointId,
      endpointUid: self.endpointUid,
    };
  },
};
