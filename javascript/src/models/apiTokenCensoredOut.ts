// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface ApiTokenCensoredOut {
  censoredToken: string;
  createdAt: Date;
  expiresAt?: Date | null;
  /** The ApplicationToken's ID. */
  id: string;
  name?: string | null;
  scopes?: string[] | null;
}

export const ApiTokenCensoredOutSerializer = {
  _fromJsonObject(object: any): ApiTokenCensoredOut {
    return {
      censoredToken: object["censoredToken"],
      createdAt: new Date(object["createdAt"]),
      expiresAt: new Date(object["expiresAt"]),
      id: object["id"],
      name: object["name"],
      scopes: object["scopes"],
    };
  },

  _toJsonObject(self: ApiTokenCensoredOut): any {
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
