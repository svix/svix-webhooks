// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface RotateTokenOut {
  ingestUrl: string;
}

export const RotateTokenOutSerializer = {
  _fromJsonObject(object: any): RotateTokenOut {
    return {
      ingestUrl: object["ingestUrl"],
    };
  },

  _toJsonObject(self: RotateTokenOut): any {
    return {
      ingestUrl: self.ingestUrl,
    };
  },
};
