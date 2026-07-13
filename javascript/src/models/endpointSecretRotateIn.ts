// this file is @generated

export interface EndpointSecretRotateIn {
  /**
   * The endpoint's verification secret.
   *
   * Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
   * It is recommended to not set this and let the server generate the secret.
   */
  key?: string | null;
  /**
   * How long the old secret will be valid for, in seconds.
   *
   * Valid values are between 0 (immediate expiry) and 7 days. The default is 24 hours.
   */
  gracePeriodSeconds?: number | null;
}

export const EndpointSecretRotateInSerializer = {
  _fromJsonObject(object: any): EndpointSecretRotateIn {
    return {
      key: object["key"],
      gracePeriodSeconds: object["gracePeriodSeconds"],
    };
  },

  _toJsonObject(self: EndpointSecretRotateIn): any {
    return {
      key: self.key,
      gracePeriodSeconds: self.gracePeriodSeconds,
    };
  },
};
