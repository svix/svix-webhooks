// this file is @generated

export interface IngestEndpointOut {
  /** The Endpoint's ID. */
  id: string;
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
  createdAt: Date;
  updatedAt: Date;
  metadata: { [key: string]: string };
}

export const IngestEndpointOutSerializer = {
  _fromJsonObject(object: any): IngestEndpointOut {
    return {
      id: object["id"],
      description: object["description"],
      throttleRate: object["throttleRate"],
      uid: object["uid"],
      url: object["url"],
      disabled: object["disabled"],
      createdAt: new Date(object["createdAt"]),
      updatedAt: new Date(object["updatedAt"]),
      metadata: object["metadata"],
    };
  },

  _toJsonObject(self: IngestEndpointOut): any {
    return {
      id: self.id,
      description: self.description,
      throttleRate: self.throttleRate,
      uid: self.uid,
      url: self.url,
      disabled: self.disabled,
      createdAt: self.createdAt,
      updatedAt: self.updatedAt,
      metadata: self.metadata,
    };
  },
};
