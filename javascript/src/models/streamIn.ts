// this file is @generated

export interface StreamIn {
  metadata?: { [key: string]: string };
  /** The stream's name. */
  name: string;
  /** An optional unique identifier for the stream. */
  uid?: string | null;
}

export const StreamInSerializer = {
  _fromJsonObject(object: any): StreamIn {
    return {
      metadata: object["metadata"],
      name: object["name"],
      uid: object["uid"],
    };
  },

  _toJsonObject(self: StreamIn): any {
    return {
      metadata: self.metadata,
      name: self.name,
      uid: self.uid,
    };
  },
};
