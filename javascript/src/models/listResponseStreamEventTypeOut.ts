// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import { StreamEventTypeOut, StreamEventTypeOutSerializer } from "./streamEventTypeOut";

export interface ListResponseStreamEventTypeOut {
  data: StreamEventTypeOut[];
  done: boolean;
  iterator: string | null;
  prevIterator?: string | null;
}

export const ListResponseStreamEventTypeOutSerializer = {
  _fromJsonObject(object: any): ListResponseStreamEventTypeOut {
    return {
      data: object["data"].map((item: StreamEventTypeOut) =>
        StreamEventTypeOutSerializer._fromJsonObject(item)
      ),
      done: object["done"],
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
    };
  },

  _toJsonObject(self: ListResponseStreamEventTypeOut): any {
    return {
      data: self.data.map((item) => StreamEventTypeOutSerializer._toJsonObject(item)),
      done: self.done,
      iterator: self.iterator,
      prevIterator: self.prevIterator,
    };
  },
};
