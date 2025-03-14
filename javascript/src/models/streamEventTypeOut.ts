// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface StreamEventTypeOut {
  createdAt: Date;
  description?: string | null;
  /** The event type's name */
  name: string;
  updatedAt: Date;
}

export const StreamEventTypeOutSerializer = {
  _fromJsonObject(object: any): StreamEventTypeOut {
    return {
      createdAt: new Date(object["createdAt"]),
      description: object["description"],
      name: object["name"],
      updatedAt: new Date(object["updatedAt"]),
    };
  },

  _toJsonObject(self: StreamEventTypeOut): any {
    return {
      createdAt: self.createdAt,
      description: self.description,
      name: self.name,
      updatedAt: self.updatedAt,
    };
  },
};
