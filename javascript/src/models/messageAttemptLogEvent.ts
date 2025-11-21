// this file is @generated
import { type MessageAttemptLog, MessageAttemptLogSerializer } from "./messageAttemptLog";

/** Sent after message attempts are made. Contains metadata about message attempts and their results. In order to reduce the frequency of webhooks, these are sent in batches periodically. */
export interface MessageAttemptLogEvent {
  data: MessageAttemptLog[];
  type: string;
}

export const MessageAttemptLogEventSerializer = {
  _fromJsonObject(object: any): MessageAttemptLogEvent {
    return {
      data: object["data"].map((item: MessageAttemptLog) =>
        MessageAttemptLogSerializer._fromJsonObject(item)
      ),
      type: object["type"],
    };
  },

  _toJsonObject(self: MessageAttemptLogEvent): any {
    return {
      data: self.data.map((item) => MessageAttemptLogSerializer._toJsonObject(item)),
      type: self.type,
    };
  },
};
