// this file is @generated

export interface FifoEndpointConfigIn {
  url: string;
  headers?: { [key: string]: string };
  key?: string | null;
}

export const FifoEndpointConfigInSerializer = {
  _fromJsonObject(object: any): FifoEndpointConfigIn {
    return {
      url: object["url"],
      headers: object["headers"],
      key: object["key"],
    };
  },

  _toJsonObject(self: FifoEndpointConfigIn): any {
    return {
      url: self.url,
      headers: self.headers,
      key: self.key,
    };
  },
};
