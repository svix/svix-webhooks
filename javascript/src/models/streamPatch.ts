// this file is @generated

export interface StreamPatch {
  /** The Stream's description. */
  description?: string;
  /** An optional unique identifier for the stream. */
  uid?: string | null;
  metadata?: { [key: string]: string };
}

export const StreamPatchSerializer = {
  _fromJsonObject(object: any): StreamPatch {
    return {
      description: object["description"],
      uid: object["uid"],
      metadata: object["metadata"],
    };
  },

  _toJsonObject(self: StreamPatch): any {
    return {
      description: self.description,
      uid: self.uid,
      metadata: self.metadata,
    };
  },
};
