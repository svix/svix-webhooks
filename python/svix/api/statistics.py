from ..internal.openapi_client.models.aggregate_event_types_out import (
    AggregateEventTypesOut,
)
from ..internal.openapi_client.models.app_usage_stats_in import AppUsageStatsIn
from ..internal.openapi_client.models.app_usage_stats_out import AppUsageStatsOut

from .common import PostOptions, ApiBase


from ..internal.openapi_client.api.statistics import (
    v1_statistics_aggregate_app_stats,
    v1_statistics_aggregate_event_types,
)


class StatisticsAsync(ApiBase):
    async def aggregate_app_stats(
        self, app_usage_stats_in: AppUsageStatsIn, options: PostOptions = PostOptions()
    ) -> AppUsageStatsOut:
        return await v1_statistics_aggregate_app_stats.request_asyncio(
            client=self._client,
            json_body=app_usage_stats_in,
            **options.to_dict(),
        )

    async def aggregate_event_types(self, task_id: str) -> AggregateEventTypesOut:
        return await v1_statistics_aggregate_event_types.request_asyncio(
            client=self._client
        )


class Statistics(ApiBase):
    def aggregate_app_stats(
        self, app_usage_stats_in: AppUsageStatsIn, options: PostOptions = PostOptions()
    ) -> AppUsageStatsOut:
        return v1_statistics_aggregate_app_stats.request_sync(
            client=self._client,
            json_body=app_usage_stats_in,
            **options.to_dict(),
        )

    def aggregate_event_types(self, task_id: str) -> AggregateEventTypesOut:
        return v1_statistics_aggregate_event_types.request_sync(client=self._client)
