// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import {
  EnvironmentModelOut,
  EnvironmentModelOutSerializer,
} from "./environmentModelOut";

export interface ListResponseEnvironmentModelOut {
  data: EnvironmentModelOut[];
  done: boolean;
  iterator: string | null;
  prevIterator?: string | null;
}

export const ListResponseEnvironmentModelOutSerializer = {
  _fromJsonObject(object: any): ListResponseEnvironmentModelOut {
    return {
      data: object["data"]?.map((item: EnvironmentModelOut) =>
        EnvironmentModelOutSerializer._fromJsonObject(item)
      ),
      done: object["done"],
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
    };
  },

  _toJsonObject(self: ListResponseEnvironmentModelOut): any {
    return {
      data: self.data.map((item) => EnvironmentModelOutSerializer._toJsonObject(item)),
      done: self.done,
      iterator: self.iterator,
      prevIterator: self.prevIterator,
    };
  },
};
