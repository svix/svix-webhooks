# this file is @generated
import typing as t

from .common import BaseModel


class AzureBlobStorageConfigIn(BaseModel):
    container: str

    account: str

    access_key: t.Optional[str] = None
    """Access key.

    Currently a required field, but marked as optional because we may add different authentication in the future."""
