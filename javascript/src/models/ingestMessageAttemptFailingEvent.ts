// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import {
  IngestMessageAttemptFailingEventData,
  IngestMessageAttemptFailingEventDataSerializer,
} from "./ingestMessageAttemptFailingEventData";

/**
 * Sent after a message has been failing for a few times.
 * It's sent on the fourth failure. It complements `ingest.message.attempt.exhausted` which is sent after the last failure.
 */
export interface IngestMessageAttemptFailingEvent {
  data: IngestMessageAttemptFailingEventData;
  type: string;
}

export const IngestMessageAttemptFailingEventSerializer = {
  _fromJsonObject(object: any): IngestMessageAttemptFailingEvent {
    return {
      data: IngestMessageAttemptFailingEventDataSerializer._fromJsonObject(
        object["data"]
      ),
      type: object["type"],
    };
  },

  _toJsonObject(self: IngestMessageAttemptFailingEvent): any {
    return {
      data: IngestMessageAttemptFailingEventDataSerializer._toJsonObject(self.data),
      type: self.type,
    };
  },
};
