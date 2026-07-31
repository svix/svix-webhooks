// this file is @generated

export interface ApiTokenOut {
  token: string;
  id: string;
  name?: string | null;
  createdAt: Date;
  expiresAt?: Date | null;
  scopes?: string[] | null;
}

export const ApiTokenOutSerializer = {
  _fromJsonObject(object: any): ApiTokenOut {
    return {
      token: object["token"],
      id: object["id"],
      name: object["name"],
      createdAt: new Date(object["createdAt"]),
      expiresAt: object["expiresAt"] ? new Date(object["expiresAt"]) : null,
      scopes: object["scopes"],
    };
  },

  _toJsonObject(self: ApiTokenOut): any {
    return {
      token: self.token,
      id: self.id,
      name: self.name,
      createdAt: self.createdAt,
      expiresAt: self.expiresAt,
      scopes: self.scopes,
    };
  },
};
