# this file is @generated
import typing as t

from .common import BaseModel


class AzureBlobStorageConfigPatch(BaseModel):
    container: t.Optional[str] = None

    account: t.Optional[str] = None

    access_key: t.Optional[str] = None
