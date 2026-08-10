// this file is @generated

export interface SinkHttpConfigIn {
  url: string;
  headers?: { [key: string]: string };
  key?: string | null;
}

export const SinkHttpConfigInSerializer = {
  _fromJsonObject(object: any): SinkHttpConfigIn {
    return {
      url: object["url"],
      headers: object["headers"],
      key: object["key"],
    };
  },

  _toJsonObject(self: SinkHttpConfigIn): any {
    return {
      url: self.url,
      headers: self.headers,
      key: self.key,
    };
  },
};
