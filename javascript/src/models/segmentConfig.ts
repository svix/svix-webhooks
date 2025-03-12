// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

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
