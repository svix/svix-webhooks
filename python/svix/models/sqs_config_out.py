# this file is @generated
import typing as t

from .common import BaseModel


class SqsConfigOut(BaseModel):
    queue_url: str

    region: str

    access_key_id: t.Optional[str] = None

    role_arn: t.Optional[str] = None

    external_id: t.Optional[str] = None

    endpoint_url: t.Optional[str] = None
