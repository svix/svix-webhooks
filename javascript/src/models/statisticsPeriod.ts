// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
/** Period length for a statistics data point. */
export enum StatisticsPeriod {
  OneDay = "OneDay",
  FiveMinutes = "FiveMinutes",
}

export const StatisticsPeriodSerializer = {
  _fromJsonObject(object: any): StatisticsPeriod {
    return object;
  },

  _toJsonObject(self: StatisticsPeriod): any {
    return self;
  },
};
