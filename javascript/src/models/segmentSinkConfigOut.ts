// this file is @generated

export interface SegmentSinkConfigOut {
  host: string;
}

export const SegmentSinkConfigOutSerializer = {
  _fromJsonObject(object: any): SegmentSinkConfigOut {
    return {
      host: object["host"],
    };
  },

  _toJsonObject(self: SegmentSinkConfigOut): any {
    return {
      host: self.host,
    };
  },
};
