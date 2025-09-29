// this file is @generated
import {
  type MessageAttemptFailingEventData,
  MessageAttemptFailingEventDataSerializer,
} from "./messageAttemptFailingEventData";

/**
 * Sent after a message has been failing for a few times.
 * It's sent on the fourth failure. It complements `message.attempt.exhausted` which is sent after the last failure.
 */
export interface MessageAttemptFailingEvent {
  data: MessageAttemptFailingEventData;
  type: string;
}

export const MessageAttemptFailingEventSerializer = {
  _fromJsonObject(object: any): MessageAttemptFailingEvent {
    return {
      data: MessageAttemptFailingEventDataSerializer._fromJsonObject(object["data"]),
      type: object["type"],
    };
  },

  _toJsonObject(self: MessageAttemptFailingEvent): any {
    return {
      data: MessageAttemptFailingEventDataSerializer._toJsonObject(self.data),
      type: self.type,
    };
  },
};
