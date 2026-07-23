// this file is @generated

export interface SinkOtelV1Config {
  url: string;
  headers?: { [key: string]: string };
}

export const SinkOtelV1ConfigSerializer = {
  _fromJsonObject(object: any): SinkOtelV1Config {
    return {
      url: object["url"],
      headers: object["headers"],
    };
  },

  _toJsonObject(self: SinkOtelV1Config): any {
    return {
      url: self.url,
      headers: self.headers,
    };
  },
};
