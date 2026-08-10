# this file is @generated
import typing as t

from .common import BaseModel


class GoogleCloudStorageConfigPatch(BaseModel):
    bucket: t.Optional[str] = None

    credentials: t.Optional[str] = None
