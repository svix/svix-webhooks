// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */

export interface AttemptStatisticsData {
  failureCount?: number[] | null;
  successCount?: number[] | null;
}

export const AttemptStatisticsDataSerializer = {
  _fromJsonObject(object: any): AttemptStatisticsData {
    return {
      failureCount: object["failureCount"],
      successCount: object["successCount"],
    };
  },

  _toJsonObject(self: AttemptStatisticsData): any {
    return {
      failureCount: self.failureCount,
      successCount: self.successCount,
    };
  },
};
