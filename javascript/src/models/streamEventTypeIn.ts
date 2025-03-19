// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface StreamEventTypeIn {
  description?: string | null;
  /** The event type's name */
  name: string;
}

export const StreamEventTypeInSerializer = {
  _fromJsonObject(object: any): StreamEventTypeIn {
    return {
      description: object["description"],
      name: object["name"],
    };
  },

  _toJsonObject(self: StreamEventTypeIn): any {
    return {
      description: self.description,
      name: self.name,
    };
  },
};
