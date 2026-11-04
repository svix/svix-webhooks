// this file is @generated

export interface SegmentSinkConfigPatch {
  host?: string;
  writeKey?: string;
}

export const SegmentSinkConfigPatchSerializer = {
  _fromJsonObject(object: any): SegmentSinkConfigPatch {
    return {
      host: object["host"],
      writeKey: object["writeKey"],
    };
  },

  _toJsonObject(self: SegmentSinkConfigPatch): any {
    return {
      host: self.host,
      writeKey: self.writeKey,
    };
  },
};
