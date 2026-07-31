// this file is @generated
import { type MessageOut, MessageOutSerializer } from "./messageOut";

export interface ListResponseMessageOut {
  data: MessageOut[];
  iterator: string | null;
  prevIterator?: string | null;
  done: boolean;
}

export const ListResponseMessageOutSerializer = {
  _fromJsonObject(object: any): ListResponseMessageOut {
    return {
      data: object["data"].map((item: MessageOut) =>
        MessageOutSerializer._fromJsonObject(item)
      ),
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
      done: object["done"],
    };
  },

  _toJsonObject(self: ListResponseMessageOut): any {
    return {
      data: self.data.map((item) => MessageOutSerializer._toJsonObject(item)),
      iterator: self.iterator,
      prevIterator: self.prevIterator,
      done: self.done,
    };
  },
};
