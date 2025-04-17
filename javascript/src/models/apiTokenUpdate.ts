// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface ApiTokenUpdate {
  name: string;
}

export const ApiTokenUpdateSerializer = {
  _fromJsonObject(object: any): ApiTokenUpdate {
    return {
      name: object["name"],
    };
  },

  _toJsonObject(self: ApiTokenUpdate): any {
    return {
      name: self.name,
    };
  },
};
