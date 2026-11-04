# this file is @generated
import typing as t

from .common import BaseModel


class S3ConfigOut(BaseModel):
    bucket: str

    access_key_id: t.Optional[str] = None

    region: str

    endpoint_url: t.Optional[str] = None

    role_arn: t.Optional[str] = None

    external_id: t.Optional[str] = None
