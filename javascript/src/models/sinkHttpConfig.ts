// this file is @generated

export interface SinkHttpConfig {
  url: string;
  headers?: { [key: string]: string };
  key?: string | null;
}

export const SinkHttpConfigSerializer = {
  _fromJsonObject(object: any): SinkHttpConfig {
    return {
      url: object["url"],
      headers: object["headers"],
      key: object["key"],
    };
  },

  _toJsonObject(self: SinkHttpConfig): any {
    return {
      url: self.url,
      headers: self.headers,
      key: self.key,
    };
  },
};
