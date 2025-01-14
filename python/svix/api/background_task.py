import typing as t
from dataclasses import asdict, dataclass
from datetime import datetime, timezone


from ..internal.openapi_client.api.background_tasks import (
    get_background_task,
    list_background_tasks,
)
from ..internal.openapi_client.client import AuthenticatedClient
from ..internal.openapi_client.models.background_task_out import BackgroundTaskOut
from ..internal.openapi_client.models.background_task_status import BackgroundTaskStatus
from ..internal.openapi_client.models.background_task_type import BackgroundTaskType
from ..internal.openapi_client.models.list_response_background_task_out import (
    ListResponseBackgroundTaskOut,
)

DEFAULT_SERVER_URL = "https://api.svix.com"


def ensure_tz(x: t.Optional[datetime]) -> t.Optional[datetime]:
    if x is None:
        return None

    if x.tzinfo is None:
        return x.replace(tzinfo=timezone.utc)
    return x


@dataclass
class ListOptions:
    iterator: t.Optional[str] = None
    limit: t.Optional[int] = None

    def to_dict(self) -> t.Dict[str, t.Any]:
        return {k: v for k, v in asdict(self).items() if v is not None}


class BackgroundTaskListOptions(ListOptions):
    status: t.Optional[BackgroundTaskStatus] = None
    task: t.Optional[BackgroundTaskType] = None


class ApiBase:
    _client: AuthenticatedClient

    def __init__(self, client: AuthenticatedClient) -> None:
        self._client = client


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
