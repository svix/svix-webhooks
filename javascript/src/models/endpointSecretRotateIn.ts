// this file is @generated

export interface EndpointSecretRotateIn {
  /**
   * How long the old secret will be valid for, in seconds.
   *
   * Valid values are between 0 (immediate expiry) and 7 days. The default is 24 hours.
   */
  gracePeriodSeconds?: number | null;
  /**
   * The endpoint's verification secret.
   *
   * Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
   * It is recommended to not set this and let the server generate the secret.
   */
  key?: string | null;
}

export const EndpointSecretRotateInSerializer = {
  _fromJsonObject(object: any): EndpointSecretRotateIn {
    return {
      gracePeriodSeconds: object["gracePeriodSeconds"],
      key: object["key"],
    };
  },

  _toJsonObject(self: EndpointSecretRotateIn): any {
    return {
      gracePeriodSeconds: self.gracePeriodSeconds,
      key: self.key,
    };
  },
};
