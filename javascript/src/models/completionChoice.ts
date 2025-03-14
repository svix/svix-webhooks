// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import { CompletionMessage, CompletionMessageSerializer } from "./completionMessage";

export interface CompletionChoice {
  finishReason: string;
  index: number;
  message: CompletionMessage;
}

export const CompletionChoiceSerializer = {
  _fromJsonObject(object: any): CompletionChoice {
    return {
      finishReason: object["finish_reason"],
      index: object["index"],
      message: CompletionMessageSerializer._fromJsonObject(object["message"]),
    };
  },

  _toJsonObject(self: CompletionChoice): any {
    return {
      finish_reason: self.finishReason,
      index: self.index,
      message: CompletionMessageSerializer._toJsonObject(self.message),
    };
  },
};
