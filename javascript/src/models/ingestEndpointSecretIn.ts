// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface IngestEndpointSecretIn {
  /**
   * The endpoint's verification secret.
   *
   * Format: `base64` encoded random bytes optionally prefixed with `whsec_`.
   * It is recommended to not set this and let the server generate the secret.
   */
  key?: string | null;
}

export const IngestEndpointSecretInSerializer = {
  _fromJsonObject(object: any): IngestEndpointSecretIn {
    return {
      key: object["key"],
    };
  },

  _toJsonObject(self: IngestEndpointSecretIn): any {
    return {
      key: self.key,
    };
  },
};
