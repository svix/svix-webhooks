// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface CountOut {
  /** There's a ceiling to how many attempts we count. When the limit is reached, this will be `true` to indicate the actual count is higher than given. */
  approximated: boolean;
  /** The count of attempts matching the query. */
  count: number;
}

export const CountOutSerializer = {
  _fromJsonObject(object: any): CountOut {
    return {
      approximated: object["approximated"],
      count: object["count"],
    };
  },

  _toJsonObject(self: CountOut): any {
    return {
      approximated: self.approximated,
      count: self.count,
    };
  },
};
