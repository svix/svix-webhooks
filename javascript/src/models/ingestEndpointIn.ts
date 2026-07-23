// this file is @generated

export interface IngestEndpointIn {
  description?: string;
  disabled?: boolean;
  metadata?: { [key: string]: string };
  /**
   * The endpoint's verification secret.
   *
   * Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
   * It is recommended to not set this and let the server generate the secret.
   */
  secret?: string | null;
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

export const IngestEndpointInSerializer = {
  _fromJsonObject(object: any): IngestEndpointIn {
    return {
      description: object["description"],
      disabled: object["disabled"],
      metadata: object["metadata"],
      secret: object["secret"],
      throttleRate: object["throttleRate"],
      uid: object["uid"],
      url: object["url"],
    };
  },

  _toJsonObject(self: IngestEndpointIn): any {
    return {
      description: self.description,
      disabled: self.disabled,
      metadata: self.metadata,
      secret: self.secret,
      throttleRate: self.throttleRate,
      uid: self.uid,
      url: self.url,
    };
  },
};
