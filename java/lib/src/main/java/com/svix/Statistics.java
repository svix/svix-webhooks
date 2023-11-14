package com.svix;

import com.svix.exceptions.ApiException;
import com.svix.internal.api.StatisticsApi;
import com.svix.models.AggregateEventTypesOut;
import com.svix.models.AppUsageStatsIn;
import com.svix.models.AppUsageStatsOut;

public class Statistics {
    private final StatisticsApi api;

    Statistics() {
        api = new StatisticsApi();
    }

    public AppUsageStatsOut calculateAggregateAppStats(final AppUsageStatsIn appUsageStatsIn, final PostOptions options) throws ApiException {
        try {
            return api.calculateAggregateAppStats(appUsageStatsIn, options.getIdempotencyKey());
        } catch (com.svix.internal.ApiException e) {
            throw Utils.wrapInternalApiException(e);
        }
    }

    public AggregateEventTypesOut aggregateEventTypes() throws com.svix.internal.ApiException {
        try {
            return api.aggregateEventTypes();
        } catch (com.svix.internal.ApiException e) {
            throw Utils.wrapInternalApiException(e);
        }
    }

}
