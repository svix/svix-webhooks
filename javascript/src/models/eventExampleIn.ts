// this file is @generated

export interface EventExampleIn {
  /** The event type's name */
  eventType: string;
  /**
   * If the event type schema contains an array of examples, chooses which one to send.
   *
   * Defaults to the first example. Ignored if the schema doesn't contain an array of examples.
   */
  exampleIndex?: number;
  /**
   * Custom payload to send as an example
   *
   * This is only available to allow-listed customers and should otherwise not be passed. Please
   * contact us if you need access to this functionality
   */
  payload?: any | null;
}

export const EventExampleInSerializer = {
  _fromJsonObject(object: any): EventExampleIn {
    return {
      eventType: object["eventType"],
      exampleIndex: object["exampleIndex"],
      payload: object["payload"],
    };
  },

  _toJsonObject(self: EventExampleIn): any {
    return {
      eventType: self.eventType,
      exampleIndex: self.exampleIndex,
      payload: self.payload,
    };
  },
};
