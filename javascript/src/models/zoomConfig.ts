// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface ZoomConfig {
  secret: string;
}

export const ZoomConfigSerializer = {
  _fromJsonObject(object: any): ZoomConfig {
    return {
      secret: object["secret"],
    };
  },

  _toJsonObject(self: ZoomConfig): any {
    return {
      secret: self.secret,
    };
  },
};
