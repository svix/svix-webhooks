// this file is @generated

export interface EndpointUpsertIn {
  url: string;
  description?: string;
  /**
   * Maximum messages per second to send to this endpoint.
   *
   * Outgoing messages will be throttled to this rate.
   */
  throttleRate?: number | null;
  /** Optional unique identifier for the endpoint. */
  uid?: string | null;
  disabled?: boolean;
  eventTypes?: string[] | null;
  /** List of message channels this endpoint listens to (omit for all). */
  channels?: string[] | null;
  metadata?: { [key: string]: string };
}

export const EndpointUpsertInSerializer = {
  _fromJsonObject(object: any): EndpointUpsertIn {
    return {
      url: object["url"],
      description: object["description"],
      throttleRate: object["throttleRate"],
      uid: object["uid"],
      disabled: object["disabled"],
      eventTypes: object["eventTypes"],
      channels: object["channels"],
      metadata: object["metadata"],
    };
  },

  _toJsonObject(self: EndpointUpsertIn): any {
    return {
      url: self.url,
      description: self.description,
      throttleRate: self.throttleRate,
      uid: self.uid,
      disabled: self.disabled,
      eventTypes: self.eventTypes,
      channels: self.channels,
      metadata: self.metadata,
    };
  },
};
