# This file is @generated
import typing as t
from dataclasses import dataclass

from ..internal.openapi_client import models
from ..internal.openapi_client.models.background_task_out import BackgroundTaskOut
from ..internal.openapi_client.models.list_response_background_task_out import (
    ListResponseBackgroundTaskOut,
)
from .common import ApiBase, BaseOptions, serialize_params


@dataclass
class BackgroundTaskListOptions(BaseOptions):
    # Filter the response based on the status.
    status: t.Optional[models.BackgroundTaskStatus] = None
    # Filter the response based on the type.
    task: t.Optional[models.BackgroundTaskType] = None
    # Limit the number of returned items
    limit: t.Optional[int] = None
    # The iterator returned from a prior invocation
    iterator: t.Optional[str] = None
    # The sorting order of the returned items
    order: t.Optional[models.Ordering] = None

    def _query_params(self) -> t.Dict[str, str]:
        return serialize_params(
            {
                "status": self.status,
                "task": self.task,
                "limit": self.limit,
                "iterator": self.iterator,
                "order": self.order,
            }
        )


class BackgroundTaskAsync(ApiBase):
    async def list(
        self, options: BackgroundTaskListOptions = BackgroundTaskListOptions()
    ) -> ListResponseBackgroundTaskOut:
        """List background tasks executed in the past 90 days."""
        # ruff: noqa: F841
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/background-task",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseBackgroundTaskOut.from_dict(response.json())

    async def get(self, task_id: str) -> BackgroundTaskOut:
        """Get a background task by ID."""
        # ruff: noqa: F841
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/background-task/{task_id}",
            path_params={
                "task_id": task_id,
            },
        )
        return BackgroundTaskOut.from_dict(response.json())


class BackgroundTask(ApiBase):
    def list(
        self, options: BackgroundTaskListOptions = BackgroundTaskListOptions()
    ) -> ListResponseBackgroundTaskOut:
        """List background tasks executed in the past 90 days."""
        # ruff: noqa: F841
        response = self._request_sync(
            method="get",
            path="/api/v1/background-task",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseBackgroundTaskOut.from_dict(response.json())

    def get(self, task_id: str) -> BackgroundTaskOut:
        """Get a background task by ID."""
        # ruff: noqa: F841
        response = self._request_sync(
            method="get",
            path="/api/v1/background-task/{task_id}",
            path_params={
                "task_id": task_id,
            },
        )
        return BackgroundTaskOut.from_dict(response.json())
