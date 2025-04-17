// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface GlobalApiTokenCensoredOut {
  censoredToken: string;
  createdAt: Date;
  expiresAt?: Date | null;
  /** The GlobalApplicationToken's ID. */
  id: string;
  name?: string | null;
  scopes?: string[] | null;
}

export const GlobalApiTokenCensoredOutSerializer = {
  _fromJsonObject(object: any): GlobalApiTokenCensoredOut {
    return {
      censoredToken: object["censoredToken"],
      createdAt: new Date(object["createdAt"]),
      expiresAt: new Date(object["expiresAt"]),
      id: object["id"],
      name: object["name"],
      scopes: object["scopes"],
    };
  },

  _toJsonObject(self: GlobalApiTokenCensoredOut): any {
    return {
      censoredToken: self.censoredToken,
      createdAt: self.createdAt,
      expiresAt: self.expiresAt,
      id: self.id,
      name: self.name,
      scopes: self.scopes,
    };
  },
};
