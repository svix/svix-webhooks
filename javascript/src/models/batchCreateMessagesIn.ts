// this file is @generated
import { type BatchMessageIn, BatchMessageInSerializer } from "./batchMessageIn";

export interface BatchCreateMessagesIn {
  messages: BatchMessageIn[];
}

export const BatchCreateMessagesInSerializer = {
  _fromJsonObject(object: any): BatchCreateMessagesIn {
    return {
      messages: object["messages"].map((item: BatchMessageIn) =>
        BatchMessageInSerializer._fromJsonObject(item)
      ),
    };
  },

  _toJsonObject(self: BatchCreateMessagesIn): any {
    return {
      messages: self.messages.map((item) => BatchMessageInSerializer._toJsonObject(item)),
    };
  },
};
