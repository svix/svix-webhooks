// this file is @generated

export interface SinkInCommon {
  /** List of message channels this sink listens to (omit for all). */
  channels?: string[] | null;
  description?: string;
  disabled?: boolean;
  filterTypes?: string[] | null;
  metadata?: { [key: string]: string };
  /**
   * Deprecated, use `throttleRate` instead.
   *
   * @deprecated
   */
  rateLimit?: number | null;
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
  /** Optional unique identifier for the sink. */
  uid?: string | null;
}

export const SinkInCommonSerializer = {
  _fromJsonObject(object: any): SinkInCommon {
    return {
      channels: object["channels"],
      description: object["description"],
      disabled: object["disabled"],
      filterTypes: object["filterTypes"],
      metadata: object["metadata"],
      rateLimit: object["rateLimit"],
      secret: object["secret"],
      throttleRate: object["throttleRate"],
      uid: object["uid"],
    };
  },

  _toJsonObject(self: SinkInCommon): any {
    return {
      channels: self.channels,
      description: self.description,
      disabled: self.disabled,
      filterTypes: self.filterTypes,
      metadata: self.metadata,
      rateLimit: self.rateLimit,
      secret: self.secret,
      throttleRate: self.throttleRate,
      uid: self.uid,
    };
  },
};
