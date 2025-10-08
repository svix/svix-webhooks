// this file is @generated

export interface SinkOtelV1Config {
  headers?: { [key: string]: string };
  url: string;
}

export const SinkOtelV1ConfigSerializer = {
  _fromJsonObject(object: any): SinkOtelV1Config {
    return {
      headers: object["headers"],
      url: object["url"],
    };
  },

  _toJsonObject(self: SinkOtelV1Config): any {
    return {
      headers: self.headers,
      url: self.url,
    };
  },
};
