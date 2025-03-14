// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface StreamEventTypePatch {
  description: string | null;
  /** The event type's name */
  name?: string | null;
}

export const StreamEventTypePatchSerializer = {
  _fromJsonObject(object: any): StreamEventTypePatch {
    return {
      description: object["description"],
      name: object["name"],
    };
  },

  _toJsonObject(self: StreamEventTypePatch): any {
    return {
      description: self.description,
      name: self.name,
    };
  },
};
