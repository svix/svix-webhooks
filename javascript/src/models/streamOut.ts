// this file is @generated

export interface StreamOut {
  /** The stream's ID. */
  id: string;
  /** The stream's UID. */
  uid?: string | null;
  /** The stream's name. */
  name?: string | null;
  createdAt: Date;
  updatedAt: Date;
  metadata: { [key: string]: string };
}

export const StreamOutSerializer = {
  _fromJsonObject(object: any): StreamOut {
    return {
      id: object["id"],
      uid: object["uid"],
      name: object["name"],
      createdAt: new Date(object["createdAt"]),
      updatedAt: new Date(object["updatedAt"]),
      metadata: object["metadata"],
    };
  },

  _toJsonObject(self: StreamOut): any {
    return {
      id: self.id,
      uid: self.uid,
      name: self.name,
      createdAt: self.createdAt,
      updatedAt: self.updatedAt,
      metadata: self.metadata,
    };
  },
};
