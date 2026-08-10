// this file is @generated

export interface OtelTracingConfigPatch {
  url?: string;
}

export const OtelTracingConfigPatchSerializer = {
  _fromJsonObject(object: any): OtelTracingConfigPatch {
    return {
      url: object["url"],
    };
  },

  _toJsonObject(self: OtelTracingConfigPatch): any {
    return {
      url: self.url,
    };
  },
};
