// this file is @generated
import { type EndpointOut, EndpointOutSerializer } from "./endpointOut";

export interface ListResponseEndpointOut {
  data: EndpointOut[];
  iterator: string | null;
  prevIterator?: string | null;
  done: boolean;
}

export const ListResponseEndpointOutSerializer = {
  _fromJsonObject(object: any): ListResponseEndpointOut {
    return {
      data: object["data"].map((item: EndpointOut) =>
        EndpointOutSerializer._fromJsonObject(item)
      ),
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
      done: object["done"],
    };
  },

  _toJsonObject(self: ListResponseEndpointOut): any {
    return {
      data: self.data.map((item) => EndpointOutSerializer._toJsonObject(item)),
      iterator: self.iterator,
      prevIterator: self.prevIterator,
      done: self.done,
    };
  },
};
