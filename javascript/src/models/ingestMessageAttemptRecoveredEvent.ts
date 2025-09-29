// this file is @generated
import {
  type IngestMessageAttemptRecoveredEventData,
  IngestMessageAttemptRecoveredEventDataSerializer,
} from "./ingestMessageAttemptRecoveredEventData";

/** Sent on a successful dispatch after an earlier failure op webhook has already been sent. */
export interface IngestMessageAttemptRecoveredEvent {
  data: IngestMessageAttemptRecoveredEventData;
  type: string;
}

export const IngestMessageAttemptRecoveredEventSerializer = {
  _fromJsonObject(object: any): IngestMessageAttemptRecoveredEvent {
    return {
      data: IngestMessageAttemptRecoveredEventDataSerializer._fromJsonObject(
        object["data"]
      ),
      type: object["type"],
    };
  },

  _toJsonObject(self: IngestMessageAttemptRecoveredEvent): any {
    return {
      data: IngestMessageAttemptRecoveredEventDataSerializer._toJsonObject(self.data),
      type: self.type,
    };
  },
};
