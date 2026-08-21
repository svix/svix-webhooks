# this file is @generated
import typing as t

from .common import BaseModel


class S3ConfigOut(BaseModel):
    bucket: str

    access_key_id: str

    region: str

    endpoint_url: t.Optional[str] = None
