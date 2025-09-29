// this file is @generated
/** Defines the ordering in a listing of results. */
export enum Ordering {
  Ascending = "ascending",
  Descending = "descending",
}

export const OrderingSerializer = {
  _fromJsonObject(object: any): Ordering {
    return object;
  },

  _toJsonObject(self: Ordering): any {
    return self;
  },
};
