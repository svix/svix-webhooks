// this file is @generated

export interface AutoConfigOut {
  createdAt: Date;
  token: string;
  /** The AutoConfigSubscription's ID. */
  id: string;
}

export const AutoConfigOutSerializer = {
  _fromJsonObject(object: any): AutoConfigOut {
    return {
      createdAt: new Date(object["createdAt"]),
      token: object["token"],
      id: object["id"],
    };
  },

  _toJsonObject(self: AutoConfigOut): any {
    return {
      createdAt: self.createdAt,
      token: self.token,
      id: self.id,
    };
  },
};
