// this file is @generated

/** Sent when an endpoint is created, updated, or deleted */
export interface EndpointCreatedEventData {
  /** The Application's ID. */
  appId: string;
  /** The Application's UID. */
  appUid?: string | null;
  /** The Endpoint's ID. */
  endpointId: string;
  /** The Endpoint's UID. */
  endpointUid?: string | null;
}

export const EndpointCreatedEventDataSerializer = {
  _fromJsonObject(object: any): EndpointCreatedEventData {
    return {
      appId: object["appId"],
      appUid: object["appUid"],
      endpointId: object["endpointId"],
      endpointUid: object["endpointUid"],
    };
  },

  _toJsonObject(self: EndpointCreatedEventData): any {
    return {
      appId: self.appId,
      appUid: self.appUid,
      endpointId: self.endpointId,
      endpointUid: self.endpointUid,
    };
  },
};
