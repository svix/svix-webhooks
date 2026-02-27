# this file is @generated
from pydantic import Field

from ..internal.base_model import BaseModel


class FetchFromStreamIn(BaseModel):
    batch_size: int = Field(alias="batch_size")
    """How many messages to read from the stream."""

    consumer_group: str = Field(alias="consumer_group")

    name: str

    visibility_timeout_seconds: int = Field(alias="visibility_timeout_seconds")
    """How long messages are locked by the consumer."""
