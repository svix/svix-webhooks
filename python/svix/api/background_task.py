# This file is @generated
import typing as t
from dataclasses import dataclass

from .. import models
from ..models import BackgroundTaskOut, ListResponseBackgroundTaskOut
from .common import ApiBase, BaseOptions, serialize_params


@dataclass
class BackgroundTaskListOptions(BaseOptions):
    status: t.Optional[models.BackgroundTaskStatus] = None
    """Filter the response based on the status."""
    task: t.Optional[models.BackgroundTaskType] = None
    """Filter the response based on the type."""
    limit: t.Optional[int] = None
    """Limit the number of returned items"""
    iterator: t.Optional[str] = None
    """The iterator returned from a prior invocation"""
    order: t.Optional[models.Ordering] = None
    """The sorting order of the returned items"""

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
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/background-task",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseBackgroundTaskOut.model_validate(response.json())

    async def get(self, task_id: str) -> BackgroundTaskOut:
        """Get a background task by ID."""
        response = await self._request_asyncio(
            method="get",
            path="/api/v1/background-task/{task_id}",
            path_params={
                "task_id": task_id,
            },
        )
        return BackgroundTaskOut.model_validate(response.json())


class BackgroundTask(ApiBase):
    def list(
        self, options: BackgroundTaskListOptions = BackgroundTaskListOptions()
    ) -> ListResponseBackgroundTaskOut:
        """List background tasks executed in the past 90 days."""
        response = self._request_sync(
            method="get",
            path="/api/v1/background-task",
            path_params={},
            query_params=options._query_params(),
            header_params=options._header_params(),
        )
        return ListResponseBackgroundTaskOut.model_validate(response.json())

    def get(self, task_id: str) -> BackgroundTaskOut:
        """Get a background task by ID."""
        response = self._request_sync(
            method="get",
            path="/api/v1/background-task/{task_id}",
            path_params={
                "task_id": task_id,
            },
        )
        return BackgroundTaskOut.model_validate(response.json())
