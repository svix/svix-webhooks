// this file is @generated

export enum BulkExpungeStatus {
  Expunged = "expunged",
  NotFound = "not-found",
}

export const BulkExpungeStatusSerializer = {
  _fromJsonObject(object: any): BulkExpungeStatus {
    return object;
  },

  _toJsonObject(self: BulkExpungeStatus): any {
    return self;
  },
};
