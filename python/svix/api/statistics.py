# This file is @generated
import typing as t
from dataclasses import dataclass

from ..models import AggregateEventTypesOut, AppUsageStatsIn, AppUsageStatsOut
from .common import ApiBase, BaseOptions, serialize_params


@dataclass
class StatisticsAggregateAppStatsOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None

    def _header_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "idempotency-key": self.idempotency_key,
            }
        )


class StatisticsAsync(ApiBase):
    async def aggregate_app_stats(
        self,
        app_usage_stats_in: AppUsageStatsIn,
        options: StatisticsAggregateAppStatsOptions = StatisticsAggregateAppStatsOptions(),
    ) -> AppUsageStatsOut:
        """Creates a background task to calculate the message destinations for all applications in the environment.

        Note that this endpoint is asynchronous. You will need to poll the `Get Background Task` endpoint to
        retrieve the results of the operation."""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/stats/usage/app",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=app_usage_stats_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return AppUsageStatsOut.model_validate(response.json())

    async def aggregate_event_types(self) -> AggregateEventTypesOut:
        """Creates a background task to calculate the listed event types for all apps in the organization.

        Note that this endpoint is asynchronous. You will need to poll the `Get Background Task` endpoint to
        retrieve the results of the operation."""
        response = await self._request_asyncio(
            method="put", path="/api/v1/stats/usage/event-types", path_params={}
        )
        return AggregateEventTypesOut.model_validate(response.json())


class Statistics(ApiBase):
    def aggregate_app_stats(
        self,
        app_usage_stats_in: AppUsageStatsIn,
        options: StatisticsAggregateAppStatsOptions = StatisticsAggregateAppStatsOptions(),
    ) -> AppUsageStatsOut:
        """Creates a background task to calculate the message destinations for all applications in the environment.

        Note that this endpoint is asynchronous. You will need to poll the `Get Background Task` endpoint to
        retrieve the results of the operation."""
        response = self._request_sync(
            method="post",
            path="/api/v1/stats/usage/app",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
            json_body=app_usage_stats_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return AppUsageStatsOut.model_validate(response.json())

    def aggregate_event_types(self) -> AggregateEventTypesOut:
        """Creates a background task to calculate the listed event types for all apps in the organization.

        Note that this endpoint is asynchronous. You will need to poll the `Get Background Task` endpoint to
        retrieve the results of the operation."""
        response = self._request_sync(
            method="put", path="/api/v1/stats/usage/event-types", path_params={}
        )
        return AggregateEventTypesOut.model_validate(response.json())
