// this file is @generated

export interface OperationalWebhookEndpointUpdate {
  description?: string;
  disabled?: boolean;
  filterTypes?: string[] | null;
  metadata?: { [key: string]: string };
  /**
   * Maximum messages per second to send to this endpoint.
   *
   * Outgoing messages will be throttled to this rate.
   */
  throttleRate?: number | null;
  /** Optional unique identifier for the endpoint. */
  uid?: string | null;
  url: string;
}

export const OperationalWebhookEndpointUpdateSerializer = {
  _fromJsonObject(object: any): OperationalWebhookEndpointUpdate {
    return {
      description: object["description"],
      disabled: object["disabled"],
      filterTypes: object["filterTypes"],
      metadata: object["metadata"],
      throttleRate: object["throttleRate"],
      uid: object["uid"],
      url: object["url"],
    };
  },

  _toJsonObject(self: OperationalWebhookEndpointUpdate): any {
    return {
      description: self.description,
      disabled: self.disabled,
      filterTypes: self.filterTypes,
      metadata: self.metadata,
      throttleRate: self.throttleRate,
      uid: self.uid,
      url: self.url,
    };
  },
};
