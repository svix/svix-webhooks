// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface EndpointMtlsConfigOut {
  serverCaCert?: string | null;
}

export const EndpointMtlsConfigOutSerializer = {
  _fromJsonObject(object: any): EndpointMtlsConfigOut {
    return {
      serverCaCert: object["serverCaCert"],
    };
  },

  _toJsonObject(self: EndpointMtlsConfigOut): any {
    return {
      serverCaCert: self.serverCaCert,
    };
  },
};
