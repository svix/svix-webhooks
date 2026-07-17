// this file is @generated

export interface EndpointIn {
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
  /** List of message channels this endpoint listens to (omit for all). */
  channels?: string[] | null;
  /**
   * The endpoint's verification secret.
   *
   * Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
   * It is recommended to not set this and let the server generate the secret.
   */
  secret?: string | null;
  metadata?: { [key: string]: string };
  headers?: { [key: string]: string } | null;
}

export const EndpointInSerializer = {
  _fromJsonObject(object: any): EndpointIn {
    return {
      url: object["url"],
      description: object["description"],
      throttleRate: object["throttleRate"],
      uid: object["uid"],
      disabled: object["disabled"],
      eventTypes: object["eventTypes"],
      channels: object["channels"],
      secret: object["secret"],
      metadata: object["metadata"],
      headers: object["headers"],
    };
  },

  _toJsonObject(self: EndpointIn): any {
    return {
      url: self.url,
      description: self.description,
      throttleRate: self.throttleRate,
      uid: self.uid,
      disabled: self.disabled,
      eventTypes: self.eventTypes,
      channels: self.channels,
      secret: self.secret,
      metadata: self.metadata,
      headers: self.headers,
    };
  },
};
