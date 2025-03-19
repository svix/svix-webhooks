// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface EventOut {
  /** The event type's name */
  eventType?: string | null;
  payload: string;
  timestamp: Date;
}

export const EventOutSerializer = {
  _fromJsonObject(object: any): EventOut {
    return {
      eventType: object["eventType"],
      payload: object["payload"],
      timestamp: new Date(object["timestamp"]),
    };
  },

  _toJsonObject(self: EventOut): any {
    return {
      eventType: self.eventType,
      payload: self.payload,
      timestamp: self.timestamp,
    };
  },
};
