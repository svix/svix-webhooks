# this file is @generated
import typing as t

from .common import BaseModel


class SqsConfig(BaseModel):
    """Configuration for an SQS sink."""

    queue_url: str

    region: str

    access_key_id: str

    secret_access_key: str

    endpoint_url: t.Optional[str] = None
