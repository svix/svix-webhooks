// this file is @generated

export interface SinkHttpConfigPatch {
  url?: string;
}

export const SinkHttpConfigPatchSerializer = {
  _fromJsonObject(object: any): SinkHttpConfigPatch {
    return {
      url: object["url"],
    };
  },

  _toJsonObject(self: SinkHttpConfigPatch): any {
    return {
      url: self.url,
    };
  },
};
