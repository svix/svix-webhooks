// this file is @generated

export interface IngestEndpointUpdate {
  description?: string;
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
  metadata?: { [key: string]: string };
}

export const IngestEndpointUpdateSerializer = {
  _fromJsonObject(object: any): IngestEndpointUpdate {
    return {
      description: object["description"],
      throttleRate: object["throttleRate"],
      uid: object["uid"],
      url: object["url"],
      disabled: object["disabled"],
      metadata: object["metadata"],
    };
  },

  _toJsonObject(self: IngestEndpointUpdate): any {
    return {
      description: self.description,
      throttleRate: self.throttleRate,
      uid: self.uid,
      url: self.url,
      disabled: self.disabled,
      metadata: self.metadata,
    };
  },
};
