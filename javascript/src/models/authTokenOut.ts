// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface AuthTokenOut {
  createdAt: Date;
  expiresAt?: Date | null;
  /** The ApplicationToken's ID. */
  id: string;
  name?: string | null;
  scopes?: string[] | null;
  token: string;
}

export const AuthTokenOutSerializer = {
  _fromJsonObject(object: any): AuthTokenOut {
    return {
      createdAt: new Date(object["createdAt"]),
      expiresAt: new Date(object["expiresAt"]),
      id: object["id"],
      name: object["name"],
      scopes: object["scopes"],
      token: object["token"],
    };
  },

  _toJsonObject(self: AuthTokenOut): any {
    return {
      createdAt: self.createdAt,
      expiresAt: self.expiresAt,
      id: self.id,
      name: self.name,
      scopes: self.scopes,
      token: self.token,
    };
  },
};
