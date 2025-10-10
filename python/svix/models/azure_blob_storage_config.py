# this file is @generated

from .common import BaseModel


class AzureBlobStorageConfig(BaseModel):
    access_key: str

    account: str

    container: str
