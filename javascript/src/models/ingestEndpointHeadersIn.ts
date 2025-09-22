// this file is @generated

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
