# this file is @generated
import typing as t

from .common import BaseModel


class SnsPatchConfig(BaseModel):
    access_key_id: t.Optional[str] = None

    endpoint_url: t.Optional[str] = None

    region: t.Optional[str] = None

    secret_access_key: t.Optional[str] = None

    topic_arn: t.Optional[str] = None
