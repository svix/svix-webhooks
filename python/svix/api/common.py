import typing as t
from dataclasses import asdict, dataclass
from datetime import datetime, timezone


from ..internal.openapi_client.client import AuthenticatedClient


def ensure_tz(x: t.Optional[datetime]) -> t.Optional[datetime]:
    if x is None:
        return None

    if x.tzinfo is None:
        return x.replace(tzinfo=timezone.utc)
    return x


def sanitize_field(v: t.Any) -> t.Any:
    if isinstance(v, datetime):
        return ensure_tz(v)

    return v


@dataclass
class BaseOptions:
    def to_dict(self) -> t.Dict[str, t.Any]:
        return {k: sanitize_field(v) for k, v in asdict(self).items() if v is not None}


@dataclass
class ListOptions(BaseOptions):
    iterator: t.Optional[str] = None
    limit: t.Optional[int] = None


@dataclass
class PostOptions(BaseOptions):
    idempotency_key: t.Optional[str] = None


class ApiBase:
    _client: AuthenticatedClient

    def __init__(self, client: AuthenticatedClient) -> None:
        self._client = client
