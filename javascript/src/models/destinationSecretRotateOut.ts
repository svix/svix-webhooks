// this file is @generated

export interface DestinationSecretRotateOut {
  /**
   * The endpoint's verification secret.
   *
   * Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
   * It is recommended to not set this and let the server generate the secret.
   */
  key: string;
}

export const DestinationSecretRotateOutSerializer = {
  _fromJsonObject(object: any): DestinationSecretRotateOut {
    return {
      key: object["key"],
    };
  },

  _toJsonObject(self: DestinationSecretRotateOut): any {
    return {
      key: self.key,
    };
  },
};
