// this file is @generated

export interface SegmentConfig {
  secret?: string | null;
}

export const SegmentConfigSerializer = {
  _fromJsonObject(object: any): SegmentConfig {
    return {
      secret: object["secret"],
    };
  },

  _toJsonObject(self: SegmentConfig): any {
    return {
      secret: self.secret,
    };
  },
};
