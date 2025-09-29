// this file is @generated
import {
  type IngestMessageAttemptExhaustedEventData,
  IngestMessageAttemptExhaustedEventDataSerializer,
} from "./ingestMessageAttemptExhaustedEventData";

/** Sent when a message delivery has failed (all of the retry attempts have been exhausted). */
export interface IngestMessageAttemptExhaustedEvent {
  data: IngestMessageAttemptExhaustedEventData;
  type: string;
}

export const IngestMessageAttemptExhaustedEventSerializer = {
  _fromJsonObject(object: any): IngestMessageAttemptExhaustedEvent {
    return {
      data: IngestMessageAttemptExhaustedEventDataSerializer._fromJsonObject(
        object["data"]
      ),
      type: object["type"],
    };
  },

  _toJsonObject(self: IngestMessageAttemptExhaustedEvent): any {
    return {
      data: IngestMessageAttemptExhaustedEventDataSerializer._toJsonObject(self.data),
      type: self.type,
    };
  },
};
