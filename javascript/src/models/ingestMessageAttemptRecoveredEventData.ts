// this file is @generated
import {
  type MessageAttemptFailedData,
  MessageAttemptFailedDataSerializer,
} from "./messageAttemptFailedData";

/** Sent when a message delivery has failed (all of the retry attempts have been exhausted) as a "ingest.message.attempt.exhausted" type, after it's failed four times as a "ingest.message.attempt.failing" event, or after it's recovered as a "ingest.message.attempt.recovered" event. */
export interface IngestMessageAttemptRecoveredEventData {
  /** The Endpoint's ID. */
  endpointId: string;
  lastAttempt: MessageAttemptFailedData;
  /** The Message's UID. */
  msgEventId?: string | null;
  /** The Message's ID. */
  msgId: string;
  /** The Source's ID. */
  sourceId: string;
}

export const IngestMessageAttemptRecoveredEventDataSerializer = {
  _fromJsonObject(object: any): IngestMessageAttemptRecoveredEventData {
    return {
      endpointId: object["endpointId"],
      lastAttempt: MessageAttemptFailedDataSerializer._fromJsonObject(
        object["lastAttempt"]
      ),
      msgEventId: object["msgEventId"],
      msgId: object["msgId"],
      sourceId: object["sourceId"],
    };
  },

  _toJsonObject(self: IngestMessageAttemptRecoveredEventData): any {
    return {
      endpointId: self.endpointId,
      lastAttempt: MessageAttemptFailedDataSerializer._toJsonObject(self.lastAttempt),
      msgEventId: self.msgEventId,
      msgId: self.msgId,
      sourceId: self.sourceId,
    };
  },
};
