// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface EventTypeExampleOut {
  example: any;
}

export const EventTypeExampleOutSerializer = {
  _fromJsonObject(object: any): EventTypeExampleOut {
    return {
      example: object["example"],
    };
  },

  _toJsonObject(self: EventTypeExampleOut): any {
    return {
      example: self.example,
    };
  },
};
