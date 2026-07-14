// this file is @generated

export interface EndpointOut {
  /** The Endpoint's ID. */
  id: string;
  metadata: { [key: string]: string };
  /** An example endpoint name. */
  description: string;
  /**
   * Maximum messages per second to send to this endpoint.
   *
   * Outgoing messages will be throttled to this rate.
   */
  throttleRate?: number | null;
  /** Optional unique identifier for the endpoint. */
  uid?: string | null;
  url: string;
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
      description: object["description"],
      throttleRate: object["throttleRate"],
      uid: object["uid"],
      url: object["url"],
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
      description: self.description,
      throttleRate: self.throttleRate,
      uid: self.uid,
      url: self.url,
      disabled: self.disabled,
      eventTypes: self.eventTypes,
      channels: self.channels,
      createdAt: self.createdAt,
      updatedAt: self.updatedAt,
    };
  },
};
