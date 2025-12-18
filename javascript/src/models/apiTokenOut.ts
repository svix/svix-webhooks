// this file is @generated

export interface ApiTokenOut {
  createdAt: Date;
  expiresAt?: Date | null;
  id: string;
  name?: string | null;
  scopes?: string[] | null;
  token: string;
}

export const ApiTokenOutSerializer = {
  _fromJsonObject(object: any): ApiTokenOut {
    return {
      createdAt: new Date(object["createdAt"]),
      expiresAt: object["expiresAt"] ? new Date(object["expiresAt"]) : null,
      id: object["id"],
      name: object["name"],
      scopes: object["scopes"],
      token: object["token"],
    };
  },

  _toJsonObject(self: ApiTokenOut): any {
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
