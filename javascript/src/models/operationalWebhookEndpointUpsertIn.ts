// this file is @generated

export interface OperationalWebhookEndpointUpsertIn {
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
  metadata?: { [key: string]: string };
}

export const OperationalWebhookEndpointUpsertInSerializer = {
  _fromJsonObject(object: any): OperationalWebhookEndpointUpsertIn {
    return {
      url: object["url"],
      description: object["description"],
      throttleRate: object["throttleRate"],
      uid: object["uid"],
      disabled: object["disabled"],
      eventTypes: object["eventTypes"],
      metadata: object["metadata"],
    };
  },

  _toJsonObject(self: OperationalWebhookEndpointUpsertIn): any {
    return {
      url: self.url,
      description: self.description,
      throttleRate: self.throttleRate,
      uid: self.uid,
      disabled: self.disabled,
      eventTypes: self.eventTypes,
      metadata: self.metadata,
    };
  },
};
