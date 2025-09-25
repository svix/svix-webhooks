// this file is @generated

export interface IngestEndpointIn {
  description?: string;
  disabled?: boolean;
  metadata?: { [key: string]: string };
  rateLimit?: number | null;
  /**
   * The endpoint's verification secret.
   *
   * Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
   * It is recommended to not set this and let the server generate the secret.
   */
  secret?: string | null;
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
      rateLimit: object["rateLimit"],
      secret: object["secret"],
      uid: object["uid"],
      url: object["url"],
    };
  },

  _toJsonObject(self: IngestEndpointIn): any {
    return {
      description: self.description,
      disabled: self.disabled,
      metadata: self.metadata,
      rateLimit: self.rateLimit,
      secret: self.secret,
      uid: self.uid,
      url: self.url,
    };
  },
};
