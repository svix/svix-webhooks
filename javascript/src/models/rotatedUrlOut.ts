// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface RotatedUrlOut {
  url: string;
}

export const RotatedUrlOutSerializer = {
  _fromJsonObject(object: any): RotatedUrlOut {
    return {
      url: object["url"],
    };
  },

  _toJsonObject(self: RotatedUrlOut): any {
    return {
      url: self.url,
    };
  },
};
