// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import { IngestEndpointOut, IngestEndpointOutSerializer } from "./ingestEndpointOut";

export interface ListResponseIngestEndpointOut {
  data: IngestEndpointOut[];
  done: boolean;
  iterator: string | null;
  prevIterator?: string | null;
}

export const ListResponseIngestEndpointOutSerializer = {
  _fromJsonObject(object: any): ListResponseIngestEndpointOut {
    return {
      data: object["data"]?.map((item: IngestEndpointOut) =>
        IngestEndpointOutSerializer._fromJsonObject(item)
      ),
      done: object["done"],
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
    };
  },

  _toJsonObject(self: ListResponseIngestEndpointOut): any {
    return {
      data: self.data.map((item) => IngestEndpointOutSerializer._toJsonObject(item)),
      done: self.done,
      iterator: self.iterator,
      prevIterator: self.prevIterator,
    };
  },
};
