// this file is @generated

export interface OtelTracingConfigIn {
  url: string;
  headers?: { [key: string]: string };
}

export const OtelTracingConfigInSerializer = {
  _fromJsonObject(object: any): OtelTracingConfigIn {
    return {
      url: object["url"],
      headers: object["headers"],
    };
  },

  _toJsonObject(self: OtelTracingConfigIn): any {
    return {
      url: self.url,
      headers: self.headers,
    };
  },
};
