// this file is @generated
import {
  type MessageEndpointOut,
  MessageEndpointOutSerializer,
} from "./messageEndpointOut";

export interface ListResponseMessageEndpointOut {
  data: MessageEndpointOut[];
  iterator: string | null;
  prevIterator?: string | null;
  done: boolean;
}

export const ListResponseMessageEndpointOutSerializer = {
  _fromJsonObject(object: any): ListResponseMessageEndpointOut {
    return {
      data: object["data"].map((item: MessageEndpointOut) =>
        MessageEndpointOutSerializer._fromJsonObject(item)
      ),
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
      done: object["done"],
    };
  },

  _toJsonObject(self: ListResponseMessageEndpointOut): any {
    return {
      data: self.data.map((item) => MessageEndpointOutSerializer._toJsonObject(item)),
      iterator: self.iterator,
      prevIterator: self.prevIterator,
      done: self.done,
    };
  },
};
