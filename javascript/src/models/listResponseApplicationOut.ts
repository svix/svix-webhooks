// this file is @generated
import { type ApplicationOut, ApplicationOutSerializer } from "./applicationOut";

export interface ListResponseApplicationOut {
  data: ApplicationOut[];
  iterator: string | null;
  prevIterator?: string | null;
  done: boolean;
}

export const ListResponseApplicationOutSerializer = {
  _fromJsonObject(object: any): ListResponseApplicationOut {
    return {
      data: object["data"].map((item: ApplicationOut) =>
        ApplicationOutSerializer._fromJsonObject(item)
      ),
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
      done: object["done"],
    };
  },

  _toJsonObject(self: ListResponseApplicationOut): any {
    return {
      data: self.data.map((item) => ApplicationOutSerializer._toJsonObject(item)),
      iterator: self.iterator,
      prevIterator: self.prevIterator,
      done: self.done,
    };
  },
};
