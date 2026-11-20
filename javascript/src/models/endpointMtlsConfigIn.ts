// this file is @generated

export interface EndpointMtlsConfigIn {
  /** A PEM encoded private key and X.509 certificate to identify the webhook sender. */
  identity: string;
}

export const EndpointMtlsConfigInSerializer = {
  _fromJsonObject(object: any): EndpointMtlsConfigIn {
    return {
      identity: object["identity"],
    };
  },

  _toJsonObject(self: EndpointMtlsConfigIn): any {
    return {
      identity: self.identity,
    };
  },
};
