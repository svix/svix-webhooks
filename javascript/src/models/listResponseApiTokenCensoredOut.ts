// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import {
  ApiTokenCensoredOut,
  ApiTokenCensoredOutSerializer,
} from "./apiTokenCensoredOut";

export interface ListResponseApiTokenCensoredOut {
  data: ApiTokenCensoredOut[];
  done: boolean;
  iterator: string | null;
  prevIterator?: string | null;
}

export const ListResponseApiTokenCensoredOutSerializer = {
  _fromJsonObject(object: any): ListResponseApiTokenCensoredOut {
    return {
      data: object["data"]?.map((item: ApiTokenCensoredOut) =>
        ApiTokenCensoredOutSerializer._fromJsonObject(item)
      ),
      done: object["done"],
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
    };
  },

  _toJsonObject(self: ListResponseApiTokenCensoredOut): any {
    return {
      data: self.data.map((item) => ApiTokenCensoredOutSerializer._toJsonObject(item)),
      done: self.done,
      iterator: self.iterator,
      prevIterator: self.prevIterator,
    };
  },
};
