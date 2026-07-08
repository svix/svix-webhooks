// this file is @generated

export interface OperationalWebhookEndpointOut {
  createdAt: Date;
  /** An example endpoint name. */
  description: string;
  disabled?: boolean;
  filterTypes?: string[] | null;
  /** The Endpoint's ID. */
  id: string;
  metadata: { [key: string]: string };
  /**
   * Maximum messages per second to send to this endpoint.
   *
   * Outgoing messages will be throttled to this rate.
   */
  throttleRate?: number | null;
  /** Optional unique identifier for the endpoint. */
  uid?: string | null;
  updatedAt: Date;
  url: string;
}

export const OperationalWebhookEndpointOutSerializer = {
  _fromJsonObject(object: any): OperationalWebhookEndpointOut {
    return {
      createdAt: new Date(object["createdAt"]),
      description: object["description"],
      disabled: object["disabled"],
      filterTypes: object["filterTypes"],
      id: object["id"],
      metadata: object["metadata"],
      throttleRate: object["throttleRate"],
      uid: object["uid"],
      updatedAt: new Date(object["updatedAt"]),
      url: object["url"],
    };
  },

  _toJsonObject(self: OperationalWebhookEndpointOut): any {
    return {
      createdAt: self.createdAt,
      description: self.description,
      disabled: self.disabled,
      filterTypes: self.filterTypes,
      id: self.id,
      metadata: self.metadata,
      throttleRate: self.throttleRate,
      uid: self.uid,
      updatedAt: self.updatedAt,
      url: self.url,
    };
  },
};
