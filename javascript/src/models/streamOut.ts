// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface StreamOut {
  createdAt: Date;
  /** The stream's description. */
  description?: string | null;
  /** The stream's ID. */
  id: string;
  /** The stream's UID. */
  uid?: string | null;
  updatedAt: Date;
}

export const StreamOutSerializer = {
  _fromJsonObject(object: any): StreamOut {
    return {
      createdAt: new Date(object["createdAt"]),
      description: object["description"],
      id: object["id"],
      uid: object["uid"],
      updatedAt: new Date(object["updatedAt"]),
    };
  },

  _toJsonObject(self: StreamOut): any {
    return {
      createdAt: self.createdAt,
      description: self.description,
      id: self.id,
      uid: self.uid,
      updatedAt: self.updatedAt,
    };
  },
};
