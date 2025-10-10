// this file is @generated

export interface StreamPatch {
  /** The Stream's description. */
  description?: string;
  metadata?: { [key: string]: string };
  /** An optional unique identifier for the stream. */
  uid?: string | null;
}

export const StreamPatchSerializer = {
  _fromJsonObject(object: any): StreamPatch {
    return {
      description: object["description"],
      metadata: object["metadata"],
      uid: object["uid"],
    };
  },

  _toJsonObject(self: StreamPatch): any {
    return {
      description: self.description,
      metadata: self.metadata,
      uid: self.uid,
    };
  },
};
