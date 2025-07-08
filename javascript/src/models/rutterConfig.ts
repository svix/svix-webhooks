// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface RutterConfig {
  secret: string;
}

export const RutterConfigSerializer = {
  _fromJsonObject(object: any): RutterConfig {
    return {
      secret: object["secret"],
    };
  },

  _toJsonObject(self: RutterConfig): any {
    return {
      secret: self.secret,
    };
  },
};
