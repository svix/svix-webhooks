// this file is @generated
import {
  type EndpointMessageOut,
  EndpointMessageOutSerializer,
} from "./endpointMessageOut";

export interface ListResponseEndpointMessageOut {
  data: EndpointMessageOut[];
  iterator: string | null;
  prevIterator?: string | null;
  done: boolean;
}

export const ListResponseEndpointMessageOutSerializer = {
  _fromJsonObject(object: any): ListResponseEndpointMessageOut {
    return {
      data: object["data"].map((item: EndpointMessageOut) =>
        EndpointMessageOutSerializer._fromJsonObject(item)
      ),
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
      done: object["done"],
    };
  },

  _toJsonObject(self: ListResponseEndpointMessageOut): any {
    return {
      data: self.data.map((item) => EndpointMessageOutSerializer._toJsonObject(item)),
      iterator: self.iterator,
      prevIterator: self.prevIterator,
      done: self.done,
    };
  },
};
