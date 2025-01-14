import typing as t

from .common import ApiBase, ListOptions

from ..internal.openapi_client.api.background_tasks import (
    get_background_task,
    list_background_tasks,
)
from ..internal.openapi_client.models.background_task_out import BackgroundTaskOut
from ..internal.openapi_client.models.background_task_status import BackgroundTaskStatus
from ..internal.openapi_client.models.background_task_type import BackgroundTaskType
from ..internal.openapi_client.models.list_response_background_task_out import (
    ListResponseBackgroundTaskOut,
)


class BackgroundTaskListOptions(ListOptions):
    status: t.Optional[BackgroundTaskStatus] = None
    task: t.Optional[BackgroundTaskType] = None


class BackgroundTaskAsync(ApiBase):
    async def list(
        self, options: BackgroundTaskListOptions = BackgroundTaskListOptions()
    ) -> ListResponseBackgroundTaskOut:
        return await list_background_tasks.request_asyncio(
            client=self._client, **options.to_dict()
        )

    async def get(self, task_id: str) -> BackgroundTaskOut:
        return await get_background_task.request_asyncio(
            client=self._client, task_id=task_id
        )


class BackgroundTask(ApiBase):
    def list(
        self, options: BackgroundTaskListOptions = BackgroundTaskListOptions()
    ) -> ListResponseBackgroundTaskOut:
        return list_background_tasks.request_sync(
            client=self._client, **options.to_dict()
        )

    def get(self, task_id: str) -> BackgroundTaskOut:
        return get_background_task.request_sync(client=self._client, task_id=task_id)
