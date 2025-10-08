# this file is @generated
import typing as t

from .common import BaseModel


class AmazonS3PatchConfig(BaseModel):
    access_key_id: t.Optional[str] = None

    bucket: t.Optional[str] = None

    region: t.Optional[str] = None

    secret_access_key: t.Optional[str] = None
