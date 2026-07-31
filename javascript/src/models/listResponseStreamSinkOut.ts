// this file is @generated
import { type StreamSinkOut, StreamSinkOutSerializer } from "./streamSinkOut";

export interface ListResponseStreamSinkOut {
  data: StreamSinkOut[];
  iterator: string | null;
  prevIterator?: string | null;
  done: boolean;
}

export const ListResponseStreamSinkOutSerializer = {
  _fromJsonObject(object: any): ListResponseStreamSinkOut {
    return {
      data: object["data"].map((item: StreamSinkOut) =>
        StreamSinkOutSerializer._fromJsonObject(item)
      ),
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
      done: object["done"],
    };
  },

  _toJsonObject(self: ListResponseStreamSinkOut): any {
    return {
      data: self.data.map((item) => StreamSinkOutSerializer._toJsonObject(item)),
      iterator: self.iterator,
      prevIterator: self.prevIterator,
      done: self.done,
    };
  },
};
