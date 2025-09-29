// this file is @generated
import {
  type MessageAttemptRecoveredEventData,
  MessageAttemptRecoveredEventDataSerializer,
} from "./messageAttemptRecoveredEventData";

/** Sent on a successful dispatch after an earlier failure op webhook has already been sent. */
export interface MessageAttemptRecoveredEvent {
  data: MessageAttemptRecoveredEventData;
  type: string;
}

export const MessageAttemptRecoveredEventSerializer = {
  _fromJsonObject(object: any): MessageAttemptRecoveredEvent {
    return {
      data: MessageAttemptRecoveredEventDataSerializer._fromJsonObject(object["data"]),
      type: object["type"],
    };
  },

  _toJsonObject(self: MessageAttemptRecoveredEvent): any {
    return {
      data: MessageAttemptRecoveredEventDataSerializer._toJsonObject(self.data),
      type: self.type,
    };
  },
};
