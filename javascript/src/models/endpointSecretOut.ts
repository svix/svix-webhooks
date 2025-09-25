// this file is @generated

export interface EndpointSecretOut {
  /**
   * The endpoint's verification secret.
   *
   * Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
   * It is recommended to not set this and let the server generate the secret.
   */
  key: string;
}

export const EndpointSecretOutSerializer = {
  _fromJsonObject(object: any): EndpointSecretOut {
    return {
      key: object["key"],
    };
  },

  _toJsonObject(self: EndpointSecretOut): any {
    return {
      key: self.key,
    };
  },
};
