import typing as t
from dataclasses import asdict, dataclass


from svix.internal.openapi_client.models.aggregate_event_types_out import (
    AggregateEventTypesOut,
)
from svix.internal.openapi_client.models.app_usage_stats_in import AppUsageStatsIn
from svix.internal.openapi_client.models.app_usage_stats_out import AppUsageStatsOut

from ..internal.openapi_client.api.statistics import (
    v1_statistics_aggregate_app_stats,
    v1_statistics_aggregate_event_types,
)
from ..internal.openapi_client.client import AuthenticatedClient

DEFAULT_SERVER_URL = "https://api.svix.com"


@dataclass
class PostOptions:
    idempotency_key: t.Optional[str] = None

    def to_dict(self) -> t.Dict[str, t.Any]:
        return {k: v for k, v in asdict(self).items() if v is not None}


class ApiBase:
    _client: AuthenticatedClient

    def __init__(self, client: AuthenticatedClient) -> None:
        self._client = client


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
