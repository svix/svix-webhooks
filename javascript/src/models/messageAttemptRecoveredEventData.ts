// this file is @generated
import {
  type MessageAttemptFailedData,
  MessageAttemptFailedDataSerializer,
} from "./messageAttemptFailedData";

/** Sent when a message delivery has failed (all of the retry attempts have been exhausted) as a "message.attempt.exhausted" type or after it's failed four times as a "message.attempt.failing" event. */
export interface MessageAttemptRecoveredEventData {
  /** The Application's ID. */
  appId: string;
  /** The Application's UID. */
  appUid?: string | null;
  /** The Endpoint's ID. */
  endpointId: string;
  lastAttempt: MessageAttemptFailedData;
  /** The Message's UID. */
  msgEventId?: string | null;
  /** The Message's ID. */
  msgId: string;
}

export const MessageAttemptRecoveredEventDataSerializer = {
  _fromJsonObject(object: any): MessageAttemptRecoveredEventData {
    return {
      appId: object["appId"],
      appUid: object["appUid"],
      endpointId: object["endpointId"],
      lastAttempt: MessageAttemptFailedDataSerializer._fromJsonObject(
        object["lastAttempt"]
      ),
      msgEventId: object["msgEventId"],
      msgId: object["msgId"],
    };
  },

  _toJsonObject(self: MessageAttemptRecoveredEventData): any {
    return {
      appId: self.appId,
      appUid: self.appUid,
      endpointId: self.endpointId,
      lastAttempt: MessageAttemptFailedDataSerializer._toJsonObject(self.lastAttempt),
      msgEventId: self.msgEventId,
      msgId: self.msgId,
    };
  },
};
