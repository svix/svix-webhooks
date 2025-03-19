// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface StreamPatch {
  /** The Stream's description. */
  description?: string;
  /** An optional unique identifier for the stream. */
  uid?: string | null;
}

export const StreamPatchSerializer = {
  _fromJsonObject(object: any): StreamPatch {
    return {
      description: object["description"],
      uid: object["uid"],
    };
  },

  _toJsonObject(self: StreamPatch): any {
    return {
      description: self.description,
      uid: self.uid,
    };
  },
};
