// this file is @generated

export interface SinkInCommon {
  description?: string;
  /**
   * Maximum messages per second to send to this endpoint.
   *
   * Outgoing messages will be throttled to this rate.
   */
  throttleRate?: number | null;
  /** Optional unique identifier for the sink. */
  uid?: string | null;
  /**
   * The endpoint's verification secret.
   *
   * Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
   * It is recommended to not set this and let the server generate the secret.
   */
  secret?: string | null;
  disabled?: boolean;
  eventTypes?: string[] | null;
  /** List of message channels this sink listens to (omit for all). */
  channels?: string[] | null;
  metadata?: { [key: string]: string };
}

export const SinkInCommonSerializer = {
  _fromJsonObject(object: any): SinkInCommon {
    return {
      description: object["description"],
      throttleRate: object["throttleRate"],
      uid: object["uid"],
      secret: object["secret"],
      disabled: object["disabled"],
      eventTypes: object["eventTypes"],
      channels: object["channels"],
      metadata: object["metadata"],
    };
  },

  _toJsonObject(self: SinkInCommon): any {
    return {
      description: self.description,
      throttleRate: self.throttleRate,
      uid: self.uid,
      secret: self.secret,
      disabled: self.disabled,
      eventTypes: self.eventTypes,
      channels: self.channels,
      metadata: self.metadata,
    };
  },
};
