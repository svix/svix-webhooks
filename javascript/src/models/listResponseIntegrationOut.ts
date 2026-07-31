// this file is @generated
import { type IntegrationOut, IntegrationOutSerializer } from "./integrationOut";

export interface ListResponseIntegrationOut {
  data: IntegrationOut[];
  iterator: string | null;
  prevIterator?: string | null;
  done: boolean;
}

export const ListResponseIntegrationOutSerializer = {
  _fromJsonObject(object: any): ListResponseIntegrationOut {
    return {
      data: object["data"].map((item: IntegrationOut) =>
        IntegrationOutSerializer._fromJsonObject(item)
      ),
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
      done: object["done"],
    };
  },

  _toJsonObject(self: ListResponseIntegrationOut): any {
    return {
      data: self.data.map((item) => IntegrationOutSerializer._toJsonObject(item)),
      iterator: self.iterator,
      prevIterator: self.prevIterator,
      done: self.done,
    };
  },
};
