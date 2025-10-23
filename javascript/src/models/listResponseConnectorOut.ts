// this file is @generated
import { type ConnectorOut, ConnectorOutSerializer } from "./connectorOut";

export interface ListResponseConnectorOut {
  data: ConnectorOut[];
  done: boolean;
  iterator: string | null;
  prevIterator?: string | null;
}

export const ListResponseConnectorOutSerializer = {
  _fromJsonObject(object: any): ListResponseConnectorOut {
    return {
      data: object["data"].map((item: ConnectorOut) =>
        ConnectorOutSerializer._fromJsonObject(item)
      ),
      done: object["done"],
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
    };
  },

  _toJsonObject(self: ListResponseConnectorOut): any {
    return {
      data: self.data.map((item) => ConnectorOutSerializer._toJsonObject(item)),
      done: self.done,
      iterator: self.iterator,
      prevIterator: self.prevIterator,
    };
  },
};
