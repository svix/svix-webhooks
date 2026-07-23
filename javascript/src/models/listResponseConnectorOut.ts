// this file is @generated
import { type ConnectorOut, ConnectorOutSerializer } from "./connectorOut";

export interface ListResponseConnectorOut {
  data: ConnectorOut[];
  iterator: string | null;
  prevIterator?: string | null;
  done: boolean;
}

export const ListResponseConnectorOutSerializer = {
  _fromJsonObject(object: any): ListResponseConnectorOut {
    return {
      data: object["data"].map((item: ConnectorOut) =>
        ConnectorOutSerializer._fromJsonObject(item)
      ),
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
      done: object["done"],
    };
  },

  _toJsonObject(self: ListResponseConnectorOut): any {
    return {
      data: self.data.map((item) => ConnectorOutSerializer._toJsonObject(item)),
      iterator: self.iterator,
      prevIterator: self.prevIterator,
      done: self.done,
    };
  },
};
