// this file is @generated

export interface StreamIn {
  /** The stream's name. */
  name: string;
  /** An optional unique identifier for the stream. */
  uid?: string | null;
  metadata?: { [key: string]: string };
}

export const StreamInSerializer = {
  _fromJsonObject(object: any): StreamIn {
    return {
      name: object["name"],
      uid: object["uid"],
      metadata: object["metadata"],
    };
  },

  _toJsonObject(self: StreamIn): any {
    return {
      name: self.name,
      uid: self.uid,
      metadata: self.metadata,
    };
  },
};
