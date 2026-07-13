# this file is @generated
import typing as t

from .common import BaseModel


class SnsConfig(BaseModel):
    """Configuration for a SNS sink."""

    topic_arn: str

    region: str

    access_key_id: str

    secret_access_key: str

    endpoint_url: t.Optional[str] = None
