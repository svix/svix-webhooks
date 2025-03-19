// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface CreateTokenIn {
  /** How long the token will be valid for, in seconds. */
  expiry?: number | null;
  /** The name of the token. */
  name: string;
}

export const CreateTokenInSerializer = {
  _fromJsonObject(object: any): CreateTokenIn {
    return {
      expiry: object["expiry"],
      name: object["name"],
    };
  },

  _toJsonObject(self: CreateTokenIn): any {
    return {
      expiry: self.expiry,
      name: self.name,
    };
  },
};
