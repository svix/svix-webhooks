// this file is @generated

export interface MessagePrecheckIn {
  /** The event type's name */
  eventType: string;
  channels?: string[] | null;
}

export const MessagePrecheckInSerializer = {
  _fromJsonObject(object: any): MessagePrecheckIn {
    return {
      eventType: object["eventType"],
      channels: object["channels"],
    };
  },

  _toJsonObject(self: MessagePrecheckIn): any {
    return {
      eventType: self.eventType,
      channels: self.channels,
    };
  },
};
