// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface DocusignConfig {
  secret?: string | null;
}

export const DocusignConfigSerializer = {
  _fromJsonObject(object: any): DocusignConfig {
    return {
      secret: object["secret"],
    };
  },

  _toJsonObject(self: DocusignConfig): any {
    return {
      secret: self.secret,
    };
  },
};
