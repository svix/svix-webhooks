import {
    Configuration,
    StatisticsApi,
    AppUsageStatsIn,
    AppUsageStatsOut,
    AggregateEventTypesOut,
} from "../openapi";
import { PostOptions } from "../util";

export class Statistics {
    private readonly api: StatisticsApi;

    public constructor(config: Configuration) {
        this.api = new StatisticsApi(config);
    }

    public aggregateEventTypes(): Promise<AggregateEventTypesOut> {
        return this.api.v1StatisticsAggregateEventTypes({});
    }

    public aggregateAppStats(
        appUsageStatsIn: AppUsageStatsIn,
        options?: PostOptions
    ): Promise<AppUsageStatsOut> {
        return this.api.v1StatisticsAggregateAppStats({
            appUsageStatsIn,
            ...options,
        });
    }
}
