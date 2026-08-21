# this file is @generated
import typing as t

from .common import BaseModel


class SqsConfigOut(BaseModel):
    queue_url: str

    region: str

    access_key_id: str

    endpoint_url: t.Optional[str] = None
