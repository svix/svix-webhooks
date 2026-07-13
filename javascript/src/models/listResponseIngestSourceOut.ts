// this file is @generated
import { type IngestSourceOut, IngestSourceOutSerializer } from "./ingestSourceOut";

export interface ListResponseIngestSourceOut {
  data: IngestSourceOut[];
  iterator: string | null;
  prevIterator?: string | null;
  done: boolean;
}

export const ListResponseIngestSourceOutSerializer = {
  _fromJsonObject(object: any): ListResponseIngestSourceOut {
    return {
      data: object["data"].map((item: IngestSourceOut) =>
        IngestSourceOutSerializer._fromJsonObject(item)
      ),
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
      done: object["done"],
    };
  },

  _toJsonObject(self: ListResponseIngestSourceOut): any {
    return {
      data: self.data.map((item) => IngestSourceOutSerializer._toJsonObject(item)),
      iterator: self.iterator,
      prevIterator: self.prevIterator,
      done: self.done,
    };
  },
};
