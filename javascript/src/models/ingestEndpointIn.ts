// this file is @generated

export interface IngestEndpointIn {
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
      description: object["description"],
      throttleRate: object["throttleRate"],
      uid: object["uid"],
      url: object["url"],
      disabled: object["disabled"],
      secret: object["secret"],
      metadata: object["metadata"],
    };
  },

  _toJsonObject(self: IngestEndpointIn): any {
    return {
      description: self.description,
      throttleRate: self.throttleRate,
      uid: self.uid,
      url: self.url,
      disabled: self.disabled,
      secret: self.secret,
      metadata: self.metadata,
    };
  },
};
