// this file is @generated

export interface StreamOut {
  createdAt: Date;
  /** The stream's ID. */
  id: string;
  metadata: { [key: string]: string };
  /** The stream's name. */
  name?: string | null;
  /** The stream's UID. */
  uid?: string | null;
  updatedAt: Date;
}

export const StreamOutSerializer = {
  _fromJsonObject(object: any): StreamOut {
    return {
      createdAt: new Date(object["createdAt"]),
      id: object["id"],
      metadata: object["metadata"],
      name: object["name"],
      uid: object["uid"],
      updatedAt: new Date(object["updatedAt"]),
    };
  },

  _toJsonObject(self: StreamOut): any {
    return {
      createdAt: self.createdAt,
      id: self.id,
      metadata: self.metadata,
      name: self.name,
      uid: self.uid,
      updatedAt: self.updatedAt,
    };
  },
};
