// this file is @generated
import { type IngestSourceOut, IngestSourceOutSerializer } from "./ingestSourceOut";

export interface ListResponseIngestSourceOut {
  data: IngestSourceOut[];
  done: boolean;
  iterator: string | null;
  prevIterator?: string | null;
}

export const ListResponseIngestSourceOutSerializer = {
  _fromJsonObject(object: any): ListResponseIngestSourceOut {
    return {
      data: object["data"].map((item: IngestSourceOut) =>
        IngestSourceOutSerializer._fromJsonObject(item)
      ),
      done: object["done"],
      iterator: object["iterator"],
      prevIterator: object["prevIterator"],
    };
  },

  _toJsonObject(self: ListResponseIngestSourceOut): any {
    return {
      data: self.data.map((item) => IngestSourceOutSerializer._toJsonObject(item)),
      done: self.done,
      iterator: self.iterator,
      prevIterator: self.prevIterator,
    };
  },
};
