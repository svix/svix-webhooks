// this file is @generated

export interface EndpointPatch {
  description?: string;
  /**
   * Maximum messages per second to send to this endpoint.
   *
   * Outgoing messages will be throttled to this rate.
   */
  throttleRate?: number | null;
  /** The Endpoint's UID. */
  uid?: string | null;
  url?: string;
  disabled?: boolean;
  eventTypes?: string[] | null;
  channels?: string[] | null;
  metadata?: { [key: string]: string };
}

export const EndpointPatchSerializer = {
  _fromJsonObject(object: any): EndpointPatch {
    return {
      description: object["description"],
      throttleRate: object["throttleRate"],
      uid: object["uid"],
      url: object["url"],
      disabled: object["disabled"],
      eventTypes: object["eventTypes"],
      channels: object["channels"],
      metadata: object["metadata"],
    };
  },

  _toJsonObject(self: EndpointPatch): any {
    return {
      description: self.description,
      throttleRate: self.throttleRate,
      uid: self.uid,
      url: self.url,
      disabled: self.disabled,
      eventTypes: self.eventTypes,
      channels: self.channels,
      metadata: self.metadata,
    };
  },
};
