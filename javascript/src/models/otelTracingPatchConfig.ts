// this file is @generated

export interface OtelTracingPatchConfig {
  url?: string;
}

export const OtelTracingPatchConfigSerializer = {
  _fromJsonObject(object: any): OtelTracingPatchConfig {
    return {
      url: object["url"],
    };
  },

  _toJsonObject(self: OtelTracingPatchConfig): any {
    return {
      url: self.url,
    };
  },
};
