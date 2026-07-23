// this file is @generated

export interface OperationalWebhookEndpointUpsertIn {
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
  filterTypes?: string[] | null;
  metadata?: { [key: string]: string };
}

export const OperationalWebhookEndpointUpsertInSerializer = {
  _fromJsonObject(object: any): OperationalWebhookEndpointUpsertIn {
    return {
      description: object["description"],
      throttleRate: object["throttleRate"],
      uid: object["uid"],
      url: object["url"],
      disabled: object["disabled"],
      filterTypes: object["filterTypes"],
      metadata: object["metadata"],
    };
  },

  _toJsonObject(self: OperationalWebhookEndpointUpsertIn): any {
    return {
      description: self.description,
      throttleRate: self.throttleRate,
      uid: self.uid,
      url: self.url,
      disabled: self.disabled,
      filterTypes: self.filterTypes,
      metadata: self.metadata,
    };
  },
};
