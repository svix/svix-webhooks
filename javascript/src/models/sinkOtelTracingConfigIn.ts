// this file is @generated

export interface SinkOtelTracingConfigIn {
  url: string;
  headers?: { [key: string]: string };
}

export const SinkOtelTracingConfigInSerializer = {
  _fromJsonObject(object: any): SinkOtelTracingConfigIn {
    return {
      url: object["url"],
      headers: object["headers"],
    };
  },

  _toJsonObject(self: SinkOtelTracingConfigIn): any {
    return {
      url: self.url,
      headers: self.headers,
    };
  },
};
