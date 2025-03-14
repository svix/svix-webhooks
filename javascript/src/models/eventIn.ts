// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface EventIn {
  /** The event type's name */
  eventType?: string | null;
  payload: string;
}

export const EventInSerializer = {
  _fromJsonObject(object: any): EventIn {
    return {
      eventType: object["eventType"],
      payload: object["payload"],
    };
  },

  _toJsonObject(self: EventIn): any {
    return {
      eventType: self.eventType,
      payload: self.payload,
    };
  },
};
