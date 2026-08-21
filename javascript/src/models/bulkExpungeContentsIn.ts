// this file is @generated

export interface BulkExpungeContentsIn {
  /** Message ID or UID to delete */
  ids?: string[];
}

export const BulkExpungeContentsInSerializer = {
  _fromJsonObject(object: any): BulkExpungeContentsIn {
    return {
      ids: object["ids"],
    };
  },

  _toJsonObject(self: BulkExpungeContentsIn): any {
    return {
      ids: self.ids,
    };
  },
};
