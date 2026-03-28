# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel

from .seek_position import SeekPosition


class MsgStreamReceiveIn(BaseModel):
    namespace: t.Optional[str] = None

    batch_size: t.Optional[int] = Field(default=None, alias="batch_size")

    lease_duration_ms: t.Optional[int] = Field(default=None, alias="lease_duration_ms")

    default_starting_position: t.Optional[SeekPosition] = Field(
        default=None, alias="default_starting_position"
    )

    batch_wait_ms: t.Optional[int] = Field(default=None, alias="batch_wait_ms")
    """Maximum time (in milliseconds) to wait for messages before returning."""


class _MsgStreamReceiveIn(BaseModel):
    namespace: t.Optional[str] = None

    topic: str

    consumer_group: str = Field(alias="consumer_group")

    batch_size: t.Optional[int] = Field(default=None, alias="batch_size")

    lease_duration_ms: t.Optional[int] = Field(default=None, alias="lease_duration_ms")

    default_starting_position: t.Optional[SeekPosition] = Field(
        default=None, alias="default_starting_position"
    )

    batch_wait_ms: t.Optional[int] = Field(default=None, alias="batch_wait_ms")
    """Maximum time (in milliseconds) to wait for messages before returning."""
