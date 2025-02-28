// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface IngestEndpointHeadersIn {
  headers: { [key: string]: string };
}

export const IngestEndpointHeadersInSerializer = {
  _fromJsonObject(object: any): IngestEndpointHeadersIn {
    return {
      headers: object["headers"],
    };
  },

  _toJsonObject(self: IngestEndpointHeadersIn): any {
    return {
      headers: self.headers,
    };
  },
};
