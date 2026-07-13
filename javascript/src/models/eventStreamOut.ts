// this file is @generated
import { type EventOut, EventOutSerializer } from "./eventOut";

export interface EventStreamOut {
  data: EventOut[];
  iterator: string;
  done: boolean;
}

export const EventStreamOutSerializer = {
  _fromJsonObject(object: any): EventStreamOut {
    return {
      data: object["data"].map((item: EventOut) =>
        EventOutSerializer._fromJsonObject(item)
      ),
      iterator: object["iterator"],
      done: object["done"],
    };
  },

  _toJsonObject(self: EventStreamOut): any {
    return {
      data: self.data.map((item) => EventOutSerializer._toJsonObject(item)),
      iterator: self.iterator,
      done: self.done,
    };
  },
};
