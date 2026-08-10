// this file is @generated

export interface HttpConfigPatch {
  url?: string;
}

export const HttpConfigPatchSerializer = {
  _fromJsonObject(object: any): HttpConfigPatch {
    return {
      url: object["url"],
    };
  },

  _toJsonObject(self: HttpConfigPatch): any {
    return {
      url: self.url,
    };
  },
};
