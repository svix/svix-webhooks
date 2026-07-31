// this file is @generated
import { type BackgroundTaskOut, BackgroundTaskOutSerializer } from "./backgroundTaskOut";

export interface ListResponseBackgroundTaskOut {
  data: BackgroundTaskOut[];
  iterator: string | null;
  prevIterator?: string | null;
  done: boolean;
}

export const ListResponseBackgroundTaskOutSerializer = {
  _fromJsonObject(object: any): ListResponseBackgroundTaskOut {
    return {
      data: object["data"].map((item: BackgroundTaskOut) =>
        BackgroundTaskOutSerializer._fromJsonObject(item)
      ),
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
      done: object["done"],
    };
  },

  _toJsonObject(self: ListResponseBackgroundTaskOut): any {
    return {
      data: self.data.map((item) => BackgroundTaskOutSerializer._toJsonObject(item)),
      iterator: self.iterator,
      prevIterator: self.prevIterator,
      done: self.done,
    };
  },
};
