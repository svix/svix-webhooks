// this file is @generated

export interface IngestEndpointIn {
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
  /**
   * The endpoint's verification secret.
   *
   * Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
   * It is recommended to not set this and let the server generate the secret.
   */
  secret?: string | null;
  metadata?: { [key: string]: string };
}

export const IngestEndpointInSerializer = {
  _fromJsonObject(object: any): IngestEndpointIn {
    return {
      url: object["url"],
      description: object["description"],
      throttleRate: object["throttleRate"],
      uid: object["uid"],
      disabled: object["disabled"],
      secret: object["secret"],
      metadata: object["metadata"],
    };
  },

  _toJsonObject(self: IngestEndpointIn): any {
    return {
      url: self.url,
      description: self.description,
      throttleRate: self.throttleRate,
      uid: self.uid,
      disabled: self.disabled,
      secret: self.secret,
      metadata: self.metadata,
    };
  },
};
