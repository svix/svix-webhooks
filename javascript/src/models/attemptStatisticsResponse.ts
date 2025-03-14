// this file is @generated
/* eslint @typescript-eslint/no-explicit-any: 0 */
import {
  AttemptStatisticsData,
  AttemptStatisticsDataSerializer,
} from "./attemptStatisticsData";
import { StatisticsPeriod, StatisticsPeriodSerializer } from "./statisticsPeriod";

export interface AttemptStatisticsResponse {
  data: AttemptStatisticsData;
  endDate: Date;
  period: StatisticsPeriod;
  startDate: Date;
}

export const AttemptStatisticsResponseSerializer = {
  _fromJsonObject(object: any): AttemptStatisticsResponse {
    return {
      data: AttemptStatisticsDataSerializer._fromJsonObject(object["data"]),
      endDate: new Date(object["endDate"]),
      period: StatisticsPeriodSerializer._fromJsonObject(object["period"]),
      startDate: new Date(object["startDate"]),
    };
  },

  _toJsonObject(self: AttemptStatisticsResponse): any {
    return {
      data: AttemptStatisticsDataSerializer._toJsonObject(self.data),
      endDate: self.endDate,
      period: StatisticsPeriodSerializer._toJsonObject(self.period),
      startDate: self.startDate,
    };
  },
};
