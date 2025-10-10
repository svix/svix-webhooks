// this file is @generated

export interface EventIn {
  /** The event type's name */
  eventType: string;
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
