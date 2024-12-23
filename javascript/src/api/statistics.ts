// this file is @generated
import {
  Configuration,
  StatisticsApi,
  AggregateEventTypesOut,
  AppUsageStatsIn,
  AppUsageStatsOut,
} from "../openapi";
import { PostOptions } from "../util";

export class Statistics {
  private readonly api: StatisticsApi;

  public constructor(config: Configuration) {
    this.api = new StatisticsApi(config);
  }

  /**
   * Creates a background task to calculate the message destinations for all applications in the environment.
   *
   * Note that this endpoint is asynchronous. You will need to poll the `Get Background Task` endpoint to
   * retrieve the results of the operation.
   */
  public aggregateAppStats(
    appUsageStatsIn: AppUsageStatsIn,
    options?: PostOptions
  ): Promise<AppUsageStatsOut> {
    return this.api.v1StatisticsAggregateAppStats({
      appUsageStatsIn,
      ...options,
    });
  }

  /**
   * Creates a background task to calculate the listed event types for all apps in the organization.
   *
   * Note that this endpoint is asynchronous. You will need to poll the `Get Background Task` endpoint to
   * retrieve the results of the operation.
   */
  public aggregateEventTypes(): Promise<AggregateEventTypesOut> {
    return this.api.v1StatisticsAggregateEventTypes({});
  }
}
