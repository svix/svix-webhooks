// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface OtelConfig {
  additionalHeaders?: { [key: string]: string } | null;
  url: string;
}

export const OtelConfigSerializer = {
  _fromJsonObject(object: any): OtelConfig {
    return {
      additionalHeaders: object["additionalHeaders"],
      url: object["url"],
    };
  },

  _toJsonObject(self: OtelConfig): any {
    return {
      additionalHeaders: self.additionalHeaders,
      url: self.url,
    };
  },
};
