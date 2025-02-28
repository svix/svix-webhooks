// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface IngestEndpointHeadersOut {
  headers: { [key: string]: string };
  sensitive: string[];
}

export const IngestEndpointHeadersOutSerializer = {
  _fromJsonObject(object: any): IngestEndpointHeadersOut {
    return {
      headers: object["headers"],
      sensitive: object["sensitive"],
    };
  },

  _toJsonObject(self: IngestEndpointHeadersOut): any {
    return {
      headers: self.headers,
      sensitive: self.sensitive,
    };
  },
};
