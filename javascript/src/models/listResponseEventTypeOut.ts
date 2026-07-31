// this file is @generated
import { type EventTypeOut, EventTypeOutSerializer } from "./eventTypeOut";

export interface ListResponseEventTypeOut {
  data: EventTypeOut[];
  iterator: string | null;
  prevIterator?: string | null;
  done: boolean;
}

export const ListResponseEventTypeOutSerializer = {
  _fromJsonObject(object: any): ListResponseEventTypeOut {
    return {
      data: object["data"].map((item: EventTypeOut) =>
        EventTypeOutSerializer._fromJsonObject(item)
      ),
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
      done: object["done"],
    };
  },

  _toJsonObject(self: ListResponseEventTypeOut): any {
    return {
      data: self.data.map((item) => EventTypeOutSerializer._toJsonObject(item)),
      iterator: self.iterator,
      prevIterator: self.prevIterator,
      done: self.done,
    };
  },
};
