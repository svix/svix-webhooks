// this file is @generated

export interface EndpointOut {
  /** The Endpoint's ID. */
  id: string;
  metadata: { [key: string]: string };
  url: string;
  description: string;
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
  createdAt: Date;
  updatedAt: Date;
}

export const EndpointOutSerializer = {
  _fromJsonObject(object: any): EndpointOut {
    return {
      id: object["id"],
      metadata: object["metadata"],
      url: object["url"],
      description: object["description"],
      throttleRate: object["throttleRate"],
      uid: object["uid"],
      disabled: object["disabled"],
      eventTypes: object["eventTypes"],
      channels: object["channels"],
      createdAt: new Date(object["createdAt"]),
      updatedAt: new Date(object["updatedAt"]),
    };
  },

  _toJsonObject(self: EndpointOut): any {
    return {
      id: self.id,
      metadata: self.metadata,
      url: self.url,
      description: self.description,
      throttleRate: self.throttleRate,
      uid: self.uid,
      disabled: self.disabled,
      eventTypes: self.eventTypes,
      channels: self.channels,
      createdAt: self.createdAt,
      updatedAt: self.updatedAt,
    };
  },
};
