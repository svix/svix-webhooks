// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import {
  MessageAttemptExhaustedEventData,
  MessageAttemptExhaustedEventDataSerializer,
} from "./messageAttemptExhaustedEventData";

/** Sent when a message delivery has failed (all of the retry attempts have been exhausted). */
export interface MessageAttemptExhaustedEvent {
  data: MessageAttemptExhaustedEventData;
  type: string;
}

export const MessageAttemptExhaustedEventSerializer = {
  _fromJsonObject(object: any): MessageAttemptExhaustedEvent {
    return {
      data: MessageAttemptExhaustedEventDataSerializer._fromJsonObject(object["data"]),
      type: object["type"],
    };
  },

  _toJsonObject(self: MessageAttemptExhaustedEvent): any {
    return {
      data: MessageAttemptExhaustedEventDataSerializer._toJsonObject(self.data),
      type: self.type,
    };
  },
};
