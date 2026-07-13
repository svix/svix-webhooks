// this file is @generated
import { type IngestEndpointOut, IngestEndpointOutSerializer } from "./ingestEndpointOut";

export interface ListResponseIngestEndpointOut {
  data: IngestEndpointOut[];
  iterator: string | null;
  prevIterator?: string | null;
  done: boolean;
}

export const ListResponseIngestEndpointOutSerializer = {
  _fromJsonObject(object: any): ListResponseIngestEndpointOut {
    return {
      data: object["data"].map((item: IngestEndpointOut) =>
        IngestEndpointOutSerializer._fromJsonObject(item)
      ),
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
      done: object["done"],
    };
  },

  _toJsonObject(self: ListResponseIngestEndpointOut): any {
    return {
      data: self.data.map((item) => IngestEndpointOutSerializer._toJsonObject(item)),
      iterator: self.iterator,
      prevIterator: self.prevIterator,
      done: self.done,
    };
  },
};
