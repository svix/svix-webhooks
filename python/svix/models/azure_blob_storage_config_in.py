# this file is @generated

from .common import BaseModel


class AzureBlobStorageConfigIn(BaseModel):
    container: str

    account: str

    access_key: str
