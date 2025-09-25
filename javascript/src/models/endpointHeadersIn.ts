// this file is @generated

export interface EndpointHeadersIn {
  headers: { [key: string]: string };
}

export const EndpointHeadersInSerializer = {
  _fromJsonObject(object: any): EndpointHeadersIn {
    return {
      headers: object["headers"],
    };
  },

  _toJsonObject(self: EndpointHeadersIn): any {
    return {
      headers: self.headers,
    };
  },
};
