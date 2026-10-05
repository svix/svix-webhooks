// this file is @generated

export interface SinkSecretRotateOut {
  /**
   * The endpoint's verification secret.
   *
   * Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
   * It is recommended to not set this and let the server generate the secret.
   */
  key: string;
}

export const SinkSecretRotateOutSerializer = {
  _fromJsonObject(object: any): SinkSecretRotateOut {
    return {
      key: object["key"],
    };
  },

  _toJsonObject(self: SinkSecretRotateOut): any {
    return {
      key: self.key,
    };
  },
};
