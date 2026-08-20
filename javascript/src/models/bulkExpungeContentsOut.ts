// this file is @generated
import { type BulkExpungeStatus, BulkExpungeStatusSerializer } from "./bulkExpungeStatus";

export interface BulkExpungeContentsOut {
  /** Results of expunging (by ID) */
  results: { [key: string]: BulkExpungeStatus };
}

export const BulkExpungeContentsOutSerializer = {
  _fromJsonObject(object: any): BulkExpungeContentsOut {
    return {
      results: Object.fromEntries(
        Object.entries<BulkExpungeStatus>(object["results"]).map(
          (item: [string, BulkExpungeStatus]) => [
            item[0],
            BulkExpungeStatusSerializer._fromJsonObject(item[1]),
          ]
        )
      ),
    };
  },

  _toJsonObject(self: BulkExpungeContentsOut): any {
    return {
      results: Object.fromEntries(
        Object.entries(self.results).map((item) => [
          item[0],
          BulkExpungeStatusSerializer._toJsonObject(item[1]),
        ])
      ),
    };
  },
};
