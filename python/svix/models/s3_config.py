# this file is @generated
import typing as t

from .common import BaseModel


class S3Config(BaseModel):
    access_key_id: str

    bucket: str

    endpoint_url: t.Optional[str] = None

    region: str

    secret_access_key: str
