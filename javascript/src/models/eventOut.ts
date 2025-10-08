// this file is @generated

export interface EventOut {
  /** The event type's name */
  eventType: string;
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
