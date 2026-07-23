// this file is @generated

export interface IngestEndpointUpsertIn {
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
  metadata?: { [key: string]: string };
}

export const IngestEndpointUpsertInSerializer = {
  _fromJsonObject(object: any): IngestEndpointUpsertIn {
    return {
      url: object["url"],
      description: object["description"],
      throttleRate: object["throttleRate"],
      uid: object["uid"],
      disabled: object["disabled"],
      metadata: object["metadata"],
    };
  },

  _toJsonObject(self: IngestEndpointUpsertIn): any {
    return {
      url: self.url,
      description: self.description,
      throttleRate: self.throttleRate,
      uid: self.uid,
      disabled: self.disabled,
      metadata: self.metadata,
    };
  },
};
