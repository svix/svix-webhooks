// this file is @generated
import { type MessageAttemptOut, MessageAttemptOutSerializer } from "./messageAttemptOut";

export interface ListResponseMessageAttemptOut {
  data: MessageAttemptOut[];
  iterator: string | null;
  prevIterator?: string | null;
  done: boolean;
}

export const ListResponseMessageAttemptOutSerializer = {
  _fromJsonObject(object: any): ListResponseMessageAttemptOut {
    return {
      data: object["data"].map((item: MessageAttemptOut) =>
        MessageAttemptOutSerializer._fromJsonObject(item)
      ),
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
      done: object["done"],
    };
  },

  _toJsonObject(self: ListResponseMessageAttemptOut): any {
    return {
      data: self.data.map((item) => MessageAttemptOutSerializer._toJsonObject(item)),
      iterator: self.iterator,
      prevIterator: self.prevIterator,
      done: self.done,
    };
  },
};
