// this file is @generated

export interface HttpPatchConfig {
  url?: string;
}

export const HttpPatchConfigSerializer = {
  _fromJsonObject(object: any): HttpPatchConfig {
    return {
      url: object["url"],
    };
  },

  _toJsonObject(self: HttpPatchConfig): any {
    return {
      url: self.url,
    };
  },
};
