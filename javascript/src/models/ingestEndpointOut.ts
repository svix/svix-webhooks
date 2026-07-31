// this file is @generated

export interface IngestEndpointOut {
  /** The Endpoint's ID. */
  id: string;
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
  createdAt: Date;
  updatedAt: Date;
  metadata: { [key: string]: string };
}

export const IngestEndpointOutSerializer = {
  _fromJsonObject(object: any): IngestEndpointOut {
    return {
      id: object["id"],
      url: object["url"],
      description: object["description"],
      throttleRate: object["throttleRate"],
      uid: object["uid"],
      disabled: object["disabled"],
      createdAt: new Date(object["createdAt"]),
      updatedAt: new Date(object["updatedAt"]),
      metadata: object["metadata"],
    };
  },

  _toJsonObject(self: IngestEndpointOut): any {
    return {
      id: self.id,
      url: self.url,
      description: self.description,
      throttleRate: self.throttleRate,
      uid: self.uid,
      disabled: self.disabled,
      createdAt: self.createdAt,
      updatedAt: self.updatedAt,
      metadata: self.metadata,
    };
  },
};
