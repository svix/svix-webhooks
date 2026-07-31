# this file is @generated

from .common import BaseModel


class AzureBlobStorageConfig(BaseModel):
    container: str

    account: str

    access_key: str
