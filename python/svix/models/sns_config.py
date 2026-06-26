# this file is @generated
import typing as t

from .common import BaseModel


class SnsConfig(BaseModel):
    """Configuration for a SNS sink."""

    access_key_id: str

    endpoint_url: t.Optional[str] = None

    region: str

    secret_access_key: str

    topic_arn: str
