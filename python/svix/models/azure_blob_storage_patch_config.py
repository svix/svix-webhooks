# this file is @generated
import typing as t

from .common import BaseModel


class AzureBlobStoragePatchConfig(BaseModel):
    access_key: t.Optional[str] = None

    account: t.Optional[str] = None

    container: t.Optional[str] = None
