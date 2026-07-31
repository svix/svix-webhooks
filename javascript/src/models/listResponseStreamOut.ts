// this file is @generated
import { type StreamOut, StreamOutSerializer } from "./streamOut";

export interface ListResponseStreamOut {
  data: StreamOut[];
  iterator: string | null;
  prevIterator?: string | null;
  done: boolean;
}

export const ListResponseStreamOutSerializer = {
  _fromJsonObject(object: any): ListResponseStreamOut {
    return {
      data: object["data"].map((item: StreamOut) =>
        StreamOutSerializer._fromJsonObject(item)
      ),
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
      done: object["done"],
    };
  },

  _toJsonObject(self: ListResponseStreamOut): any {
    return {
      data: self.data.map((item) => StreamOutSerializer._toJsonObject(item)),
      iterator: self.iterator,
      prevIterator: self.prevIterator,
      done: self.done,
    };
  },
};
