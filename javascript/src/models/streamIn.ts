// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface StreamIn {
  /** The stream's description. */
  description: string;
  /** An optional unique identifier for the stream. */
  uid?: string | null;
}

export const StreamInSerializer = {
  _fromJsonObject(object: any): StreamIn {
    return {
      description: object["description"],
      uid: object["uid"],
    };
  },

  _toJsonObject(self: StreamIn): any {
    return {
      description: self.description,
      uid: self.uid,
    };
  },
};
