// this file is @generated
import { type StreamSinkOut, StreamSinkOutSerializer } from "./streamSinkOut";

export interface ListResponseStreamSinkOut {
  data: StreamSinkOut[];
  done: boolean;
  iterator: string | null;
  prevIterator?: string | null;
}

export const ListResponseStreamSinkOutSerializer = {
  _fromJsonObject(object: any): ListResponseStreamSinkOut {
    return {
      data: object["data"].map((item: StreamSinkOut) =>
        StreamSinkOutSerializer._fromJsonObject(item)
      ),
      done: object["done"],
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
    };
  },

  _toJsonObject(self: ListResponseStreamSinkOut): any {
    return {
      data: self.data.map((item) => StreamSinkOutSerializer._toJsonObject(item)),
      done: self.done,
      iterator: self.iterator,
      prevIterator: self.prevIterator,
    };
  },
};
