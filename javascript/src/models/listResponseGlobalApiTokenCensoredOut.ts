// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import {
  GlobalApiTokenCensoredOut,
  GlobalApiTokenCensoredOutSerializer,
} from "./globalApiTokenCensoredOut";

export interface ListResponseGlobalApiTokenCensoredOut {
  data: GlobalApiTokenCensoredOut[];
  done: boolean;
  iterator: string | null;
  prevIterator?: string | null;
}

export const ListResponseGlobalApiTokenCensoredOutSerializer = {
  _fromJsonObject(object: any): ListResponseGlobalApiTokenCensoredOut {
    return {
      data: object["data"]?.map((item: GlobalApiTokenCensoredOut) =>
        GlobalApiTokenCensoredOutSerializer._fromJsonObject(item)
      ),
      done: object["done"],
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
    };
  },

  _toJsonObject(self: ListResponseGlobalApiTokenCensoredOut): any {
    return {
      data: self.data.map((item) =>
        GlobalApiTokenCensoredOutSerializer._toJsonObject(item)
      ),
      done: self.done,
      iterator: self.iterator,
      prevIterator: self.prevIterator,
    };
  },
};
