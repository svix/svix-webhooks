import typing as t
from dataclasses import dataclass

from .common import ApiBase, BaseOptions


from ..internal.openapi_client.api.statistics import (
    v1_statistics_aggregate_app_stats,
    v1_statistics_aggregate_event_types,
)

from ..internal.openapi_client.models.app_usage_stats_in import AppUsageStatsIn
from ..internal.openapi_client.models.app_usage_stats_out import AppUsageStatsOut
from ..internal.openapi_client.models.aggregate_event_types_out import (
    AggregateEventTypesOut,
)


@dataclass
class StatisticsAggregateAppStatsOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None


class StatisticsAsync(ApiBase):
    async def aggregate_app_stats(
        self,
        app_usage_stats_in: AppUsageStatsIn,
        options: StatisticsAggregateAppStatsOptions = StatisticsAggregateAppStatsOptions(),
    ) -> AppUsageStatsOut:
        """Creates a background task to calculate the message destinations for all applications in the environment.

        Note that this endpoint is asynchronous. You will need to poll the `Get Background Task` endpoint to
        retrieve the results of the operation."""
        return await v1_statistics_aggregate_app_stats.request_asyncio(
            client=self._client, json_body=app_usage_stats_in, **options.to_dict()
        )

    async def aggregate_event_types(self) -> AggregateEventTypesOut:
        """Creates a background task to calculate the listed event types for all apps in the organization.

        Note that this endpoint is asynchronous. You will need to poll the `Get Background Task` endpoint to
        retrieve the results of the operation."""
        return await v1_statistics_aggregate_event_types.request_asyncio(
            client=self._client
        )


class Statistics(ApiBase):
    def aggregate_app_stats(
        self,
        app_usage_stats_in: AppUsageStatsIn,
        options: StatisticsAggregateAppStatsOptions = StatisticsAggregateAppStatsOptions(),
    ) -> AppUsageStatsOut:
        """Creates a background task to calculate the message destinations for all applications in the environment.

        Note that this endpoint is asynchronous. You will need to poll the `Get Background Task` endpoint to
        retrieve the results of the operation."""
        return v1_statistics_aggregate_app_stats.request_sync(
            client=self._client, json_body=app_usage_stats_in, **options.to_dict()
        )

    def aggregate_event_types(self) -> AggregateEventTypesOut:
        """Creates a background task to calculate the listed event types for all apps in the organization.

        Note that this endpoint is asynchronous. You will need to poll the `Get Background Task` endpoint to
        retrieve the results of the operation."""
        return v1_statistics_aggregate_event_types.request_sync(client=self._client)
