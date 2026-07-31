// this file is @generated
import {
  type StreamEventTypeOut,
  StreamEventTypeOutSerializer,
} from "./streamEventTypeOut";

export interface ListResponseStreamEventTypeOut {
  data: StreamEventTypeOut[];
  iterator: string | null;
  prevIterator?: string | null;
  done: boolean;
}

export const ListResponseStreamEventTypeOutSerializer = {
  _fromJsonObject(object: any): ListResponseStreamEventTypeOut {
    return {
      data: object["data"].map((item: StreamEventTypeOut) =>
        StreamEventTypeOutSerializer._fromJsonObject(item)
      ),
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
      done: object["done"],
    };
  },

  _toJsonObject(self: ListResponseStreamEventTypeOut): any {
    return {
      data: self.data.map((item) => StreamEventTypeOutSerializer._toJsonObject(item)),
      iterator: self.iterator,
      prevIterator: self.prevIterator,
      done: self.done,
    };
  },
};
