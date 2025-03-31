// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface ApiTokenIn {
  name: string;
  scopes?: string[] | null;
}

export const ApiTokenInSerializer = {
  _fromJsonObject(object: any): ApiTokenIn {
    return {
      name: object["name"],
      scopes: object["scopes"],
    };
  },

  _toJsonObject(self: ApiTokenIn): any {
    return {
      name: self.name,
      scopes: self.scopes,
    };
  },
};
