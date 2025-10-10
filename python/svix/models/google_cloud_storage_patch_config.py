# this file is @generated
import typing as t

from .common import BaseModel


class GoogleCloudStoragePatchConfig(BaseModel):
    bucket: t.Optional[str] = None

    credentials: t.Optional[str] = None
