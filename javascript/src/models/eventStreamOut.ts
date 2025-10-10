// this file is @generated
import { type EventOut, EventOutSerializer } from "./eventOut";

export interface EventStreamOut {
  data: EventOut[];
  done: boolean;
  iterator: string;
}

export const EventStreamOutSerializer = {
  _fromJsonObject(object: any): EventStreamOut {
    return {
      data: object["data"].map((item: EventOut) =>
        EventOutSerializer._fromJsonObject(item)
      ),
      done: object["done"],
      iterator: object["iterator"],
    };
  },

  _toJsonObject(self: EventStreamOut): any {
    return {
      data: self.data.map((item) => EventOutSerializer._toJsonObject(item)),
      done: self.done,
      iterator: self.iterator,
    };
  },
};
