// this file is @generated

export interface SinkHttpConfig {
  headers?: { [key: string]: string };
  key?: string | null;
  url: string;
}

export const SinkHttpConfigSerializer = {
  _fromJsonObject(object: any): SinkHttpConfig {
    return {
      headers: object["headers"],
      key: object["key"],
      url: object["url"],
    };
  },

  _toJsonObject(self: SinkHttpConfig): any {
    return {
      headers: self.headers,
      key: self.key,
      url: self.url,
    };
  },
};
