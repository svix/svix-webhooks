// this file is @generated

export interface SinkOtelTracingConfigPatch {
  url?: string;
}

export const SinkOtelTracingConfigPatchSerializer = {
  _fromJsonObject(object: any): SinkOtelTracingConfigPatch {
    return {
      url: object["url"],
    };
  },

  _toJsonObject(self: SinkOtelTracingConfigPatch): any {
    return {
      url: self.url,
    };
  },
};
