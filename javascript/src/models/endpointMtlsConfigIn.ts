// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface EndpointMtlsConfigIn {
  /** A PEM encoded private key and X509 certificate to identify the webhook sender. */
  identity: string;
  /** A PEM encoded X509 certificate used to verify the webhook receiver's certificate. */
  serverCaCert?: string | null;
}

export const EndpointMtlsConfigInSerializer = {
  _fromJsonObject(object: any): EndpointMtlsConfigIn {
    return {
      identity: object["identity"],
      serverCaCert: object["serverCaCert"],
    };
  },

  _toJsonObject(self: EndpointMtlsConfigIn): any {
    return {
      identity: self.identity,
      serverCaCert: self.serverCaCert,
    };
  },
};
