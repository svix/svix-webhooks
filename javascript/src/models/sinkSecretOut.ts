// this file is @generated

export interface SinkSecretOut {
  /**
   * The endpoint's verification secret.
   *
   * Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
   * It is recommended to not set this and let the server generate the secret.
   */
  key?: string | null;
}

export const SinkSecretOutSerializer = {
  _fromJsonObject(object: any): SinkSecretOut {
    return {
      key: object["key"],
    };
  },

  _toJsonObject(self: SinkSecretOut): any {
    return {
      key: self.key,
    };
  },
};
