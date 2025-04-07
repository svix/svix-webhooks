// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import { StreamOut, StreamOutSerializer } from "./streamOut";

export interface ListResponseStreamOut {
  data: StreamOut[];
  done: boolean;
  iterator: string | null;
  prevIterator?: string | null;
}

export const ListResponseStreamOutSerializer = {
  _fromJsonObject(object: any): ListResponseStreamOut {
    return {
      data: object["data"]?.map((item: StreamOut) =>
        StreamOutSerializer._fromJsonObject(item)
      ),
      done: object["done"],
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
    };
  },

  _toJsonObject(self: ListResponseStreamOut): any {
    return {
      data: self.data.map((item) => StreamOutSerializer._toJsonObject(item)),
      done: self.done,
      iterator: self.iterator,
      prevIterator: self.prevIterator,
    };
  },
};
