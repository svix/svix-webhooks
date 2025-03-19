// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface MessagePrecheckIn {
  channels?: string[] | null;
  /** The event type's name */
  eventType: string;
}

export const MessagePrecheckInSerializer = {
  _fromJsonObject(object: any): MessagePrecheckIn {
    return {
      channels: object["channels"],
      eventType: object["eventType"],
    };
  },

  _toJsonObject(self: MessagePrecheckIn): any {
    return {
      channels: self.channels,
      eventType: self.eventType,
    };
  },
};
