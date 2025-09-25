// this file is @generated

export interface EndpointIn {
  /** List of message channels this endpoint listens to (omit for all). */
  channels?: string[] | null;
  description?: string;
  disabled?: boolean;
  filterTypes?: string[] | null;
  headers?: { [key: string]: string } | null;
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
  version?: number | null;
}

export const EndpointInSerializer = {
  _fromJsonObject(object: any): EndpointIn {
    return {
      channels: object["channels"],
      description: object["description"],
      disabled: object["disabled"],
      filterTypes: object["filterTypes"],
      headers: object["headers"],
      metadata: object["metadata"],
      rateLimit: object["rateLimit"],
      secret: object["secret"],
      uid: object["uid"],
      url: object["url"],
      version: object["version"],
    };
  },

  _toJsonObject(self: EndpointIn): any {
    return {
      channels: self.channels,
      description: self.description,
      disabled: self.disabled,
      filterTypes: self.filterTypes,
      headers: self.headers,
      metadata: self.metadata,
      rateLimit: self.rateLimit,
      secret: self.secret,
      uid: self.uid,
      url: self.url,
      version: self.version,
    };
  },
};
