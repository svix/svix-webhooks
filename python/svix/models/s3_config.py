# this file is @generated
import typing as t

from .common import BaseModel


class S3Config(BaseModel):
    bucket: str

    access_key_id: str

    secret_access_key: str

    region: str

    endpoint_url: t.Optional[str] = None
