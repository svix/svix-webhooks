# this file is @generated
import typing as t

from .common import BaseModel


class SqsConfig(BaseModel):
    """Configuration for an SQS sink."""

    access_key_id: str

    endpoint_url: t.Optional[str] = None

    queue_url: str

    region: str

    secret_access_key: str
