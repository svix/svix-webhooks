// this file is @generated
import { type DestinationOut, DestinationOutSerializer } from "./destinationOut";

export interface ListResponseDestinationOut {
  data: DestinationOut[];
  iterator: string | null;
  prevIterator?: string | null;
  done: boolean;
}

export const ListResponseDestinationOutSerializer = {
  _fromJsonObject(object: any): ListResponseDestinationOut {
    return {
      data: object["data"].map((item: DestinationOut) =>
        DestinationOutSerializer._fromJsonObject(item)
      ),
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
      done: object["done"],
    };
  },

  _toJsonObject(self: ListResponseDestinationOut): any {
    return {
      data: self.data.map((item) => DestinationOutSerializer._toJsonObject(item)),
      iterator: self.iterator,
      prevIterator: self.prevIterator,
      done: self.done,
    };
  },
};
